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
//! receives a `GenerationContext`, asks a `Planner` to turn its manifest
//! into a `GenerationPlan`, and hands that plan to a `FilesystemExecutor`
//! for execution. It does not write templates, does not manually create
//! files, and does not inspect frontend/backend choices itself — those
//! decisions belong to the `Planner` (and, transitively, to whatever
//! `Template`s the planner consults).

use crate::manifest::ProjectManifest;

use super::context::GenerationContext;
use super::error::GenerationError;
use super::filesystem::{FilesystemExecutor, TemplateResolver};
use super::generator_traits::PlanExecutor;
use super::plan::GenerationPlan;

/// Builds a `GenerationPlan` from a `ProjectManifest`.
///
/// Implementations decide *what* operations a project needs; they do not
/// execute those operations. Kept as a trait so `Generator` can be
/// coordinated without depending on any particular planning strategy.
///
/// A `Planner` may inspect the manifest's frontend/backend/database/tooling
/// selections, but only to decide which templates or operations apply —
/// not to perform any generation itself.
// TODO: implement a concrete `Planner` (e.g. in a `planner` module) that
// inspects the manifest and builds a real `GenerationPlan`, likely by
// discovering applicable `Template`s and asking each to contribute. None
// exists yet.
pub trait Planner {
    /// Builds a plan describing everything required to generate the
    /// project described by `manifest`.
    fn plan(&self, manifest: &ProjectManifest) -> Result<GenerationPlan, GenerationError>;
}

/// Coordinates project generation from a validated `ProjectManifest`.
///
/// `Generator` holds a `Planner` and a `FilesystemExecutor` and does
/// nothing but pass data between them, in the order described above. It
/// never mutates the `ProjectManifest` it is given, and it performs no
/// filesystem operations directly — those are entirely the
/// `FilesystemExecutor`'s responsibility.
pub struct Generator<P: Planner, T: TemplateResolver> {
    planner: P,
    executor: FilesystemExecutor<T>,
}

impl<P: Planner, T: TemplateResolver> Generator<P, T> {
    /// Creates a new `Generator` from a planner and a filesystem executor.
    pub fn new(planner: P, executor: FilesystemExecutor<T>) -> Self {
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