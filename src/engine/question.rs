//! Defines the semantic `Question` used by the Chaos Engine during project
//! initialization.
//!
//! A `Question` represents a single piece of information required to
//! construct a valid `ProjectManifest`. This module contains data only —
//! prompting, dependency resolution, validation, and generation are the
//! responsibilities of other modules.

use super::dependency::{Dependency, Field, Value};

/// A unique identifier for a `Question`.
///
/// Kept as a string-based newtype rather than an enum, since the set of
/// questions is expected to grow as the architecture evolves.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QuestionId(pub String);

impl QuestionId {
    /// Creates a new `QuestionId` from anything convertible to a `String`.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

/// The kind of answer a `Question` expects.
///
/// Every `Question` explicitly declares its `AnswerKind` so that later
/// stages of the engine (normalization, validation) know how to treat the
/// answer without needing to infer it from other fields, such as whether
/// `options` happens to be empty.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnswerKind {
    /// Free-form text with no further structure implied.
    Text,
    /// Text intended to become a canonical identifier (e.g. a project name).
    Identifier,
    /// A yes/no answer.
    Boolean,
    /// A selection among the question's `options`.
    Choice,
    // TODO: Number
    // TODO: Path
    // TODO: File
    // TODO: Directory
    // TODO: Version
    // TODO: Enum (a closed set of values not sourced from `options`)
}

/// A single selectable option offered by a `Question`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuestionOption {
    /// The value assigned to the manifest if this option is chosen.
    pub value: Value,
    /// The user-facing label for this option.
    pub label: String,
    // TODO: options may later need their own per-option dependencies
    // (e.g. an option only appearing under certain conditions), distinct
    // from the Question-level dependencies.
}

impl QuestionOption {
    /// Creates a new `QuestionOption` from a value and a label.
    pub fn new(value: Value, label: impl Into<String>) -> Self {
        Self {
            value,
            label: label.into(),
        }
    }
}

/// A semantic effect describing what a `Question`'s answer enables or
/// influences elsewhere in the manifest.
///
/// This is descriptive only — it does not perform the influence itself;
/// that is the responsibility of a future resolver.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Effect {
    /// The field elsewhere in the manifest that this answer affects.
    pub affects: Field,
    /// An optional human-readable explanation of the effect.
    pub description: Option<String>,
    // TODO: effects may later need to express *how* a field is affected
    // (e.g. filters its options, sets a default), not just that it is.
}

impl Effect {
    /// Creates a new effect with no description.
    pub fn new(affects: Field) -> Self {
        Self {
            affects,
            description: None,
        }
    }

    /// Creates a new effect with an attached human-readable description.
    pub fn with_description(affects: Field, description: impl Into<String>) -> Self {
        Self {
            affects,
            description: Some(description.into()),
        }
    }
}

/// A single piece of information required to construct a valid
/// `ProjectManifest`.
///
/// `Question` is purely descriptive. It does not know how to prompt for
/// itself, resolve its dependencies, or validate an answer; those
/// responsibilities belong to other modules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Question {
    pub id: QuestionId,
    pub prompt: String,
    pub description: Option<String>,
    pub answer_kind: AnswerKind,
    pub options: Vec<QuestionOption>,
    pub default: Option<Value>,
    pub dependencies: Vec<Dependency>,
    /// Where this answer belongs inside the `ProjectManifest`. This is a
    /// semantic destination, not a generation target.
    pub manifest_field: Field,
    pub effects: Vec<Effect>,
    // TODO: free-form (non-option-list) questions, such as the project name
    // prompt, are not yet represented distinctly from selectable ones
    // beyond their `AnswerKind`.
}

impl Question {
    /// Creates a new `Question` from its constituent parts.
    pub fn new(
        id: QuestionId,
        prompt: impl Into<String>,
        description: Option<String>,
        answer_kind: AnswerKind,
        options: Vec<QuestionOption>,
        default: Option<Value>,
        dependencies: Vec<Dependency>,
        manifest_field: Field,
        effects: Vec<Effect>,
    ) -> Self {
        Self {
            id,
            prompt: prompt.into(),
            description,
            answer_kind,
            options,
            default,
            dependencies,
            manifest_field,
            effects,
        }
    }
}