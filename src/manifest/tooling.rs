//! Defines `ToolingManifest`, the semantic representation of a Chaos
//! project's tooling choices.
//!
//! Tooling choices are independent of the application stack (frontend,
//! backend, database). This module contains data only: it describes what
//! tooling is configured, not how it is validated or generated.

/// Whether a Git repository is initialized for the project.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Git {
    Enabled,
    Disabled,
}

/// Whether Docker support is generated for the project.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Docker {
    Enabled,
    Disabled,
}

/// The level of testing set up for the project.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Testing {
    None,
    Unit,
    UnitAndIntegration,
}

/// The semantic representation of a Chaos project's tooling choices.
///
/// `ToolingManifest` owns the properties that fully describe a project's
/// tooling configuration. It performs no validation, generation, or
/// serialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ToolingManifest {
    pub git: Git,
    pub docker: Docker,
    pub testing: Testing,
    // TODO: additional tooling properties are not yet specified in the
    // architecture.
}

impl ToolingManifest {
    /// Creates a new `ToolingManifest` from its constituent properties.
    pub fn new(git: Git, docker: Docker, testing: Testing) -> Self {
        Self {
            git,
            docker,
            testing,
        }
    }
}