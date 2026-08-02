//! Planning stage of the Generation subsystem.
//!
//! A `Planner` converts a validated `ProjectManifest` into a
//! `GenerationPlan`: an implementation-independent description of what
//! should be created. It decides *what* is needed by inspecting the
//! manifest's frontend, backend, database, and tooling — but it never
//! executes anything itself. It does not write files, does not create
//! directories, and does not render template content; it only describes
//! operations for a `FilesystemExecutor` (or equivalent) to carry out
//! later.

use crate::manifest::ProjectManifest;

use super::error::GenerationError;
use super::plan::GenerationPlan;

/// Builds a `GenerationPlan` from a `ProjectManifest`.
///
/// Implementations decide *what* operations a project needs by examining
/// the manifest's `frontend`, `backend`, `database` (nested within
/// `backend`), and `tooling` — but they do not execute those operations.
/// This keeps planning (a semantic decision) fully separate from execution
/// (a filesystem concern), which lives elsewhere in the Generation
/// subsystem.
pub trait Planner {
    /// Builds a plan describing everything required to generate the
    /// project described by `manifest`.
    ///
    /// Returns `Err` if the manifest cannot be planned for — for example,
    /// because it describes a combination the planner has no template or
    /// strategy for yet.
    fn plan(&self, manifest: &ProjectManifest) -> Result<GenerationPlan, GenerationError>;
}

/// A placeholder `Planner` implementation.
///
/// `DefaultPlanner` currently produces an empty `GenerationPlan` for every
/// manifest, regardless of its contents. It exists so the rest of the
/// Generation subsystem (in particular `Generator`) has a concrete
/// `Planner` to be constructed with today, ahead of real template
/// discovery being implemented.
// TODO: discover applicable `Template`s (see `generator::template`) for
// the manifest's frontend framework/meta-framework, backend
// language/framework, database engine/ORM, and tooling choices.
// TODO: ask each applicable template to contribute to the plan, in an
// order that respects template dependencies once those exist (see the
// TODOs on `TemplateMetadata`).
// TODO: append shared, manifest-independent operations (e.g. a root
// README, a `.chaos` manifest file) once their generation is designed.
// TODO: surface `GenerationError::InvalidManifest` for manifests this
// planner recognizes as unplannable, rather than silently producing an
// empty plan for them as it does today.
pub struct DefaultPlanner;

impl DefaultPlanner {
    /// Creates a new `DefaultPlanner`.
    pub fn new() -> Self {
        Self
    }
}

impl Default for DefaultPlanner {
    fn default() -> Self {
        Self::new()
    }
}

impl Planner for DefaultPlanner {
    /// Returns an empty `GenerationPlan`, regardless of `manifest`'s
    /// contents.
    ///
    /// This is a placeholder — see the TODOs on `DefaultPlanner` for what
    /// real planning will eventually involve.
    fn plan(&self, _manifest: &ProjectManifest) -> Result<GenerationPlan, GenerationError> {
        Ok(GenerationPlan::new())
    }
}