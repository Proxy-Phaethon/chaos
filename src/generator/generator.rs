//! Orchestration layer of the Generation subsystem.
//!
//! `Generator` is the single coordinator of the pipeline:
//!
//! ```text
//! GenerationContext
//!         │
//!         ▼
//!     Generator
//!         │
//!         ▼
//!   GenerationPlan
//!         │
//!         ▼
//! FilesystemExecutor
//! ```
//!
//! `Generator` owns no semantic logic and no filesystem logic itself. It
//! receives a `GenerationContext`, asks a `Planner` (from
//! `super::planner`) to turn its manifest into a `GenerationPlan`, and
//! hands that plan to a `FilesystemExecutor` for execution. It does not
//! write templates, does not manually create files, and does not inspect
//! frontend/backend choices itself — those decisions belong to the
//! `Planner` (and, transitively, to whatever `Template`s the planner
//! consults).

use super::context::GenerationContext;
use super::error::GenerationError;
use super::plan::GenerationPlan;
use super::planner::Planner;

/// Executes a `GenerationPlan` against the filesystem.
///
/// Implementations perform the actual filesystem work; this trait exists
/// so `Generator` can be coordinated without depending on any particular
/// filesystem strategy (or, in dry-run mode, no filesystem access at all).
/// `FilesystemExecutor` (see `super::filesystem`) is the concrete
/// implementation `Generator` is used with.
pub trait PlanExecutor {
    /// Executes `plan` within the given `context`.
    fn execute(&self, plan: &GenerationPlan, context: &GenerationContext) -> Result<(), GenerationError>;
}

/// Coordinates project generation from a validated `ProjectManifest`.
///
/// `Generator` holds a `Planner` and a `PlanExecutor` and does nothing but
/// pass data between them, in the order described above. It never mutates
/// the `ProjectManifest` it is given, and it performs no filesystem
/// operations directly — those are entirely the `PlanExecutor`'s
/// responsibility. In practice, `E` is `FilesystemExecutor` (see
/// `super::filesystem`), which implements `PlanExecutor`.
pub struct Generator<P: Planner, E: PlanExecutor> {
    planner: P,
    executor: E,
}

impl<P: Planner, E: PlanExecutor> Generator<P, E> {
    /// Creates a new `Generator` from a planner and a plan executor.
    pub fn new(planner: P, executor: E) -> Self {
        Self { planner, executor }
    }

    /// Generates a project for the given `GenerationContext`.
    ///
    /// Pipeline:
    ///
    /// 1. Receive `context`.
    /// 2. Build a `GenerationPlan` from `context.manifest()` via `self.planner`.
    /// 3. Execute the plan via `self.executor` (a `FilesystemExecutor`).
    /// 4. Return success, or the first `GenerationError` encountered.
    // TODO: template selection — the planner will eventually need to
    // choose templates as part of building the plan; not yet wired in
    // (see the TODOs on `planner::DefaultPlanner`).
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