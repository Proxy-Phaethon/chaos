//! Semantic validation for the Chaos Engine.
//!
//! This module determines whether a normalized `Value` is valid for a
//! particular `Question`. It is a pure validation module: it does not
//! resolve dependencies, determine question availability, normalize raw
//! input, or touch `SemanticState` or `ProjectManifest`. Validation is the
//! stage after normalization:
//!
//! ```text
//! Question → RawAnswer → Normalizer → Validator → SemanticState
//! ```

use super::dependency::Value;
use super::question::{AnswerKind, Question};

/// A placeholder for future, richer validation diagnostics.
///
/// No variant is populated yet; this exists so that structured diagnostic
/// information (e.g. which specific option was expected, or a machine
/// readable error code) can be added later without changing
/// `ValidationResult`'s shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationDiagnostic {
    // TODO: e.g. ExpectedOneOf(Vec<String>), EmptyIdentifier, etc., once
    // richer diagnostics are needed.
}

/// The outcome of validating a normalized answer against a `Question`.
///
/// `ValidationResult` is descriptive only — it carries no behavior, and
/// callers decide what to do with an invalid result (e.g. re-prompt,
/// surface an error). CLI-friendly formatting and localization are
/// explicitly out of scope for this module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationResult {
    pub valid: bool,
    pub message: Option<String>,
    /// Reserved for future structured diagnostics. Always empty for now.
    pub diagnostics: Vec<ValidationDiagnostic>,
}

impl ValidationResult {
    /// Constructs a valid result with no message.
    pub fn valid() -> Self {
        Self {
            valid: true,
            message: None,
            diagnostics: Vec::new(),
        }
    }

    /// Constructs an invalid result with an attached human-readable
    /// message.
    pub fn invalid(message: impl Into<String>) -> Self {
        Self {
            valid: false,
            message: Some(message.into()),
            diagnostics: Vec::new(),
        }
    }
}

/// Validates a normalized `Value` against the metadata of `question`.
///
/// Validation rules are dispatched by the question's `AnswerKind`:
///
/// * `Identifier` — the value must be `Value::Text` and non-empty.
/// * `Boolean` — the value must be `Value::Bool`.
/// * `Choice` — the value must be `Value::Text` and match one of the
///   question's declared `options`, by value.
/// * `Text` — no additional constraints are applied; any value is valid.
///
/// New rules for existing `AnswerKind`s, or rules for future `AnswerKind`
/// variants, can be added inside this dispatch without changing the
/// function's signature.
pub fn validate(question: &Question, value: &Value) -> ValidationResult {
    match question.answer_kind {
        AnswerKind::Text => ValidationResult::valid(),
        AnswerKind::Identifier => validate_identifier(value),
        AnswerKind::Boolean => validate_boolean(value),
        AnswerKind::Choice => validate_choice(question, value),
    }
}

/// Validates an `Identifier` answer: must be text, and non-empty after
/// normalization.
fn validate_identifier(value: &Value) -> ValidationResult {
    match value {
        Value::Text(text) if !text.is_empty() => ValidationResult::valid(),
        Value::Text(_) => ValidationResult::invalid("An identifier must not be empty."),
        Value::Bool(_) => ValidationResult::invalid("Expected an identifier, got a boolean value."),
    }
}

/// Validates a `Boolean` answer: must be a `Value::Bool`.
///
/// This is where an ambiguous normalized answer (see `engine::normalizer`,
/// which deliberately does not coerce unrecognized boolean input) is
/// finally rejected, rather than silently accepted as one branch or the
/// other.
fn validate_boolean(value: &Value) -> ValidationResult {
    match value {
        Value::Bool(_) => ValidationResult::valid(),
        Value::Text(text) => ValidationResult::invalid(format!(
            "Expected yes or no, but couldn't interpret '{}' as either.",
            text
        )),
    }
}

/// Validates a `Choice` answer: must be text, and must match one of the
/// question's declared options by value.
fn validate_choice(question: &Question, value: &Value) -> ValidationResult {
    match value {
        Value::Text(_) => {
            let matches_option = question.options.iter().any(|option| &option.value == value);
            if matches_option {
                ValidationResult::valid()
            } else {
                ValidationResult::invalid(format!(
                    "'{}' is not one of the available options for this question.",
                    describe(value)
                ))
            }
        }
        Value::Bool(_) => ValidationResult::invalid("Expected a choice, got a boolean value."),
    }
}

/// Renders a `Value` for use in a human-readable validation message.
fn describe(value: &Value) -> String {
    match value {
        Value::Text(text) => text.clone(),
        Value::Bool(b) => b.to_string(),
    }
}