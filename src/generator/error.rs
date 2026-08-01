//! Defines the errors produced by the Generation subsystem.
//!
//! This module is purely declarative: it describes the ways project
//! generation can fail, without performing any generation, filesystem
//! access, or CLI interaction itself. Those responsibilities belong to
//! other modules within `generator`.

use std::fmt;

/// A failure that occurred during project generation.
///
/// `GenerationError` covers only failures within the generation pipeline
/// itself (turning a `ProjectManifest` into files on disk) — it does not
/// cover earlier stages such as prompting, normalization, validation, or
/// manifest construction, each of which has its own error type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenerationError {
    /// The `ProjectManifest` supplied to the generator was not valid for
    /// generation (e.g. structurally incomplete, or describing a
    /// combination the generator does not know how to produce).
    InvalidManifest { reason: String },

    /// A template required to generate part of the project could not be
    /// found.
    MissingTemplate { name: String },

    /// A filesystem operation (creating a directory, writing a file, and
    /// so on) failed during generation.
    FilesystemFailure { path: String, reason: String },

    /// Generation would write into a location that already contains a
    /// conflicting project or files, and the generator was not instructed
    /// to overwrite them.
    ExistingProjectConflict { path: String },

    /// The generation plan derived from the manifest was invalid (e.g.
    /// internally inconsistent, or missing a step required to produce a
    /// usable project).
    InvalidGenerationPlan { reason: String },

    /// A template was found, but failed while being rendered (e.g. a
    /// malformed template, or missing data it required).
    TemplateRenderingFailure { name: String, reason: String },

    /// Installing a project's dependencies (via its package manager or
    /// toolchain) failed.
    DependencyInstallationFailure { package: String, reason: String },

    /// A generation failure occurred that does not fit any of the other
    /// variants.
    UnknownGenerationFailure { reason: String },
    // TODO: additional variants (e.g. version incompatibility between a
    // chosen framework and toolchain, network failures when fetching
    // templates) may be introduced as the generator's capabilities grow.
}

impl fmt::Display for GenerationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GenerationError::InvalidManifest { reason } => {
                write!(f, "invalid manifest: {}", reason)
            }
            GenerationError::MissingTemplate { name } => {
                write!(f, "missing template: {}", name)
            }
            GenerationError::FilesystemFailure { path, reason } => {
                write!(f, "filesystem failure at '{}': {}", path, reason)
            }
            GenerationError::ExistingProjectConflict { path } => {
                write!(f, "existing project conflict at '{}'", path)
            }
            GenerationError::InvalidGenerationPlan { reason } => {
                write!(f, "invalid generation plan: {}", reason)
            }
            GenerationError::TemplateRenderingFailure { name, reason } => {
                write!(f, "failed to render template '{}': {}", name, reason)
            }
            GenerationError::DependencyInstallationFailure { package, reason } => {
                write!(f, "failed to install dependency '{}': {}", package, reason)
            }
            GenerationError::UnknownGenerationFailure { reason } => {
                write!(f, "generation failed: {}", reason)
            }
        }
    }
}

impl std::error::Error for GenerationError {}