//! Defines `ProjectManifest`, the semantic state of a Chaos project.
//!
//! The manifest aggregates every decision made during `chaos initialize`
//! (and later `chaos edit`) into a single structure. It holds no business
//! logic — only the data describing what a project is.

use super::backend::BackendManifest;
use super::frontend::FrontendManifest;
use super::tooling::ToolingManifest;

/// Identifying information for a Chaos project, independent of stack choices.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectMetadata {
    /// The sanitized project name. Used as the project folder name and in
    /// package/module naming.
    pub name: String,
    // TODO: additional metadata (version, description, author) is not
    // specified in the architecture yet.
}

impl ProjectMetadata {
    /// Creates metadata for a new project with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

/// The lifecycle state of a `ProjectManifest`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectState {
    /// The manifest is being built from user selections and has not been validated.
    Draft,
    /// The manifest has passed validation (e.g. at least one application layer is present).
    Validated,
    /// The project's files have been generated on disk.
    Generated,
    /// Generation completed successfully; the project is ready to use.
    Ready,
}

/// The semantic state of a Chaos project.
///
/// `ProjectManifest` owns project metadata plus each application layer's own
/// manifest. It performs no validation, generation, or serialization —
/// those are the responsibilities of other modules.
#[derive(Debug, Clone)]
pub struct ProjectManifest {
    pub metadata: ProjectMetadata,
    pub frontend: Option<FrontendManifest>,
    pub backend: Option<BackendManifest>,
    pub tooling: ToolingManifest,
    pub state: ProjectState,
}

impl ProjectManifest {
    /// Creates a new, empty manifest in the `Draft` state for the given
    /// project name. `frontend` and `backend` start unset; `tooling` is
    /// required since tooling choices apply regardless of which
    /// application layers are present.
    pub fn new(name: impl Into<String>, tooling: ToolingManifest) -> Self {
        Self {
            metadata: ProjectMetadata::new(name),
            frontend: None,
            backend: None,
            tooling,
            state: ProjectState::Draft,
        }
    }
}