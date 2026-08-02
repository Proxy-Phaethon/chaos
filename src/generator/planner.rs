//! Planning stage of the Generation subsystem.
//!
//! A `Planner` converts a validated `ProjectManifest` into a
//! `GenerationPlan`: an implementation-independent description of what
//! should be created. It decides *what* is needed by inspecting the
//! manifest's frontend, backend, database, and tooling — but it never
//! executes anything itself. It does not write files, does not create
//! directories, and does not render template content; it only describes
//! operations for a `FilesystemExecutor` (or equivalent) to carry out
//! later.

use crate::manifest::{
    Authentication, BackendFramework, BackendLanguage, BackendManifest, Docker,
    FrontendFramework, FrontendLanguage, FrontendManifest, Git, ProjectManifest, Testing,
};

use super::error::GenerationError;
use super::plan::GenerationPlan;

/// Builds a `GenerationPlan` from a `ProjectManifest`.
///
/// Implementations decide *what* operations a project needs by examining
/// the manifest's `frontend`, `backend`, `database` (nested within
/// `backend`), and `tooling` — but they do not execute those operations.
/// This keeps planning (a semantic decision) fully separate from execution
/// (a filesystem concern), which lives elsewhere in the Generation
/// subsystem.
pub trait Planner {
    /// Builds a plan describing everything required to generate the
    /// project described by `manifest`.
    ///
    /// Returns `Err` if the manifest cannot be planned for — for example,
    /// because it describes a combination the planner has no template or
    /// strategy for yet.
    fn plan(&self, manifest: &ProjectManifest) -> Result<GenerationPlan, GenerationError>;
}

/// The Version 1 default `Planner` implementation.
///
/// `DefaultPlanner` inspects a manifest's frontend, backend, and tooling
/// sections and appends a small, hardcoded set of operations for each —
/// directories, and representative `CreateFile`/`WriteFile`/`CopyTemplate`
/// entries. This stands in for real template discovery: the operations it
/// contributes today are illustrative placeholders, not the final output
/// content, and are expected to be replaced once the `Template` system
/// (see `generator::template`) has concrete implementations to discover
/// and delegate to instead.
// TODO: discover applicable `Template`s (see `generator::template`) for
// the manifest's frontend framework/meta-framework, backend
// language/framework, database engine/ORM, and tooling choices, and
// replace the hardcoded operations below with contributions gathered from
// them.
// TODO: ask each applicable template to contribute to the plan, in an
// order that respects template dependencies once those exist (see the
// TODOs on `TemplateMetadata`).
// TODO: surface `GenerationError::InvalidManifest` for manifests this
// planner recognizes as unplannable, rather than assuming every reachable
// combination below is always plannable as it does today.
pub struct DefaultPlanner;

impl DefaultPlanner {
    /// Creates a new `DefaultPlanner`.
    pub fn new() -> Self {
        Self
    }

    /// Appends the frontend's operations to `plan`, if a frontend is
    /// present in `manifest`.
    // TODO: routing and styling and state_management are not yet reflected
    // in any operation below — see `FrontendManifest`'s own fields, which
    // this planner does not yet consult beyond language and framework.
    fn plan_frontend(&self, frontend: &FrontendManifest, plan: &mut GenerationPlan) {
        plan.create_directory("frontend");

        let language_slug = match frontend.language {
            FrontendLanguage::TypeScript => "typescript",
            FrontendLanguage::JavaScript => "javascript",
        };
        let (framework_slug, entry_file) = match frontend.framework {
            FrontendFramework::React => ("react", "frontend/src/App"),
            FrontendFramework::Vue => ("vue", "frontend/src/App"),
            FrontendFramework::Svelte => ("svelte", "frontend/src/App"),
            FrontendFramework::Solid => ("solid", "frontend/src/App"),
        };

        // Illustrative only — the real entry file's extension, content,
        // and surrounding project structure belong to a future Template,
        // not to this hardcoded stopgap.
        let extension = match (frontend.framework, frontend.language) {
            (FrontendFramework::React, FrontendLanguage::TypeScript) => "tsx",
            (FrontendFramework::React, FrontendLanguage::JavaScript) => "jsx",
            (_, FrontendLanguage::TypeScript) => "ts",
            (_, FrontendLanguage::JavaScript) => "js",
        };

        plan.copy_template(
            format!("frontend/{}-{}", framework_slug, language_slug),
            format!("{}.{}", entry_file, extension),
        );

        plan.write_file(
            "frontend/package.json",
            "{\n  \"name\": \"frontend\",\n  \"private\": true\n}\n",
        );
    }

    /// Appends the backend's operations to `plan`, if a backend is
    /// present in `manifest`.
    // TODO: API style is not yet reflected in any operation below — see
    // `BackendManifest::api_style`, which this planner does not yet
    // consult.
    fn plan_backend(&self, backend: &BackendManifest, plan: &mut GenerationPlan) {
        plan.create_directory("backend");

        let (language_slug, entry_file) = match backend.language {
            BackendLanguage::Python => ("python", "backend/main.py"),
            BackendLanguage::Go => ("go", "backend/main.go"),
            BackendLanguage::Rust => ("rust", "backend/src/main.rs"),
            BackendLanguage::NodeJs => ("nodejs", "backend/index.js"),
            BackendLanguage::Php => ("php", "backend/index.php"),
            BackendLanguage::Java => ("java", "backend/Main.java"),
            BackendLanguage::CSharp => ("csharp", "backend/Program.cs"),
        };
        let framework_slug = match backend.framework {
            BackendFramework::Django => "django",
            BackendFramework::FastApi => "fastapi",
            BackendFramework::Flask => "flask",
            BackendFramework::Gin => "gin",
            BackendFramework::Echo => "echo",
            BackendFramework::Fiber => "fiber",
            BackendFramework::Axum => "axum",
            BackendFramework::ActixWeb => "actix-web",
            BackendFramework::Rocket => "rocket",
            BackendFramework::Express => "express",
            BackendFramework::Fastify => "fastify",
            BackendFramework::NestJs => "nestjs",
            BackendFramework::Laravel => "laravel",
            BackendFramework::Symfony => "symfony",
            BackendFramework::SpringBoot => "spring-boot",
            BackendFramework::AspNetCore => "aspnet-core",
        };

        plan.copy_template(
            format!("backend/{}-{}", language_slug, framework_slug),
            entry_file,
        );

        // TODO: the database engine/ORM (backend.database) should
        // contribute its own operations (e.g. a schema or config file)
        // once a database Template exists. Not yet represented.

        if backend.authentication != Authentication::None {
            // TODO: real auth scaffolding (middleware, config) belongs to
            // a future auth Template. For now, just note that it was
            // requested.
            plan.write_file(
                "backend/AUTH_TODO.md",
                "Authentication was requested for this project but is not yet scaffolded by Chaos.\n",
            );
        }
    }

    /// Appends tooling operations to `plan`.
    ///
    /// Unlike frontend/backend, tooling is not optional — every manifest
    /// has a `ToolingManifest`, so this is always called.
    // TODO: Docker::Enabled should contribute a Dockerfile / compose file
    // once an infra Template exists. Not yet represented.
    // TODO: Git::Enabled describes a repository that should be
    // initialized, which is a command to run, not a filesystem operation
    // this planner can express with today's `GenerationOperation`
    // variants — only the file that commonly accompanies it (.gitignore)
    // is planned here.
    fn plan_tooling(&self, manifest: &ProjectManifest, plan: &mut GenerationPlan) {
        if manifest.tooling.git == Git::Enabled {
            plan.write_file(".gitignore", "node_modules/\n.env\n");
        }

        if manifest.tooling.docker == Docker::Enabled {
            plan.write_file(
                "Dockerfile",
                "# Generated by Chaos — placeholder until a real infra Template exists.\n",
            );
        }

        match manifest.tooling.testing {
            Testing::None => {}
            Testing::Unit | Testing::UnitAndIntegration => {
                plan.create_directory("tests");
                // TODO: per-language test scaffolding belongs to a future
                // testing Template. This only reserves the directory.
            }
        }
    }
}

impl Default for DefaultPlanner {
    fn default() -> Self {
        Self::new()
    }
}

impl Planner for DefaultPlanner {
    /// Builds a plan by inspecting `manifest`'s frontend, backend, and
    /// tooling sections and appending a hardcoded set of operations for
    /// each present section, plus a root `README.md` that is always
    /// contributed.
    ///
    /// This is a stopgap implementation — see the TODOs on `DefaultPlanner`
    /// for what real, template-driven planning will eventually involve.
    fn plan(&self, manifest: &ProjectManifest) -> Result<GenerationPlan, GenerationError> {
        let mut plan = GenerationPlan::new();

        plan.write_file(
            "README.md",
            format!("# {}\n\nGenerated by Chaos.\n", manifest.metadata.name),
        );

        if let Some(frontend) = &manifest.frontend {
            self.plan_frontend(frontend, &mut plan);
        }

        if let Some(backend) = &manifest.backend {
            self.plan_backend(backend, &mut plan);
        }

        self.plan_tooling(manifest, &mut plan);

        Ok(plan)
    }
}