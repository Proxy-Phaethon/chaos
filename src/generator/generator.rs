//! Orchestration layer of the Generation subsystem.
//!
//! `Generator` coordinates project generation from a validated
//! `ProjectManifest`, but performs no low-level generation work itself: it
//! neither builds a `GenerationPlan`'s contents nor executes filesystem
//! operations directly. Both responsibilities are delegated through the
//! `Planner` and `PlanExecutor` traits, whose concrete implementations
//! live in their own modules (e.g. a future `planner` module and a future
//! `filesystem` module) and are supplied to the `Generator` rather than
//! hard-coded here.

use crate::manifest::ProjectManifest;

use super::context::GenerationContext;
use super::error::GenerationError;
use super::plan::GenerationPlan;

/// Builds a `GenerationPlan` from a `ProjectManifest`.
///
/// Implementations decide *what* operations a project needs; they do not
/// execute those operations. Kept as a trait so `Generator` can be
/// coordinated without depending on any particular planning strategy.
// TODO: implement a concrete `Planner` (e.g. in a `planner` module) that
// inspects the manifest's frontend/backend/database/tooling and builds a
// real `GenerationPlan`. None exists yet.
pub trait Planner {
    /// Builds a plan describing everything required to generate the
    /// project described by `manifest`.
    fn plan(&self, manifest: &ProjectManifest) -> Result<GenerationPlan, GenerationError>;
}

/// Executes a `GenerationPlan` against the filesystem.
///
/// Implementations perform the actual filesystem work; this trait exists
/// so `Generator` can be coordinated without depending on any particular
/// filesystem strategy (or, in dry-run mode, no filesystem access at all).
// TODO: implement a concrete `PlanExecutor` (e.g. in a `filesystem`
// module) that performs real filesystem operations for each
// `GenerationOperation`, honoring `GenerationContext::options`. None
// exists yet.
pub trait PlanExecutor {
    /// Executes `plan` within the given `context`.
    fn execute(&self, plan: &GenerationPlan, context: &GenerationContext) -> Result<(), GenerationError>;
}

/// Coordinates project generation from a validated `ProjectManifest`.
///
/// `Generator` owns no generation logic of its own — it holds a `Planner`
/// and a `PlanExecutor` and orchestrates calls between them. It never
/// mutates the `ProjectManifest` it is given.
pub struct Generator<P: Planner, E: PlanExecutor> {
    planner: P,
    executor: E,
}

impl<P: Planner, E: PlanExecutor> Generator<P, E> {
    /// Creates a new `Generator` from a planner and an executor.
    pub fn new(planner: P, executor: E) -> Self {
        Self { planner, executor }
    }

    /// Generates a project for the given `GenerationContext`.
    ///
    /// Pipeline:
    ///
    /// 1. Receive `context`.
    /// 2. Build a `GenerationPlan` from `context.manifest()` via `self.planner`.
    /// 3. Execute the plan via `self.executor`.
    /// 4. Return success, or the first `GenerationError` encountered.
    // TODO: template selection — the planner will eventually need to
    // choose templates as part of building the plan; not yet wired in.
    // TODO: dependency installation — installing a generated project's
    // dependencies is a distinct pipeline stage, not yet represented.
    // TODO: post-generation hooks (e.g. running formatters) after a
    // successful execute.
    // TODO: plugin execution, once a plugin system exists.
    // TODO: progress reporting throughout the pipeline.
    // TODO: rollback of partially-applied operations if execution fails
    // partway through.
    pub fn generate(&self, context: &GenerationContext) -> Result<(), GenerationError> {
        let plan = self.planner.plan(context.manifest())?;
        self.executor.execute(&plan, context)?;
        Ok(())
    }
}