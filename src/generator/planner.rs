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
use super::template::{ReadmeTemplate, Template};

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
/// `DefaultPlanner` owns a collection of registered `Template`s and builds
/// a plan by asking each one, in registration order, whether it applies to
/// the manifest and, if so, letting it contribute its own operations. This
/// is the planner's only role: template *selection* and *orchestration* —
/// it never decides what a template's contribution should look like, and
/// it never touches the filesystem itself.
///
/// Only `ReadmeTemplate` is registered so far. The remaining sections a
/// project needs (frontend, backend, tooling) are still produced by
/// hardcoded, illustrative operations below rather than real `Template`s,
/// exactly as before — that hardcoding is a stopgap pending the concrete
/// templates listed in the TODOs on `plan_frontend`/`plan_backend`/
/// `plan_tooling` and in `template.rs`. As each of those templates is
/// implemented, it should be registered here and its corresponding
/// hardcoded method removed.
// TODO: discover applicable `Template`s for the manifest's frontend
// framework/meta-framework, backend language/framework, database
// engine/ORM, and tooling choices, and register them below as they're
// implemented (Cargo.toml, package.json, .gitignore, Next.js, Django,
// Axum — see `template.rs`), replacing the corresponding hardcoded
// `plan_*` method.
// TODO: respect template dependencies once they exist (see the TODOs on
// `TemplateMetadata`), rather than assuming registration order alone is a
// sufficient contribution order.
// TODO: surface `GenerationError::InvalidManifest` for manifests this
// planner recognizes as unplannable, rather than assuming every reachable
// combination below is always plannable as it does today.
pub struct DefaultPlanner {
    templates: Vec<Box<dyn Template>>,
}

impl DefaultPlanner {
    /// Creates a new `DefaultPlanner`, registered with the Version 1 set
    /// of built-in templates.
    pub fn new() -> Self {
        Self {
            templates: vec![Box::new(ReadmeTemplate::new())],
        }
    }

    /// Runs every registered template against `manifest`, appending the
    /// contribution of each template whose `applies_to` returns `true`.
    ///
    /// Templates are consulted in registration order; each applicable
    /// template's `contribute` is called in turn, and any error it
    /// returns is propagated immediately.
    fn plan_templates(&self, manifest: &ProjectManifest, plan: &mut GenerationPlan) -> Result<(), GenerationError> {
        for template in &self.templates {
            if template.applies_to(manifest) {
                template.contribute(manifest, plan)?;
            }
        }
        Ok(())
    }

    /// Appends the frontend's operations to `plan`, if a frontend is
    /// present in `manifest`.
    ///
    /// Hardcoded pending real frontend `Template`s (package.json, Next.js,
    /// and so on — see `template.rs`); not yet expressed via the
    /// `Template` trait.
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
    ///
    /// Hardcoded pending real backend `Template`s (Django, Axum, and so
    /// on — see `template.rs`); not yet expressed via the `Template`
    /// trait.
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
    /// has a `ToolingManifest`, so this is always called. Hardcoded
    /// pending real tooling `Template`s (.gitignore, an infra template for
    /// Docker, and so on — see `template.rs`); not yet expressed via the
    /// `Template` trait.
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
    /// Builds a plan by first running every registered `Template` against
    /// `manifest` (currently just `ReadmeTemplate`), then appending the
    /// remaining hardcoded frontend/backend/tooling operations for
    /// whichever sections are present.
    ///
    /// This is a transitional implementation — see the TODOs on
    /// `DefaultPlanner` for how the hardcoded portion is expected to
    /// shrink as more concrete `Template`s are registered.
    fn plan(&self, manifest: &ProjectManifest) -> Result<GenerationPlan, GenerationError> {
        let mut plan = GenerationPlan::new();

        self.plan_templates(manifest, &mut plan)?;

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