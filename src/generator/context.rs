//! Defines the immutable generation context used throughout the Generation
//! subsystem.
//!
//! A `GenerationContext` groups together all information required for a
//! single generation run, so generator components don't need to pass
//! numerous unrelated parameters between one another. This module is
//! purely semantic: it contains no filesystem operations, no plan
//! construction, no template selection, no validation, and no CLI code.

use std::path::PathBuf;

use crate::manifest::ProjectManifest;

/// The policy to apply when generation would write into a location that
/// already contains files.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverwritePolicy {
    /// Refuse to write over any existing file.
    Refuse,
    /// Overwrite existing files without asking.
    Overwrite,
    // TODO: a `Prompt` variant (ask per-conflict) belongs to a CLI layer,
    // not here — this module stays free of CLI code. If a non-interactive
    // equivalent is needed (e.g. "overwrite only files the generator
    // itself previously created"), it can be added as a new variant.
}

/// Version 1 generation settings.
///
/// `GenerationOptions` is immutable once constructed; a new run with
/// different settings is expressed as a new `GenerationOptions` value
/// rather than mutating an existing one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerationOptions {
    /// How to handle writes into a location that already contains files.
    pub overwrite_policy: OverwritePolicy,

    /// If `true`, generation should describe what it would do without
    /// performing any filesystem operations. Interpreting this flag is the
    /// responsibility of whatever module executes a `GenerationPlan`; this
    /// struct only carries the setting.
    pub dry_run: bool,
    // TODO: additional Version 1 settings, if any turn out to be needed
    // (e.g. verbosity), belong here.
}

impl GenerationOptions {
    /// Creates new generation options with the given overwrite policy and
    /// dry-run setting.
    pub fn new(overwrite_policy: OverwritePolicy, dry_run: bool) -> Self {
        Self {
            overwrite_policy,
            dry_run,
        }
    }

    /// Creates the Version 1 default options: refuse to overwrite existing
    /// files, and actually perform generation (not a dry run).
    pub fn defaults() -> Self {
        Self {
            overwrite_policy: OverwritePolicy::Refuse,
            dry_run: false,
        }
    }
}

impl Default for GenerationOptions {
    fn default() -> Self {
        Self::defaults()
    }
}

/// Groups together everything a single generation run needs to know.
///
/// `GenerationContext` is immutable after construction — a generation run
/// that needs different settings or a different manifest is expressed as a
/// new context, not a mutation of an existing one. This struct contains no
/// business logic; it is a pure data container.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerationContext {
    /// The validated manifest describing the project being generated.
    manifest: ProjectManifest,

    /// The directory generation should write into.
    output_directory: PathBuf,

    /// Settings controlling how generation behaves.
    options: GenerationOptions,
    // TODO: progress reporting (e.g. a sink for step-by-step status)
    // TODO: plugin context (data/hooks for future plugin support)
    // TODO: logging (a structured logger or log sink)
    // TODO: user configuration (persistent, cross-project user preferences)
    // TODO: build profiles (e.g. development vs. production generation)
    // TODO: target platforms (once non-web targets exist)
    // TODO: incremental generation (state from a previous run, for `chaos edit`)
}

impl GenerationContext {
    /// Creates a new generation context from a manifest, an output
    /// directory, and generation options.
    pub fn new(
        manifest: ProjectManifest,
        output_directory: impl Into<PathBuf>,
        options: GenerationOptions,
    ) -> Self {
        Self {
            manifest,
            output_directory: output_directory.into(),
            options,
        }
    }

    /// Creates a new generation context using the Version 1 default
    /// generation options.
    pub fn with_defaults(manifest: ProjectManifest, output_directory: impl Into<PathBuf>) -> Self {
        Self::new(manifest, output_directory, GenerationOptions::defaults())
    }

    /// Returns the manifest being generated.
    pub fn manifest(&self) -> &ProjectManifest {
        &self.manifest
    }

    /// Returns the directory generation should write into.
    pub fn output_directory(&self) -> &PathBuf {
        &self.output_directory
    }

    /// Returns the generation options in effect for this context.
    pub fn options(&self) -> &GenerationOptions {
        &self.options
    }
}