//! Semantic dependency resolution for the Chaos Engine.
//!
//! This module determines which `Question`s from the registry are
//! currently available, given the answers collected so far. It is a pure
//! reasoning module: it evaluates the dependency graph declared in
//! `engine::dependency` against an intermediate semantic state, and does
//! nothing else. It does not prompt, normalize, validate, generate files,
//! or touch `ProjectManifest`.

use std::collections::HashMap;

use super::dependency::{Condition, Dependency, Field, Value};
use super::question::Question;

/// The current semantic state of an in-progress initialization.
///
/// `SemanticState` is deliberately independent of `ProjectManifest`: it is
/// a flat map of the fields that have been answered so far, keyed by
/// `Field`. Fields that have not yet been answered are simply absent from
/// the map, rather than present with a placeholder value.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SemanticState {
    answers: HashMap<Field, Value>,
}

impl SemanticState {
    /// Creates a new, empty semantic state, representing the start of
    /// initialization before any question has been answered.
    pub fn new() -> Self {
        Self {
            answers: HashMap::new(),
        }
    }

    /// Creates a semantic state from an existing collection of answers.
    pub fn from_answers(answers: HashMap<Field, Value>) -> Self {
        Self { answers }
    }

    /// Returns the value currently recorded for `field`, if any.
    pub fn get(&self, field: &Field) -> Option<&Value> {
        self.answers.get(field)
    }

    /// Returns `true` if `field` has been answered, regardless of value.
    pub fn is_present(&self, field: &Field) -> bool {
        self.get(field).is_some()
    }

    /// Returns `true` if `field` has not been answered.
    pub fn is_absent(&self, field: &Field) -> bool {
        self.get(field).is_none()
    }

    /// Returns `true` if `field` has been answered with `Value::Bool(true)`.
    ///
    /// An unanswered field is not considered enabled.
    pub fn is_enabled(&self, field: &Field) -> bool {
        matches!(self.get(field), Some(Value::Bool(true)))
    }

    /// Returns `true` if `field` has been answered with `Value::Bool(false)`.
    ///
    /// An unanswered field is not considered disabled — it is simply
    /// unknown, which is distinct from an explicit "no".
    pub fn is_disabled(&self, field: &Field) -> bool {
        matches!(self.get(field), Some(Value::Bool(false)))
    }

    /// Returns `true` if `field` has been answered with exactly `expected`.
    ///
    /// An unanswered field never equals `expected`.
    pub fn equals(&self, field: &Field, expected: &Value) -> bool {
        self.get(field) == Some(expected)
    }

    /// Returns a new `SemanticState` with `field` set to `value`, leaving
    /// `self` unmodified.
    ///
    /// This does not mutate `self` — the resolver and its state are
    /// side-effect free. Callers that need to build up state over the
    /// course of initialization thread the returned value forward
    /// themselves.
    pub fn with_answer(&self, field: Field, value: Value) -> Self {
        let mut answers = self.answers.clone();
        answers.insert(field, value);
        Self { answers }
    }
}

/// Evaluates a `Condition` against a `SemanticState`.
///
/// Evaluation is total and never panics, even when a `Condition` refers to
/// a `Field` that has not yet been answered. Conditions that require
/// knowledge of an unanswered field evaluate conservatively to `false`,
/// with the single exception of `IsAbsent`, which is specifically about the
/// absence of an answer. This keeps resolution safe to call at any point
/// during initialization, including before any question has been answered.
pub fn evaluate_condition(condition: &Condition, state: &SemanticState) -> bool {
    match condition {
        Condition::Enabled(field) => state.is_enabled(field),
        Condition::Disabled(field) => state.is_disabled(field),
        Condition::IsPresent(field) => state.is_present(field),
        Condition::IsAbsent(field) => state.is_absent(field),
        Condition::Equals(field, expected) => state.equals(field, expected),
        Condition::NotEquals(field, expected) => match state.get(field) {
            Some(actual) => actual != expected,
            // The field hasn't been answered yet, so we don't actually
            // know whether it differs from `expected`. Evaluating to
            // `false` here mirrors `Equals`: any condition requiring
            // knowledge we don't have is not satisfied.
            None => false,
        },
        Condition::And(conditions) => conditions
            .iter()
            .all(|condition| evaluate_condition(condition, state)),
        Condition::Or(conditions) => conditions
            .iter()
            .any(|condition| evaluate_condition(condition, state)),
        Condition::Not(condition) => !evaluate_condition(condition, state),
    }
}

/// Evaluates a `Dependency` against a `SemanticState`.
///
/// A `Dependency`'s description, if any, plays no role in evaluation; only
/// its `condition` is evaluated.
pub fn evaluate_dependency(dependency: &Dependency, state: &SemanticState) -> bool {
    evaluate_condition(&dependency.condition, state)
}

/// Determines whether a `Question` is currently available under the given
/// `SemanticState`.
///
/// A question with no dependencies is always available. A question with
/// one or more dependencies is available only if every dependency
/// evaluates to `true`.
pub fn is_question_available(question: &Question, state: &SemanticState) -> bool {
    question
        .dependencies
        .iter()
        .all(|dependency| evaluate_dependency(dependency, state))
}

/// A placeholder for future reasoning information explaining *why* a
/// `Question` is or isn't available.
///
/// No variant is populated by the resolver yet; this exists so that richer
/// diagnostics (e.g. which specific dependency was unsatisfied) can be
/// added later without changing `QuestionAvailability`'s shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AvailabilityReason {
    // TODO: e.g. UnsatisfiedDependency(Dependency), once dependency tracing
    // is implemented.
}

/// The result of resolving a single `Question` against a `SemanticState`:
/// the question itself, whether it is currently available, and room for
/// future reasoning about why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuestionAvailability<'a> {
    pub question: &'a Question,
    pub available: bool,
    /// Reserved for future dependency-tracing information. Always empty
    /// for now.
    pub reasons: Vec<AvailabilityReason>,
}

impl<'a> QuestionAvailability<'a> {
    /// Resolves a single `Question`'s availability against `state`.
    pub fn resolve(question: &'a Question, state: &SemanticState) -> Self {
        Self {
            question,
            available: is_question_available(question, state),
            reasons: Vec::new(),
        }
    }
}

/// Resolves every `Question` in `registry` against `state`, returning one
/// `QuestionAvailability` per question, in the registry's original order.
pub fn resolve_availability<'a>(
    registry: &'a [Question],
    state: &SemanticState,
) -> Vec<QuestionAvailability<'a>> {
    registry
        .iter()
        .map(|question| QuestionAvailability::resolve(question, state))
        .collect()
}

/// Returns every `Question` in `registry` that is currently available
/// under the given `SemanticState`, preserving the registry's original
/// order.
///
/// Built on top of `resolve_availability`, so it reflects the same
/// evaluation as the richer per-question results.
pub fn available_questions<'a>(
    registry: &'a [Question],
    state: &SemanticState,
) -> Vec<&'a Question> {
    resolve_availability(registry, state)
        .into_iter()
        .filter(|availability| availability.available)
        .map(|availability| availability.question)
        .collect()
}