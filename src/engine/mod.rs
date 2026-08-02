//! Public entry point for the Chaos Engine.
//!
//! The engine exposes project initialization as a single semantic system,
//! coordinating the question registry, dependency resolver, answer
//! normalizer, and answer validator behind one type: [`ChaosEngine`]. It
//! contains no CLI code. Once every question has been answered, the engine
//! delegates construction of the semantic `ProjectManifest` to
//! `ManifestBuilder`.

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

/// Describes why `ChaosEngine::answer_question` failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnswerError {
    /// No question exists with the supplied id.
    UnknownQuestion,

    /// The normalized answer failed validation.
    Invalid(ValidationResult),
}

/// The semantic initialization engine.
///
/// `ChaosEngine` owns the question registry and the accumulated semantic
/// state. It coordinates normalization, validation, dependency
/// resolution, and manifest construction, but performs no I/O or
/// generation itself.
pub struct ChaosEngine {
    registry: Vec<Question>,
    state: SemanticState,
}

impl ChaosEngine {
    /// Creates a new engine.
    pub fn new() -> Self {
        Self {
            registry: registry::registry(),
            state: SemanticState::new(),
        }
    }

    /// Returns the engine's question registry.
    pub fn registry(&self) -> &[Question] {
        &self.registry
    }

    /// Returns the current semantic state.
    pub fn state(&self) -> &SemanticState {
        &self.state
    }

    /// Returns the availability of every registered question.
    pub fn available_questions(&self) -> Vec<QuestionAvailability<'_>> {
        resolver::resolve_availability(&self.registry, &self.state)
    }

    /// Returns the next unanswered available question.
    pub fn next_question(&self) -> Option<&Question> {
        self.available_questions()
            .into_iter()
            .find(|availability| {
                availability.available
                    && self
                        .state
                        .is_absent(&availability.question.manifest_field)
            })
            .map(|availability| availability.question)
    }

    /// Returns the current active question.
    pub fn current_question(&self) -> Option<&Question> {
        self.next_question()
    }

    /// Records an answer for a question.
    pub fn answer_question(
        &mut self,
        question_id: &QuestionId,
        raw: RawAnswer,
    ) -> Result<(), AnswerError> {
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

        let validation = validator::validate(question, &normalized);

        if !validation.valid {
            return Err(AnswerError::Invalid(validation));
        }

        self.state = self
            .state
            .with_answer(question.manifest_field.clone(), normalized);

        Ok(())
    }

    /// Returns true if every available question has been answered.
    pub fn is_complete(&self) -> bool {
        self.available_questions().into_iter().all(|availability| {
            !availability.available
                || self
                    .state
                    .is_present(&availability.question.manifest_field)
        })
    }

    /// Builds a semantic project manifest.
    pub fn build_manifest(
        &self,
    ) -> Result<ProjectManifest, ManifestBuildError> {
        ManifestBuilder::build(&self.state)
    }
}

impl Default for ChaosEngine {
    fn default() -> Self {
        Self::new()
    }
}