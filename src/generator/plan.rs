//! Defines the semantic generation plan used by the Generation subsystem.
//!
//! A `GenerationPlan` is an implementation-independent description of
//! everything that should be created for a project. It describes intended
//! filesystem operations only — it does not perform any filesystem access,
//! render any template content, or know anything about CLI code. Executing
//! a plan is the responsibility of another module within `generator`.

use std::path::PathBuf;

/// A single semantic filesystem action to be performed during generation.
///
/// `GenerationOperation` describes *what* should happen, not *how* — it
/// carries only the information needed to describe the action, not to
/// execute it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenerationOperation {
    /// Create a directory at `path`, including any missing parent
    /// directories.
    CreateDirectory { path: PathBuf },

    /// Create a new, empty file at `path`.
    ///
    /// Distinct from `WriteFile`: this operation carries no content,
    /// describing only that a file should exist.
    CreateFile { path: PathBuf },

    /// Copy a named template to `destination`, without describing how the
    /// template is rendered or where its source lives — that is the
    /// responsibility of whatever module resolves templates by name.
    CopyTemplate {
        template_name: String,
        destination: PathBuf,
    },

    /// Write `content` to `path`, creating the file if it does not exist.
    WriteFile { path: PathBuf, content: String },
    // TODO: Delete { path: PathBuf }
    // TODO: Rename { from: PathBuf, to: PathBuf }
    // TODO: Patch { path: PathBuf, patch: /* patch representation */ }
    // TODO: Merge { path: PathBuf, content: String, strategy: /* merge strategy */ }
    // TODO: Conditional { condition: /* condition representation */, operation: Box<GenerationOperation> }
}

/// An ordered, implementation-independent description of everything that
/// should be created for a project.
///
/// `GenerationPlan` owns its operations in the exact order they were
/// added — that order is meaningful (e.g. a directory must be created
/// before a file is written into it) and is preserved rather than
/// reordered or deduplicated.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GenerationPlan {
    operations: Vec<GenerationOperation>,
}

impl GenerationPlan {
    /// Creates a new, empty generation plan.
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    /// Appends `operation` to the end of the plan.
    pub fn push(&mut self, operation: GenerationOperation) {
        self.operations.push(operation);
    }

    /// Appends a `CreateDirectory` operation for `path`.
    pub fn create_directory(&mut self, path: impl Into<PathBuf>) {
        self.push(GenerationOperation::CreateDirectory { path: path.into() });
    }

    /// Appends a `CreateFile` operation for `path`.
    pub fn create_file(&mut self, path: impl Into<PathBuf>) {
        self.push(GenerationOperation::CreateFile { path: path.into() });
    }

    /// Appends a `CopyTemplate` operation for the named template.
    pub fn copy_template(&mut self, template_name: impl Into<String>, destination: impl Into<PathBuf>) {
        self.push(GenerationOperation::CopyTemplate {
            template_name: template_name.into(),
            destination: destination.into(),
        });
    }

    /// Appends a `WriteFile` operation for `path` with the given content.
    pub fn write_file(&mut self, path: impl Into<PathBuf>, content: impl Into<String>) {
        self.push(GenerationOperation::WriteFile {
            path: path.into(),
            content: content.into(),
        });
    }

    /// Returns the plan's operations, in the exact order they were added.
    pub fn operations(&self) -> &[GenerationOperation] {
        &self.operations
    }

    /// Returns `true` if the plan contains no operations.
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Returns the number of operations in the plan.
    pub fn len(&self) -> usize {
        self.operations.len()
    }
}