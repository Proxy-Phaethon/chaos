//! Constructs a `ProjectManifest` from a completed `SemanticState`.
//!
//! `ManifestBuilder` performs semantic mapping only: it reads answers out
//! of a `SemanticState` and assembles them into the typed manifest structs
//! defined throughout `crate::manifest`. It does not prompt, normalize,
//! validate, resolve dependencies, or generate anything — those are the
//! responsibilities of other modules, and are expected to have already run
//! by the time `build` is called.

use crate::engine::{Field, SemanticState, Value};

use super::backend::{ApiStyle, Authentication, BackendFramework, BackendLanguage, BackendManifest};
use super::database::{DatabaseEngine, DatabaseManifest, Orm};
use super::frontend::{FrontendFramework, FrontendLanguage, FrontendManifest};
use super::project::{ProjectManifest, ProjectMetadata, ProjectState};
use super::tooling::{Docker, Git, Testing, ToolingManifest};

/// Describes why a `ProjectManifest` could not be constructed from a
/// `SemanticState`.
///
/// These are structural mapping failures only — they say nothing about
/// whether the *answers themselves* were reasonable (that is the
/// responsibility of `engine::validator`), only whether the state, as
/// given, can be mapped onto the manifest's typed structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManifestBuildError {
    /// A field required for the manifest to be constructed was never
    /// answered.
    MissingField(Field),

    /// A field was answered, but with a `Value` variant the mapping did
    /// not expect (e.g. a `Value::Bool` where text was required).
    InvalidValueType {
        field: Field,
        expected: &'static str,
    },

    /// A field held text that does not correspond to any known semantic
    /// variant for that field.
    UnknownVariant { field: Field, value: String },

    /// The semantic state describes a combination that cannot exist
    /// together (e.g. a database enabled while the backend is disabled).
    ImpossibleState(&'static str),

    /// The manifest requires information that `SemanticState`/`Field`
    /// cannot yet represent at all. This is a modeling gap, not a missing
    /// answer — see the TODOs on the relevant `build_*` function for what
    /// is still needed before this can be resolved.
    UnmodeledField(&'static str),
}

impl std::fmt::Display for ManifestBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ManifestBuildError::MissingField(field) => {
                write!(f, "missing required field: {:?}", field)
            }
            ManifestBuildError::InvalidValueType { field, expected } => {
                write!(f, "field {:?} expected a {} value", field, expected)
            }
            ManifestBuildError::UnknownVariant { field, value } => {
                write!(f, "field {:?} has an unrecognized value: '{}'", field, value)
            }
            ManifestBuildError::ImpossibleState(message) => {
                write!(f, "impossible semantic state: {}", message)
            }
            ManifestBuildError::UnmodeledField(name) => {
                write!(f, "'{}' cannot be built yet: not represented by the current Field model", name)
            }
        }
    }
}

impl std::error::Error for ManifestBuildError {}

/// Constructs a `ProjectManifest` from a `SemanticState`.
///
/// `ManifestBuilder` holds no state of its own — it is a namespace for the
/// mapping logic in `build`. It never modifies the `SemanticState` it is
/// given.
pub struct ManifestBuilder;

impl ManifestBuilder {
    /// Builds a `ProjectManifest` from `state`.
    ///
    /// Returns `Err` if `state` cannot be mapped onto the manifest's typed
    /// structure, either because a required field is missing, holds a
    /// value of the wrong kind, holds text with no known semantic
    /// meaning, describes an impossible combination, or requires
    /// information the current `Field` model does not yet represent.
    pub fn build(state: &SemanticState) -> Result<ProjectManifest, ManifestBuildError> {
        let metadata = build_metadata(state)?;
        let frontend = build_frontend(state)?;
        let backend = build_backend(state)?;
        let tooling = build_tooling(state)?;

        Ok(ProjectManifest {
            metadata,
            frontend,
            backend,
            tooling,
            // The manifest is fully assembled from an (assumed) validated
            // state, but has not yet been used to generate a project.
            state: ProjectState::Validated,
        })
    }
}

/// Builds `ProjectMetadata` from `state`.
// TODO: `Field` has no `ProjectName` variant yet (see the same gap noted in
// `engine::registry`), so the project name cannot currently be sourced from
// `SemanticState`. Once `Field::ProjectName` exists and the corresponding
// question is added to the registry, replace this with a real read via
// `text_value(state, &Field::ProjectName)`.
fn build_metadata(_state: &SemanticState) -> Result<ProjectMetadata, ManifestBuildError> {
    Err(ManifestBuildError::UnmodeledField("project.metadata.name"))
}

/// Builds an `Option<FrontendManifest>` from `state`.
///
/// Returns `Ok(None)` if the frontend was not enabled. If it was enabled,
/// building currently always fails — see the TODO below.
// TODO: `Field` has no variants for frontend routing, styling, or state
// management (see the same gap noted in `engine::registry`). Once those
// variants exist and their questions are added to the registry, this
// function can read them and construct a complete `FrontendManifest`
// instead of returning `UnmodeledField`.
fn build_frontend(state: &SemanticState) -> Result<Option<FrontendManifest>, ManifestBuildError> {
    if !state.is_enabled(&Field::FrontendEnabled) {
        return Ok(None);
    }

    // These two are modeled and can be read today, even though the
    // manifest as a whole cannot yet be completed:
    let _language = map_frontend_language(text_value(state, &Field::FrontendLanguage)?)?;
    let _framework = map_frontend_framework(text_value(state, &Field::FrontendFramework)?)?;

    Err(ManifestBuildError::UnmodeledField(
        "frontend.routing / frontend.styling / frontend.state_management",
    ))
}

/// Builds an `Option<BackendManifest>` from `state`.
///
/// Returns `Ok(None)` if the backend was not enabled.
fn build_backend(state: &SemanticState) -> Result<Option<BackendManifest>, ManifestBuildError> {
    if !state.is_enabled(&Field::BackendEnabled) {
        // A database cannot exist without a backend to own it. This isn't
        // reachable through the current registry (database.enabled
        // depends on backend.enabled being true), but the builder checks
        // it explicitly rather than assuming the registry's dependency
        // graph is the only thing that can ever produce a SemanticState.
        if state.is_enabled(&Field::DatabaseEnabled) {
            return Err(ManifestBuildError::ImpossibleState(
                "database is enabled but backend is not",
            ));
        }
        return Ok(None);
    }

    let language = map_backend_language(text_value(state, &Field::BackendLanguage)?)?;
    let framework = map_backend_framework(text_value(state, &Field::BackendFramework)?)?;
    let database = build_database(state)?;
    let authentication = map_authentication(text_value(state, &Field::Authentication)?)?;
    let api_style = map_api_style(text_value(state, &Field::ApiStyle)?)?;

    Ok(Some(BackendManifest::new(
        language,
        framework,
        database,
        authentication,
        api_style,
    )))
}

/// Builds an `Option<DatabaseManifest>` from `state`.
///
/// Returns `Ok(None)` if the database was not enabled.
fn build_database(state: &SemanticState) -> Result<Option<DatabaseManifest>, ManifestBuildError> {
    if !state.is_enabled(&Field::DatabaseEnabled) {
        return Ok(None);
    }

    let engine = map_database_engine(text_value(state, &Field::DatabaseEngine)?)?;
    let orm = map_orm(text_value(state, &Field::DatabaseOrm)?)?;

    Ok(Some(DatabaseManifest::new(engine, orm)))
}

/// Builds a `ToolingManifest` from `state`.
///
/// Unlike frontend/backend, tooling is not optional at the manifest level —
/// every project has a `ToolingManifest`, even if every individual choice
/// within it is "off".
fn build_tooling(state: &SemanticState) -> Result<ToolingManifest, ManifestBuildError> {
    let git = if bool_value(state, &Field::GitEnabled)? {
        Git::Enabled
    } else {
        Git::Disabled
    };

    let docker = if bool_value(state, &Field::DockerEnabled)? {
        Docker::Enabled
    } else {
        Docker::Disabled
    };

    let testing = map_testing(text_value(state, &Field::Testing)?)?;

    Ok(ToolingManifest::new(git, docker, testing))
}

// ---- Value accessors ----

/// Reads `field` from `state` as text, or fails with the appropriate error
/// if it is missing or holds a non-text value.
fn text_value<'a>(state: &'a SemanticState, field: &Field) -> Result<&'a str, ManifestBuildError> {
    match state.get(field) {
        Some(Value::Text(text)) => Ok(text.as_str()),
        Some(Value::Bool(_)) => Err(ManifestBuildError::InvalidValueType {
            field: field.clone(),
            expected: "text",
        }),
        None => Err(ManifestBuildError::MissingField(field.clone())),
    }
}

/// Reads `field` from `state` as a boolean, or fails with the appropriate
/// error if it is missing or holds a non-boolean value.
fn bool_value(state: &SemanticState, field: &Field) -> Result<bool, ManifestBuildError> {
    match state.get(field) {
        Some(Value::Bool(value)) => Ok(*value),
        Some(Value::Text(_)) => Err(ManifestBuildError::InvalidValueType {
            field: field.clone(),
            expected: "boolean",
        }),
        None => Err(ManifestBuildError::MissingField(field.clone())),
    }
}

// ---- Text-to-enum mappings ----
//
// Each mapping corresponds to the canonical label text declared for that
// field's options in `engine::registry`.

fn map_frontend_language(value: &str) -> Result<FrontendLanguage, ManifestBuildError> {
    match value {
        "TypeScript" => Ok(FrontendLanguage::TypeScript),
        "JavaScript" => Ok(FrontendLanguage::JavaScript),
        other => Err(unknown(Field::FrontendLanguage, other)),
    }
}

fn map_frontend_framework(value: &str) -> Result<FrontendFramework, ManifestBuildError> {
    match value {
        "React" => Ok(FrontendFramework::React),
        "Vue" => Ok(FrontendFramework::Vue),
        "Svelte" => Ok(FrontendFramework::Svelte),
        "Solid" => Ok(FrontendFramework::Solid),
        other => Err(unknown(Field::FrontendFramework, other)),
    }
}

fn map_backend_language(value: &str) -> Result<BackendLanguage, ManifestBuildError> {
    match value {
        "Python" => Ok(BackendLanguage::Python),
        "Go" => Ok(BackendLanguage::Go),
        "Rust" => Ok(BackendLanguage::Rust),
        "Node.js" => Ok(BackendLanguage::NodeJs),
        "PHP" => Ok(BackendLanguage::Php),
        "Java" => Ok(BackendLanguage::Java),
        "C#" => Ok(BackendLanguage::CSharp),
        other => Err(unknown(Field::BackendLanguage, other)),
    }
}

fn map_backend_framework(value: &str) -> Result<BackendFramework, ManifestBuildError> {
    match value {
        "Django" => Ok(BackendFramework::Django),
        "FastAPI" => Ok(BackendFramework::FastApi),
        "Flask" => Ok(BackendFramework::Flask),
        "Gin" => Ok(BackendFramework::Gin),
        "Echo" => Ok(BackendFramework::Echo),
        "Fiber" => Ok(BackendFramework::Fiber),
        "Axum" => Ok(BackendFramework::Axum),
        "Actix Web" => Ok(BackendFramework::ActixWeb),
        "Rocket" => Ok(BackendFramework::Rocket),
        "Express" => Ok(BackendFramework::Express),
        "Fastify" => Ok(BackendFramework::Fastify),
        "NestJS" => Ok(BackendFramework::NestJs),
        "Laravel" => Ok(BackendFramework::Laravel),
        "Symfony" => Ok(BackendFramework::Symfony),
        "Spring Boot" => Ok(BackendFramework::SpringBoot),
        "ASP.NET Core" => Ok(BackendFramework::AspNetCore),
        other => Err(unknown(Field::BackendFramework, other)),
    }
}

fn map_database_engine(value: &str) -> Result<DatabaseEngine, ManifestBuildError> {
    match value {
        "PostgreSQL" => Ok(DatabaseEngine::PostgreSql),
        "MySQL" => Ok(DatabaseEngine::MySql),
        "SQLite" => Ok(DatabaseEngine::Sqlite),
        "MongoDB" => Ok(DatabaseEngine::MongoDb),
        other => Err(unknown(Field::DatabaseEngine, other)),
    }
}

fn map_orm(value: &str) -> Result<Orm, ManifestBuildError> {
    match value {
        "Django ORM" => Ok(Orm::DjangoOrm),
        "SQLAlchemy" => Ok(Orm::SqlAlchemy),
        "SQLModel" => Ok(Orm::SqlModel),
        "Tortoise ORM" => Ok(Orm::TortoiseOrm),
        "GORM" => Ok(Orm::Gorm),
        "Bun" => Ok(Orm::Bun),
        "Ent" => Ok(Orm::Ent),
        "SQLC" => Ok(Orm::Sqlc),
        "SQLx" => Ok(Orm::SqlX),
        "Diesel" => Ok(Orm::Diesel),
        "SeaORM" => Ok(Orm::SeaOrm),
        "Prisma" => Ok(Orm::Prisma),
        "Drizzle" => Ok(Orm::Drizzle),
        "TypeORM" => Ok(Orm::TypeOrm),
        "Sequelize" => Ok(Orm::Sequelize),
        "Mongoose" => Ok(Orm::Mongoose),
        "Eloquent" => Ok(Orm::Eloquent),
        "Doctrine" => Ok(Orm::Doctrine),
        "Hibernate" => Ok(Orm::Hibernate),
        "EclipseLink" => Ok(Orm::EclipseLink),
        "Entity Framework Core" => Ok(Orm::EntityFrameworkCore),
        "Dapper" => Ok(Orm::Dapper),
        other => Err(unknown(Field::DatabaseOrm, other)),
    }
}

fn map_authentication(value: &str) -> Result<Authentication, ManifestBuildError> {
    match value {
        "None" => Ok(Authentication::None),
        "JWT" => Ok(Authentication::Jwt),
        "Sessions" => Ok(Authentication::Sessions),
        "OAuth" => Ok(Authentication::OAuth),
        other => Err(unknown(Field::Authentication, other)),
    }
}

fn map_api_style(value: &str) -> Result<ApiStyle, ManifestBuildError> {
    match value {
        "REST" => Ok(ApiStyle::Rest),
        "GraphQL" => Ok(ApiStyle::GraphQl),
        other => Err(unknown(Field::ApiStyle, other)),
    }
}

fn map_testing(value: &str) -> Result<Testing, ManifestBuildError> {
    match value {
        "None" => Ok(Testing::None),
        "Unit" => Ok(Testing::Unit),
        "Unit + Integration" => Ok(Testing::UnitAndIntegration),
        other => Err(unknown(Field::Testing, other)),
    }
}

fn unknown(field: Field, value: &str) -> ManifestBuildError {
    ManifestBuildError::UnknownVariant {
        field,
        value: value.to_string(),
    }
}

// TODO: as future manifest fields are introduced (plugins, mobile targets,
// desktop targets, etc.), add corresponding `build_*` functions here and
// wire their results into `ManifestBuilder::build`, following the same
// pattern used for frontend/backend/database/tooling above.