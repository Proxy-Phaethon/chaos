//! Public entry point for the Chaos Engine.
//!
//! The engine exposes project initialization as a single semantic system,
//! coordinating the question registry, dependency resolver, answer
//! normalizer, and answer validator behind one type: [`ChaosEngine`]. It
//! contains no CLI code — it never prints, reads stdin, or knows anything
//! about terminals. Callers (a future CLI layer) drive the engine through
//! semantic operations only.
//!
//! Once every question has been answered validly, the engine can hand its
//! accumulated `SemanticState` to `ManifestBuilder` (from `crate::manifest`)
//! to produce a `ProjectManifest`. The engine does not duplicate that
//! mapping logic itself — it only orchestrates the call.

mod dependency;
mod normalizer;
mod question;
mod registry;
mod resolver;
mod validator;

pub use dependency::{Condition, Dependency, Field, Value};
pub use normalizer::{Normalizer, RawAnswer};
pub use question::{AnswerKind, Effect, Question, QuestionId, QuestionOption};
pub use resolver::{AvailabilityReason, QuestionAvailability, SemanticState};
pub use validator::{ValidationDiagnostic, ValidationResult};

use crate::manifest::{ManifestBuildError, ManifestBuilder, ProjectManifest};

/// Describes why `ChaosEngine::answer_question` could not record an
/// answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnswerError {
    /// No question in the registry has the given `QuestionId`.
    UnknownQuestion,

    /// The normalized answer failed validation. Carries the `ValidationResult`
    /// that rejected it, including its message, for the caller to surface.
    Invalid(ValidationResult),
}

/// The Chaos Engine: the semantic core of `chaos initialize`.
///
/// `ChaosEngine` owns the Version 1 question registry and the
/// `SemanticState` accumulated so far, and coordinates the resolver,
/// normalizer, and validator to answer questions like "what should be
/// asked next", "what does this answer mean, semantically", and "is this
/// answer acceptable". It performs no I/O, no filesystem access, and no
/// project generation — those belong to other modules that consume the
/// engine's output. Manifest construction from a completed state is
/// delegated entirely to `ManifestBuilder`; the engine only orchestrates
/// the call.
pub struct ChaosEngine {
    registry: Vec<Question>,
    state: SemanticState,
}

impl ChaosEngine {
    /// Creates a new engine loaded with the Version 1 question registry and
    /// an empty `SemanticState`.
    pub fn new() -> Self {
        Self {
            registry: registry::registry(),
            state: SemanticState::new(),
        }
    }

    /// Returns the full question registry this engine was constructed with.
    pub fn registry(&self) -> &[Question] {
        &self.registry
    }

    /// Returns the engine's current semantic state.
    pub fn state(&self) -> &SemanticState {
        &self.state
    }

    /// Resolves and returns the availability of every question in the
    /// registry against the engine's current state.
    ///
    /// Delegates entirely to `resolver::resolve_availability` — the engine
    /// does not duplicate dependency evaluation logic.
    pub fn available_questions(&self) -> Vec<QuestionAvailability<'_>> {
        resolver::resolve_availability(&self.registry, &self.state)
    }

    /// Returns the next question that is currently available but has not
    /// yet been answered, in registry order, or `None` if every available
    /// question has already been answered.
    ///
    /// For Version 1 this is a simple linear scan. Branching or
    /// user-directed navigation (e.g. going back to a previous question)
    /// is not yet supported.
    pub fn next_question(&self) -> Option<&Question> {
        self.available_questions()
            .into_iter()
            .find(|availability| {
                availability.available && self.state.is_absent(&availability.question.manifest_field)
            })
            .map(|availability| availability.question)
    }

    /// Returns the question the engine currently considers "active".
    ///
    /// For Version 1 this is identical to [`Self::next_question`].
    // TODO: once the engine supports revisiting or editing previously
    // answered questions (e.g. for `chaos edit`), `current_question` should
    // diverge from `next_question` to reflect a distinct cursor/position
    // concept rather than always pointing at the next unanswered question.
    pub fn current_question(&self) -> Option<&Question> {
        self.next_question()
    }

    /// Normalizes `raw` according to the `AnswerKind` of the question
    /// identified by `question_id`, validates the result, and — if valid —
    /// records it in the engine's state under that question's
    /// `manifest_field`.
    ///
    /// Returns `Err(AnswerError::UnknownQuestion)` if no question with that
    /// id exists, or `Err(AnswerError::Invalid(_))` if the normalized
    /// answer fails validation. In the invalid case, the engine's state is
    /// left unchanged.
    pub fn answer_question(&mut self, question_id: &QuestionId, raw: RawAnswer) -> Result<(), AnswerError> {
        let Some(question) = self.registry.iter().find(|q| &q.id == question_id) else {
            return Err(AnswerError::UnknownQuestion);
        };

        let choices: Vec<String> = question
            .options
            .iter()
            .filter_map(|option| match &option.value {
                Value::Text(text) => Some(text.clone()),
                Value::Bool(_) => None,
            })
            .collect();

        let normalized = question.answer_kind.normalize(&raw, &choices);

        let result = validator::validate(question, &normalized);
        if !result.valid {
            return Err(AnswerError::Invalid(result));
        }

        self.state = self.state.with_answer(question.manifest_field.clone(), normalized);
        Ok(())
    }

    /// Returns `true` if every currently available question has been
    /// answered.
    ///
    /// This does not itself guarantee `build_manifest` will succeed —
    /// availability can change as `SemanticState` grows (a newly-answered
    /// question can make further questions available), and the manifest
    /// mapping may still fail for reasons `ManifestBuilder` alone knows
    /// about (see `ManifestBuildError`).
    pub fn is_complete(&self) -> bool {
        self.available_questions()
            .into_iter()
            .all(|availability| {
                !availability.available || self.state.is_present(&availability.question.manifest_field)
            })
    }

    /// Builds a `ProjectManifest` from the engine's current
    /// `SemanticState`.
    ///
    /// This is a thin orchestration call: all mapping logic lives in
    /// `ManifestBuilder::build`, which the engine does not duplicate. The
    /// engine does not require `is_complete()` to be `true` before calling
    /// this — an incomplete state simply surfaces as a
    /// `ManifestBuildError::MissingField` (or similar) from the builder,
    /// giving the caller one consistent error path rather than two.
    pub fn build_manifest(&self) -> Result<ProjectManifest, ManifestBuildError> {
        ManifestBuilder::build(&self.state)
    }
}

impl Default for ChaosEngine {
    fn default() -> Self {
        Self::new()
    }
}