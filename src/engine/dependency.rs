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
///
/// Derives `Hash` (alongside `Eq`) so `Field` can be used as a key in
/// hash-based collections, such as the map backing `SemanticState` in
/// `engine::resolver`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Field {
    ProjectName,

    FrontendEnabled,
    FrontendLanguage,
    FrontendFramework,
    FrontendRouting,
    FrontendStyling,
    FrontendStateManagement,

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
///
/// Derives `Hash` (alongside `Eq`) for consistency with `Field` and to
/// support potential future use as, or within, a hash-based collection key.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    Bool(bool),
    Text(String),
    // TODO: numeric or version-aware values are not yet needed.
}

/// A condition that a resolver can later evaluate against a project's
/// current semantic state.
///
/// `Condition` is purely descriptive. It does not know how to evaluate
/// itself; that responsibility belongs to the resolver.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Condition {
    /// The given field represents a toggle-like entity that must be enabled.
    Enabled(Field),

    /// The given field represents a toggle-like entity that must be disabled.
    Disabled(Field),

    /// The given field must have a selected value.
    IsPresent(Field),

    /// The given field must not have a selected value.
    IsAbsent(Field),

    /// The given field must equal the given value.
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
    // comparisons) are not yet specified.
}

/// A named dependency: a condition plus an optional human-readable
/// explanation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dependency {
    pub condition: Condition,
    pub description: Option<String>,
}

impl Dependency {
    /// Creates a new dependency from a condition.
    pub fn new(condition: Condition) -> Self {
        Self {
            condition,
            description: None,
        }
    }

    /// Creates a dependency with a description.
    pub fn with_description(
        condition: Condition,
        description: impl Into<String>,
    ) -> Self {
        Self {
            condition,
            description: Some(description.into()),
        }
    }
}