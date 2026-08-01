//! Executes a completed `GenerationPlan` against the filesystem.
//!
//! `FilesystemExecutor` is the only component of the Generation subsystem
//! that touches the filesystem. It performs the operations described by a
//! `GenerationPlan` exactly as given, in the exact order they appear, and
//! makes no decisions of its own about *what* to generate — only *how* to
//! carry out operations it has already been handed. It does not resolve
//! dependencies, validate anything, select templates, or know about CLI
//! code or manifests.

use std::fs;
use std::path::{Path, PathBuf};

use super::context::{GenerationContext, OverwritePolicy};
use super::error::GenerationError;
use super::generator::PlanExecutor;
use super::plan::{GenerationOperation, GenerationPlan};

/// Resolves a named template to its rendered content.
///
/// `FilesystemExecutor` does not select or render templates itself — a
/// `CopyTemplate` operation only carries a template's *name*, so the
/// executor depends on a `TemplateResolver` to turn that name into actual
/// content. This keeps template logic out of this module entirely.
// TODO: no concrete `TemplateResolver` implementation exists yet. One
// should be added once the template subsystem is designed, and supplied
// to `FilesystemExecutor::new` in place of relying on a stub.
pub trait TemplateResolver {
    /// Resolves `template_name` to its content, or fails if it cannot be
    /// found or rendered.
    fn resolve(&self, template_name: &str) -> Result<String, GenerationError>;
}

/// Executes a `GenerationPlan` against the real filesystem.
///
/// `FilesystemExecutor` holds a `TemplateResolver` so that `CopyTemplate`
/// operations can be carried out without this module knowing anything
/// about how templates are found or rendered. It respects the
/// `GenerationContext`'s `GenerationOptions` (dry-run and overwrite
/// policy) rather than deciding generation policy itself.
pub struct FilesystemExecutor<T: TemplateResolver> {
    template_resolver: T,
}

impl<T: TemplateResolver> FilesystemExecutor<T> {
    /// Creates a new `FilesystemExecutor` backed by the given template
    /// resolver.
    pub fn new(template_resolver: T) -> Self {
        Self { template_resolver }
    }

    /// Resolves an operation's path against the context's output
    /// directory.
    fn resolve_path(&self, context: &GenerationContext, path: &Path) -> PathBuf {
        context.output_directory().join(path)
    }

    /// Creates `path`, including any missing parent directories.
    fn create_directory(&self, context: &GenerationContext, path: &Path) -> Result<(), GenerationError> {
        let full_path = self.resolve_path(context, path);
        if context.options().dry_run {
            return Ok(());
        }
        fs::create_dir_all(&full_path).map_err(|error| GenerationError::FilesystemFailure {
            path: full_path.display().to_string(),
            reason: error.to_string(),
        })
    }

    /// Writes `content` to `path`, honoring the context's overwrite
    /// policy if the file already exists.
    fn write_file(&self, context: &GenerationContext, path: &Path, content: &str) -> Result<(), GenerationError> {
        let full_path = self.resolve_path(context, path);

        if full_path.exists() && context.options().overwrite_policy == OverwritePolicy::Refuse {
            return Err(GenerationError::ExistingProjectConflict {
                path: full_path.display().to_string(),
            });
        }

        if context.options().dry_run {
            return Ok(());
        }

        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).map_err(|error| GenerationError::FilesystemFailure {
                path: parent.display().to_string(),
                reason: error.to_string(),
            })?;
        }

        fs::write(&full_path, content).map_err(|error| GenerationError::FilesystemFailure {
            path: full_path.display().to_string(),
            reason: error.to_string(),
        })
    }

    /// Creates an empty file at `path`, honoring the context's overwrite
    /// policy if the file already exists.
    fn create_file(&self, context: &GenerationContext, path: &Path) -> Result<(), GenerationError> {
        self.write_file(context, path, "")
    }

    /// Resolves `template_name` via `self.template_resolver` and writes
    /// its content to `destination`, honoring the context's overwrite
    /// policy.
    fn copy_template(
        &self,
        context: &GenerationContext,
        template_name: &str,
        destination: &Path,
    ) -> Result<(), GenerationError> {
        let content = self.template_resolver.resolve(template_name)?;
        self.write_file(context, destination, &content)
    }

    /// Executes a single operation.
    fn execute_operation(
        &self,
        context: &GenerationContext,
        operation: &GenerationOperation,
    ) -> Result<(), GenerationError> {
        match operation {
            GenerationOperation::CreateDirectory { path } => self.create_directory(context, path),
            GenerationOperation::CreateFile { path } => self.create_file(context, path),
            GenerationOperation::WriteFile { path, content } => self.write_file(context, path, content),
            GenerationOperation::CopyTemplate {
                template_name,
                destination,
            } => self.copy_template(context, template_name, destination),
            // TODO: Delete
            // TODO: Rename
            // TODO: Patch
            // TODO: Merge
            // TODO: Conditional
        }
    }
}

impl<T: TemplateResolver> PlanExecutor for FilesystemExecutor<T> {
    /// Executes every operation in `plan` against the filesystem, in the
    /// exact order they appear.
    ///
    /// Stops at, and returns, the first error encountered. Operations
    /// already applied before that point are not rolled back — see the
    /// rollback TODO below.
    // TODO: rollback of already-applied operations on failure.
    // TODO: atomic generation (e.g. write to a staging location and move
    // into place only on full success).
    // TODO: file patching, once a `Patch` operation exists.
    // TODO: permission handling (e.g. preserving or setting file modes).
    // TODO: symbolic link support.
    // TODO: progress reporting as operations are executed.
    // TODO: parallel execution where operation order allows it safely
    // (e.g. independent file writes within an already-created directory).
    fn execute(&self, plan: &GenerationPlan, context: &GenerationContext) -> Result<(), GenerationError> {
        for operation in plan.operations() {
            self.execute_operation(context, operation)?;
        }
        Ok(())
    }
}