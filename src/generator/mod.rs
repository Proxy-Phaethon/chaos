//! Public entry point for the `generator` package.
//!
//! Re-exports the primary types from each submodule so other parts of the
//! project can import from `crate::generator` directly, without needing to
//! know the internal module layout. This file contains no business logic,
//! no filesystem operations, no manifest construction, no project
//! generation, and no CLI code — it is a thin public interface only.

mod context;
mod error;
mod filesystem;
mod generator;
mod plan;
mod template;

pub use context::{GenerationContext, GenerationOptions, OverwritePolicy};
pub use error::GenerationError;
pub use filesystem::{FilesystemExecutor, TemplateResolver};
pub use generator::{Generator, PlanExecutor, Planner};
pub use plan::{GenerationOperation, GenerationPlan};
pub use template::{Template, TemplateId, TemplateMetadata};