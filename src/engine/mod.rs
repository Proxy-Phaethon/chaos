//! Public entry point for the Chaos Engine.
//!
//! The engine exposes project initialization as a single semantic system,
//! coordinating the question registry, dependency resolver, and answer
//! normalizer behind one type: [`ChaosEngine`]. It contains no CLI code —
//! it never prints, reads stdin, or knows anything about terminals. Callers
//! (a future CLI layer) drive the engine through semantic operations only.

mod dependency;
mod normalizer;
mod question;
mod registry;
mod resolver;

pub use dependency::{Condition, Dependency, Field, Value};
pub use normalizer::{Normalizer, RawAnswer};
pub use question::{AnswerKind, Effect, Question, QuestionId, QuestionOption};
pub use resolver::{AvailabilityReason, QuestionAvailability, SemanticState};

/// The Chaos Engine: the semantic core of `chaos initialize`.
///
/// `ChaosEngine` owns the Version 1 question registry and the
/// `SemanticState` accumulated so far, and coordinates the resolver and
/// normalizer to answer questions like "what should be asked next" and
/// "what does this answer mean, semantically". It performs no I/O, no
/// filesystem access, and no project generation — those belong to other
/// modules that will consume the engine's output.
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
    /// identified by `question_id`, and records the resulting value in the
    /// engine's state under that question's `manifest_field`.
    ///
    /// Returns `true` if a matching question was found in the registry and
    /// its answer was recorded, or `false` if no question with that id
    /// exists.
    // TODO: this does not yet consult a Validator. Once one exists, the
    // normalized value should be validated before being committed to
    // `state`, and this method should return a result capable of
    // expressing validation failure rather than a bare `bool`.
    pub fn answer_question(&mut self, question_id: &QuestionId, raw: RawAnswer) -> bool {
        let Some(question) = self.registry.iter().find(|q| &q.id == question_id) else {
            return false;
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
        self.state = self.state.with_answer(question.manifest_field.clone(), normalized);
        true
    }
}

impl Default for ChaosEngine {
    fn default() -> Self {
        Self::new()
    }
}