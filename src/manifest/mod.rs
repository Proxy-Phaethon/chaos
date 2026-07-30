//! Public entry point for the `manifest` package.
//!
//! Re-exports the primary types from each submodule so other modules can
//! import from `crate::manifest` directly, without needing to know the
//! internal module layout.

mod backend;
mod database;
mod frontend;
mod project;
mod tooling;

pub use backend::{ApiStyle, Authentication, BackendFramework, BackendLanguage, BackendManifest};
pub use database::{DatabaseEngine, DatabaseManifest, Orm};
pub use frontend::{
    FrontendFramework, FrontendLanguage, FrontendManifest, Routing, StateManagement, Styling,
};
pub use project::{ProjectManifest, ProjectMetadata, ProjectState};
pub use tooling::{Docker, Git, Testing, ToolingManifest}; 