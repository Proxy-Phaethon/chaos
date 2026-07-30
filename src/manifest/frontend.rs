//! Defines `FrontendManifest`, the semantic representation of a Chaos
//! project's frontend.
//!
//! This module contains data only: it describes what a frontend *is*, not
//! how it is validated or generated. Those responsibilities belong to other
//! modules.

/// The language a frontend is written in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrontendLanguage {
    TypeScript,
    JavaScript,
}

/// The frontend framework in use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrontendFramework {
    React,
    Vue,
    Svelte,
    Solid,
}

/// Whether routing is set up for the frontend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Routing {
    Enabled,
    Disabled,
}

/// The styling approach used by the frontend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Styling {
    None,
    Css,
    Scss,
    TailwindCss,
}

/// The state management library used by the frontend, if any.
///
/// Availability of a given variant is framework-dependent; this module does
/// not encode or enforce that relationship, since it holds data only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateManagement {
    None,
    ReduxToolkit,
    Zustand,
    Pinia,
    // TODO: additional state management options are not yet specified in
    // the architecture.
}

/// The semantic representation of a Chaos project's frontend.
///
/// `FrontendManifest` owns the properties that fully describe a frontend's
/// configuration. It performs no validation, generation, or serialization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrontendManifest {
    pub language: FrontendLanguage,
    pub framework: FrontendFramework,
    pub routing: Routing,
    pub styling: Styling,
    pub state_management: StateManagement,
    // TODO: additional frontend properties are not yet specified in the
    // architecture.
}

impl FrontendManifest {
    /// Creates a new `FrontendManifest` from its constituent properties.
    pub fn new(
        language: FrontendLanguage,
        framework: FrontendFramework,
        routing: Routing,
        styling: Styling,
        state_management: StateManagement,
    ) -> Self {
        Self {
            language,
            framework,
            routing,
            styling,
            state_management,
        }
    }
}