//! Defines the semantic normalization system used by the Chaos Engine.
//!
//! Normalization sits between raw user input and validation:
//!
//! ```text
//! Question → RawAnswer → Normalizer → Validator → ProjectManifest
//! ```
//!
//! A normalizer never rejects input — it only performs safe, deterministic
//! transformations (trimming, case folding, alias resolution, and so on).
//! Rejecting malformed or semantically invalid input is the responsibility
//! of the validator, not this module. Normalization is driven by a
//! `Question`'s `AnswerKind`, rather than a separate strategy type.

use super::dependency::Value;
use super::question::AnswerKind;

/// Raw, unprocessed input as received from the user, prior to normalization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawAnswer(pub String);

impl RawAnswer {
    /// Creates a new `RawAnswer` from anything convertible to a `String`.
    pub fn new(input: impl Into<String>) -> Self {
        Self(input.into())
    }
}

/// Transforms a `RawAnswer` into a canonical `Value` according to an
/// `AnswerKind`.
///
/// This trait exists separately from `AnswerKind` itself so that future
/// normalization behavior can be implemented without changing the enum in
/// `engine::question`.
pub trait Normalizer {
    /// Normalizes a raw answer into a canonical value. Never fails.
    ///
    /// `choices` supplies the canonical labels to match against when `self`
    /// is `AnswerKind::Choice`; it is ignored for all other kinds. Passing
    /// an empty slice for a `Choice` answer simply yields no match, which
    /// falls back to trimmed text — the validator, not this module, is
    /// responsible for judging whether that is acceptable.
    fn normalize(&self, raw: &RawAnswer, choices: &[String]) -> Value;
}

impl Normalizer for AnswerKind {
    fn normalize(&self, raw: &RawAnswer, choices: &[String]) -> Value {
        match self {
            AnswerKind::Text => Value::Text(collapse_whitespace(&trim(&raw.0))),
            AnswerKind::Identifier => Value::Text(to_identifier(&raw.0)),
            AnswerKind::Boolean => normalize_boolean(&raw.0),
            AnswerKind::Choice => Value::Text(match_case_insensitive(&raw.0, choices)),
        }
    }
}

/// Trims leading and trailing whitespace.
fn trim(input: &str) -> String {
    input.trim().to_string()
}

/// Collapses runs of internal whitespace into a single space, after
/// trimming leading and trailing whitespace.
fn collapse_whitespace(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Converts input into a canonical identifier: trims whitespace, lowercases
/// it, and collapses any run of non-alphanumeric characters into a single
/// hyphen, with no leading or trailing hyphens.
fn to_identifier(input: &str) -> String {
    let lowered = input.trim().to_lowercase();
    let mut result = String::new();
    let mut last_was_separator = false;

    for c in lowered.chars() {
        if c.is_alphanumeric() {
            result.push(c);
            last_was_separator = false;
        } else if !last_was_separator && !result.is_empty() {
            result.push('-');
            last_was_separator = true;
        }
    }

    result.trim_end_matches('-').to_string()
}

/// Interprets common yes/no aliases, case-insensitively.
///
/// Recognized aliases normalize to a canonical `Value::Bool`. Unrecognized
/// input is deliberately **not** coerced into `false` — silently forcing
/// ambiguous input to a specific boolean would hide the ambiguity from the
/// validator. Instead, unrecognized input normalizes to trimmed,
/// lowercased `Value::Text`, letting the validator decide whether it is
/// acceptable.
fn normalize_boolean(input: &str) -> Value {
    let trimmed_lower = input.trim().to_lowercase();
    match trimmed_lower.as_str() {
        "y" | "yes" | "true" | "1" => Value::Bool(true),
        "n" | "no" | "false" | "0" => Value::Bool(false),
        _ => Value::Text(trimmed_lower),
    }
}

/// Matches trimmed input against a list of canonical labels,
/// case-insensitively, returning the canonical label's own casing on a
/// match, or the trimmed input unchanged if no label matches.
fn match_case_insensitive(input: &str, canonical_labels: &[String]) -> String {
    let trimmed = input.trim();
    for label in canonical_labels {
        if label.eq_ignore_ascii_case(trimmed) {
            return label.clone();
        }
    }
    trimmed.to_string()
}