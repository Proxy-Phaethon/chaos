//! Defines the semantic normalization system used by the Chaos Engine.
//!
//! Normalization sits between raw user input and validation:
//!
//! ```text
//! Question → Raw Answer → Normalizer → Validator → ProjectManifest
//! ```
//!
//! A normalizer never rejects input — it only performs safe, deterministic
//! transformations (trimming, case folding, alias resolution, and so on).
//! Rejecting malformed or semantically invalid input is the responsibility
//! of the validator, not this module.

use super::dependency::Value;

/// Raw, unprocessed input as received from the user, prior to normalization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawAnswer(pub String);

impl RawAnswer {
    /// Creates a new `RawAnswer` from anything convertible to a `String`.
    pub fn new(input: impl Into<String>) -> Self {
        Self(input.into())
    }
}

/// A strategy describing how a `RawAnswer` should be transformed into a
/// canonical `Value`.
///
/// Different questions may require different normalization behavior (a
/// project name is not normalized the same way as a yes/no answer), so
/// strategies are represented as distinct variants rather than a single
/// fixed transformation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NormalizationStrategy {
    /// Trims surrounding whitespace and collapses repeated internal
    /// whitespace, with no other transformation. Suitable for free-form
    /// text that will later be validated against a fixed set of options.
    Text,

    /// Converts input into a canonical identifier: trims whitespace, folds
    /// to a consistent case, and replaces runs of non-alphanumeric
    /// characters with a single separator. Suitable for project names and
    /// similar identifiers.
    Identifier,

    /// Interprets common yes/no aliases (e.g. "y", "yes", "true", "n",
    /// "no", "false") as a canonical boolean, case-insensitively. Input
    /// that matches no known alias is left as `false` rather than
    /// rejected — normalization never rejects input.
    Boolean,

    /// Matches input against a fixed list of canonical labels,
    /// case-insensitively and with surrounding whitespace trimmed,
    /// resolving to the canonical label's own casing when a match is
    /// found. Input matching no canonical label is passed through as
    /// trimmed text.
    CaseInsensitiveMatch(Vec<String>),
    // TODO: numeric normalization (e.g. parsing integers/versions) is not
    // yet needed by any Version 1 question.
    // TODO: locale-aware normalization is not yet specified.
}

/// Transforms a `RawAnswer` into a canonical `Value` according to a
/// `NormalizationStrategy`.
///
/// This trait exists separately from `NormalizationStrategy` itself so
/// that future normalization behavior (e.g. composite or question-specific
/// strategies) can implement it without changing the enum.
pub trait Normalizer {
    /// Normalizes a raw answer into a canonical value. Never fails.
    fn normalize(&self, raw: &RawAnswer) -> Value;
}

impl Normalizer for NormalizationStrategy {
    fn normalize(&self, raw: &RawAnswer) -> Value {
        match self {
            NormalizationStrategy::Text => Value::Text(collapse_whitespace(&trim(&raw.0))),
            NormalizationStrategy::Identifier => Value::Text(to_identifier(&raw.0)),
            NormalizationStrategy::Boolean => Value::Bool(parse_boolean_alias(&raw.0)),
            NormalizationStrategy::CaseInsensitiveMatch(canonical_labels) => {
                Value::Text(match_case_insensitive(&raw.0, canonical_labels))
            }
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

/// Interprets common yes/no aliases, case-insensitively, defaulting to
/// `false` for unrecognized input rather than rejecting it.
fn parse_boolean_alias(input: &str) -> bool {
    match input.trim().to_lowercase().as_str() {
        "y" | "yes" | "true" | "1" => true,
        // TODO: unrecognized input silently defaults to false. Whether
        // this is the correct default for every boolean question, versus
        // falling back to the question's own default value, is a decision
        // for the validator/resolver stage, not this module.
        _ => false,
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