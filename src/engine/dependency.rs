//! Defines the semantic dependency system used by the Chaos Engine.
//!
//! A dependency expresses a condition that must be satisfied before another
//! semantic entity becomes available (e.g. "Backend is enabled" or
//! "Backend framework equals Django"). This module defines dependency data
//! only — conditions are structured so a resolver can evaluate them later,
//! but no evaluation logic lives here.

/// Identifies a semantic field that a `Condition` may refer to.
///
/// Fields are grouped loosely by the entity they belong to. This is a flat
/// enumeration rather than a path into the manifest tree, keeping
/// dependency data independent of any particular manifest representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Field {
    FrontendEnabled,
    FrontendLanguage,
    FrontendFramework,

    BackendEnabled,
    BackendLanguage,
    BackendFramework,

    DatabaseEnabled,
    DatabaseEngine,
    DatabaseOrm,

    Authentication,
    ApiStyle,

    GitEnabled,
    DockerEnabled,
    Testing,
    // TODO: additional fields will be added as the manifest grows.
}

/// A value a `Field` may be compared against.
///
/// Kept generic rather than tied to specific manifest enums, so this module
/// does not depend on `crate::manifest`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Bool(bool),
    Text(String),
    // TODO: numeric or version-aware values are not yet needed.
}

/// A condition that a resolver can later evaluate against a project's
/// current semantic state.
///
/// `Condition` is purely descriptive. It does not know how to evaluate
/// itself; that responsibility belongs to a future resolver.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Condition {
    /// The given field represents a toggle-like entity that must be enabled.
    /// Example: "Backend is enabled."
    Enabled(Field),

    /// The given field represents a toggle-like entity that must be disabled.
    Disabled(Field),

    /// The given field must have a selected value, without constraining
    /// which one. Example: "Backend language has been selected."
    IsPresent(Field),

    /// The given field must not have a selected value.
    IsAbsent(Field),

    /// The given field must equal the given value.
    /// Example: "Backend framework equals Django."
    Equals(Field, Value),

    /// The given field must not equal the given value.
    NotEquals(Field, Value),

    /// All of the given conditions must hold.
    And(Vec<Condition>),

    /// At least one of the given conditions must hold.
    Or(Vec<Condition>),

    /// The given condition must not hold.
    Not(Box<Condition>),
    // TODO: future condition kinds (e.g. version constraints, cross-entity
    // comparisons) are not yet specified in the architecture.
}

/// A named dependency: a condition, plus an optional human-readable
/// description of why it exists.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dependency {
    pub condition: Condition,
    pub description: Option<String>,
}

impl Dependency {
    /// Creates a new dependency from a condition, with no description.
    pub fn new(condition: Condition) -> Self {
        Self {
            condition,
            description: None,
        }
    }

    /// Creates a new dependency from a condition with an attached
    /// human-readable description.
    pub fn with_description(condition: Condition, description: impl Into<String>) -> Self {
        Self {
            condition,
            description: Some(description.into()),
        }
    }
}