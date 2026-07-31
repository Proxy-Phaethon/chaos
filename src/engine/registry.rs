//! The complete Question Registry used by `chaos initialize`.
//!
//! This module defines the static collection of all Questions supported by
//! Version 1. It does not ask questions, resolve dependencies, validate
//! answers, or generate projects — it only constructs question metadata.
//!
//! # Known gaps
//!
//! The current `Field` enum (`engine::dependency`) does not yet define
//! variants for the project name, frontend routing, frontend styling, or
//! frontend state management. Those Version 1 tree nodes cannot be given a
//! valid `manifest_field` without inventing a `Field` variant, which this
//! file is not permitted to do. They are intentionally omitted below.
//!
//! TODO: add `Field::ProjectName`, `Field::Routing`, `Field::Styling`, and
//! `Field::StateManagement` to `engine::dependency`, then add the
//! corresponding questions here.

use super::dependency::{Condition, Dependency, Field, Value};
use super::question::{AnswerKind, Effect, Question, QuestionId, QuestionOption};

fn text_opt(value: &str, label: &str) -> QuestionOption {
    QuestionOption::new(Value::Text(value.to_string()), label)
}

fn bool_opt(value: bool, label: &str) -> QuestionOption {
    QuestionOption::new(Value::Bool(value), label)
}

fn equals(field: Field, value: &str) -> Condition {
    Condition::Equals(field, Value::Text(value.to_string()))
}

/// Returns the complete collection of Questions that make up the Version 1
/// `chaos initialize` tree.
pub fn registry() -> Vec<Question> {
    vec![
        // ---- Frontend ----
        Question::new(
            QuestionId::new("frontend.enabled"),
            "Will this project have a frontend?",
            None,
            AnswerKind::Boolean,
            vec![bool_opt(true, "Yes"), bool_opt(false, "No")],
            Some(Value::Bool(true)),
            vec![],
            Field::FrontendEnabled,
            vec![
                Effect::with_description(
                    Field::FrontendLanguage,
                    "If No, the frontend subtree is skipped entirely.",
                ),
                Effect::new(Field::FrontendFramework),
            ],
        ),
        Question::new(
            QuestionId::new("frontend.language"),
            "Which frontend language would you like to use?",
            None,
            AnswerKind::Choice,
            vec![text_opt("TypeScript", "TypeScript"), text_opt("JavaScript", "JavaScript")],
            Some(Value::Text("TypeScript".to_string())),
            vec![Dependency::new(Condition::Enabled(Field::FrontendEnabled))],
            Field::FrontendLanguage,
            vec![
                Effect::with_description(Field::FrontendFramework, "Determines available templates and package configuration."),
            ],
        ),
        Question::new(
            QuestionId::new("frontend.framework"),
            "Which frontend framework would you like to use?",
            None,
            AnswerKind::Choice,
            vec![
                text_opt("React", "React"),
                text_opt("Vue", "Vue"),
                text_opt("Svelte", "Svelte"),
                text_opt("Solid", "Solid"),
            ],
            Some(Value::Text("React".to_string())),
            vec![Dependency::new(Condition::Enabled(Field::FrontendEnabled))],
            Field::FrontendFramework,
            vec![
                Effect::with_description(Field::FrontendFramework, "Determines project structure, dependencies, routing options, and available state libraries."),
            ],
        ),

        // ---- Backend ----
        Question::new(
            QuestionId::new("backend.enabled"),
            "Will this project have a backend?",
            None,
            AnswerKind::Boolean,
            vec![bool_opt(true, "Yes"), bool_opt(false, "No")],
            Some(Value::Bool(true)),
            vec![],
            Field::BackendEnabled,
            vec![
                Effect::with_description(
                    Field::BackendLanguage,
                    "If No, the backend subtree is skipped entirely.",
                ),
                Effect::new(Field::BackendFramework),
            ],
        ),
        Question::new(
            QuestionId::new("backend.language"),
            "Which backend language would you like to use?",
            None,
            AnswerKind::Choice,
            vec![
                text_opt("Python", "Python"),
                text_opt("Go", "Go"),
                text_opt("Rust", "Rust"),
                text_opt("Node.js", "Node.js"),
                text_opt("PHP", "PHP"),
                text_opt("Java", "Java"),
                text_opt("C#", "C#"),
            ],
            None,
            vec![Dependency::new(Condition::Enabled(Field::BackendEnabled))],
            Field::BackendLanguage,
            vec![
                Effect::with_description(Field::BackendFramework, "Determines framework choices, templates, package manager, build commands, and database layer choices."),
            ],
        ),

        // Backend framework, one Question per language, gated by BackendLanguage.
        Question::new(
            QuestionId::new("backend.framework.python"),
            "Which backend framework would you like to use?",
            None,
            AnswerKind::Choice,
            vec![text_opt("Django", "Django"), text_opt("FastAPI", "FastAPI"), text_opt("Flask", "Flask")],
            Some(Value::Text("Django".to_string())),
            vec![Dependency::new(equals(Field::BackendLanguage, "Python"))],
            Field::BackendFramework,
            vec![Effect::new(Field::DatabaseOrm)],
        ),
        Question::new(
            QuestionId::new("backend.framework.go"),
            "Which backend framework would you like to use?",
            None,
            AnswerKind::Choice,
            vec![text_opt("Gin", "Gin"), text_opt("Echo", "Echo"), text_opt("Fiber", "Fiber")],
            Some(Value::Text("Gin".to_string())),
            vec![Dependency::new(equals(Field::BackendLanguage, "Go"))],
            Field::BackendFramework,
            vec![Effect::new(Field::DatabaseOrm)],
        ),
        Question::new(
            QuestionId::new("backend.framework.rust"),
            "Which backend framework would you like to use?",
            None,
            AnswerKind::Choice,
            vec![text_opt("Axum", "Axum"), text_opt("Actix Web", "Actix Web"), text_opt("Rocket", "Rocket")],
            Some(Value::Text("Axum".to_string())),
            vec![Dependency::new(equals(Field::BackendLanguage, "Rust"))],
            Field::BackendFramework,
            vec![Effect::new(Field::DatabaseOrm)],
        ),
        Question::new(
            QuestionId::new("backend.framework.nodejs"),
            "Which backend framework would you like to use?",
            None,
            AnswerKind::Choice,
            vec![text_opt("Express", "Express"), text_opt("Fastify", "Fastify"), text_opt("NestJS", "NestJS")],
            Some(Value::Text("Express".to_string())),
            vec![Dependency::new(equals(Field::BackendLanguage, "Node.js"))],
            Field::BackendFramework,
            vec![Effect::new(Field::DatabaseOrm)],
        ),
        Question::new(
            QuestionId::new("backend.framework.php"),
            "Which backend framework would you like to use?",
            None,
            AnswerKind::Choice,
            vec![text_opt("Laravel", "Laravel"), text_opt("Symfony", "Symfony")],
            Some(Value::Text("Laravel".to_string())),
            vec![Dependency::new(equals(Field::BackendLanguage, "PHP"))],
            Field::BackendFramework,
            vec![Effect::new(Field::DatabaseOrm)],
        ),
        Question::new(
            QuestionId::new("backend.framework.java"),
            "Which backend framework would you like to use?",
            None,
            AnswerKind::Choice,
            vec![text_opt("Spring Boot", "Spring Boot")],
            Some(Value::Text("Spring Boot".to_string())),
            vec![Dependency::new(equals(Field::BackendLanguage, "Java"))],
            Field::BackendFramework,
            vec![Effect::new(Field::DatabaseOrm)],
        ),
        Question::new(
            QuestionId::new("backend.framework.csharp"),
            "Which backend framework would you like to use?",
            None,
            AnswerKind::Choice,
            vec![text_opt("ASP.NET Core", "ASP.NET Core")],
            Some(Value::Text("ASP.NET Core".to_string())),
            vec![Dependency::new(equals(Field::BackendLanguage, "C#"))],
            Field::BackendFramework,
            vec![Effect::new(Field::DatabaseOrm)],
        ),

        // ---- Database ----
        Question::new(
            QuestionId::new("database.enabled"),
            "Would you like a database?",
            None,
            AnswerKind::Boolean,
            vec![bool_opt(true, "Yes"), bool_opt(false, "No")],
            Some(Value::Bool(false)),
            vec![Dependency::new(Condition::Enabled(Field::BackendEnabled))],
            Field::DatabaseEnabled,
            vec![
                Effect::with_description(Field::DatabaseEngine, "Enables the database engine and database layer questions, and migrations."),
                Effect::new(Field::DatabaseOrm),
            ],
        ),
        Question::new(
            QuestionId::new("database.engine"),
            "Which database engine would you like to use?",
            None,
            AnswerKind::Choice,
            vec![
                text_opt("PostgreSQL", "PostgreSQL"),
                text_opt("MySQL", "MySQL"),
                text_opt("SQLite", "SQLite"),
                text_opt("MongoDB", "MongoDB"),
            ],
            Some(Value::Text("PostgreSQL".to_string())),
            vec![Dependency::new(Condition::Enabled(Field::DatabaseEnabled))],
            Field::DatabaseEngine,
            vec![
                Effect::with_description(Field::DatabaseOrm, "Filters the available database layer options."),
            ],
        ),

        // Database layer, one Question per backend language, gated by DatabaseEnabled and BackendLanguage.
        Question::new(
            QuestionId::new("database.orm.python"),
            "Which database layer would you like to use?",
            None,
            AnswerKind::Choice,
            vec![
                text_opt("Django ORM", "Django ORM"),
                text_opt("SQLAlchemy", "SQLAlchemy"),
                text_opt("SQLModel", "SQLModel"),
                text_opt("Tortoise ORM", "Tortoise ORM"),
            ],
            Some(Value::Text("Django ORM".to_string())),
            vec![
                Dependency::new(Condition::Enabled(Field::DatabaseEnabled)),
                Dependency::new(equals(Field::BackendLanguage, "Python")),
            ],
            Field::DatabaseOrm,
            vec![],
        ),
        Question::new(
            QuestionId::new("database.orm.go"),
            "Which database layer would you like to use?",
            None,
            AnswerKind::Choice,
            vec![
                text_opt("GORM", "GORM"),
                text_opt("Bun", "Bun"),
                text_opt("Ent", "Ent"),
                text_opt("SQLC", "SQLC"),
            ],
            Some(Value::Text("GORM".to_string())),
            vec![
                Dependency::new(Condition::Enabled(Field::DatabaseEnabled)),
                Dependency::new(equals(Field::BackendLanguage, "Go")),
            ],
            Field::DatabaseOrm,
            vec![],
        ),
        Question::new(
            QuestionId::new("database.orm.rust"),
            "Which database layer would you like to use?",
            None,
            AnswerKind::Choice,
            vec![
                text_opt("SQLx", "SQLx"),
                text_opt("Diesel", "Diesel"),
                text_opt("SeaORM", "SeaORM"),
            ],
            Some(Value::Text("SQLx".to_string())),
            vec![
                Dependency::new(Condition::Enabled(Field::DatabaseEnabled)),
                Dependency::new(equals(Field::BackendLanguage, "Rust")),
            ],
            Field::DatabaseOrm,
            vec![],
        ),
        Question::new(
            QuestionId::new("database.orm.nodejs"),
            "Which database layer would you like to use?",
            None,
            AnswerKind::Choice,
            vec![
                text_opt("Prisma", "Prisma"),
                text_opt("Drizzle", "Drizzle"),
                text_opt("TypeORM", "TypeORM"),
                text_opt("Sequelize", "Sequelize"),
                text_opt("Mongoose", "Mongoose"),
            ],
            Some(Value::Text("Prisma".to_string())),
            vec![
                Dependency::new(Condition::Enabled(Field::DatabaseEnabled)),
                Dependency::new(equals(Field::BackendLanguage, "Node.js")),
            ],
            Field::DatabaseOrm,
            // TODO: Mongoose is only valid when DatabaseEngine == MongoDB.
            // Option-level dependencies are not yet supported by
            // `QuestionOption` (see engine::question).
            vec![],
        ),
        Question::new(
            QuestionId::new("database.orm.php"),
            "Which database layer would you like to use?",
            None,
            AnswerKind::Choice,
            vec![text_opt("Eloquent", "Eloquent"), text_opt("Doctrine", "Doctrine")],
            Some(Value::Text("Eloquent".to_string())),
            vec![
                Dependency::new(Condition::Enabled(Field::DatabaseEnabled)),
                Dependency::new(equals(Field::BackendLanguage, "PHP")),
            ],
            Field::DatabaseOrm,
            vec![],
        ),
        Question::new(
            QuestionId::new("database.orm.java"),
            "Which database layer would you like to use?",
            None,
            AnswerKind::Choice,
            vec![text_opt("Hibernate", "Hibernate"), text_opt("EclipseLink", "EclipseLink")],
            Some(Value::Text("Hibernate".to_string())),
            vec![
                Dependency::new(Condition::Enabled(Field::DatabaseEnabled)),
                Dependency::new(equals(Field::BackendLanguage, "Java")),
            ],
            Field::DatabaseOrm,
            vec![],
        ),
        Question::new(
            QuestionId::new("database.orm.csharp"),
            "Which database layer would you like to use?",
            None,
            AnswerKind::Choice,
            vec![text_opt("Entity Framework Core", "Entity Framework Core"), text_opt("Dapper", "Dapper")],
            Some(Value::Text("Entity Framework Core".to_string())),
            vec![
                Dependency::new(Condition::Enabled(Field::DatabaseEnabled)),
                Dependency::new(equals(Field::BackendLanguage, "C#")),
            ],
            Field::DatabaseOrm,
            vec![],
        ),

        // ---- Backend: Authentication & API Style ----
        Question::new(
            QuestionId::new("backend.authentication"),
            "Would you like authentication?",
            None,
            AnswerKind::Choice,
            vec![
                text_opt("None", "None"),
                text_opt("JWT", "JWT"),
                text_opt("Sessions", "Sessions"),
                text_opt("OAuth", "OAuth"),
            ],
            Some(Value::Text("None".to_string())),
            vec![Dependency::new(Condition::Enabled(Field::BackendEnabled))],
            Field::Authentication,
            vec![
                Effect::with_description(Field::Authentication, "Sessions generally expects a database. OAuth adds provider configuration."),
            ],
        ),
        Question::new(
            QuestionId::new("backend.api_style"),
            "Which API style would you like to use?",
            None,
            AnswerKind::Choice,
            vec![text_opt("REST", "REST"), text_opt("GraphQL", "GraphQL")],
            Some(Value::Text("REST".to_string())),
            vec![Dependency::new(Condition::Enabled(Field::BackendEnabled))],
            Field::ApiStyle,
            vec![
                Effect::with_description(Field::ApiStyle, "Determines routing structure, dependencies, and generated starter endpoints."),
            ],
        ),

        // ---- Tooling ----
        Question::new(
            QuestionId::new("tooling.git"),
            "Would you like to initialize a Git repository?",
            None,
            AnswerKind::Boolean,
            vec![bool_opt(true, "Yes"), bool_opt(false, "No")],
            Some(Value::Bool(true)),
            vec![],
            Field::GitEnabled,
            vec![],
        ),
        Question::new(
            QuestionId::new("tooling.docker"),
            "Would you like to use Docker?",
            None,
            AnswerKind::Boolean,
            vec![bool_opt(true, "Yes"), bool_opt(false, "No")],
            Some(Value::Bool(false)),
            vec![],
            Field::DockerEnabled,
            vec![],
        ),
        Question::new(
            QuestionId::new("tooling.testing"),
            "Would you like to add testing?",
            None,
            AnswerKind::Choice,
            vec![
                text_opt("None", "None"),
                text_opt("Unit", "Unit"),
                text_opt("Unit + Integration", "Unit + Integration"),
            ],
            Some(Value::Text("Unit".to_string())),
            vec![],
            Field::Testing,
            vec![],
        ),
    ]
}