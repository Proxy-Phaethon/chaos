//! Defines the semantic template abstraction used by the Generation
//! subsystem.
//!
//! A template is a reusable generation component capable of contributing
//! operations to a `GenerationPlan` for a project it recognizes as
//! applicable. This module defines only the common interface every
//! template implements — it contains no framework-specific templates
//! (React, Django, Axum, or otherwise), no filesystem operations, no
//! manifest construction, and no CLI code. The generator discovers
//! applicable templates and asks each one to extend a plan; how templates
//! are discovered is not this module's concern.

use crate::manifest::ProjectManifest;

use super::error::GenerationError;
use super::plan::GenerationPlan;

/// A unique identifier for a `Template`.
///
/// Kept as a string-based newtype rather than an enum, since the set of
/// templates is expected to grow — including, eventually, templates not
/// known at compile time (see the plugin/user-defined template TODOs
/// below).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TemplateId(pub String);

impl TemplateId {
    /// Creates a new `TemplateId` from anything convertible to a `String`.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

/// Descriptive information about a `Template`.
///
/// `TemplateMetadata` is data only — it exists so callers (e.g. a future
/// `chaos edit`, or diagnostic output) can describe a template without
/// needing to invoke it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateMetadata {
    pub id: TemplateId,
    pub name: String,
    pub description: Option<String>,
    // TODO: template dependencies (other TemplateIds this template
    // requires to have already run, e.g. a framework template depending
    // on a language template).
    // TODO: template versioning (so a manifest can pin, or the generator
    // can report, which version of a template produced a project).
    // TODO: template priority (ordering hints for when multiple
    // applicable templates could conflict or must run in a particular
    // sequence).
}

impl TemplateMetadata {
    /// Creates new template metadata with no description.
    pub fn new(id: TemplateId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            description: None,
        }
    }

    /// Creates new template metadata with an attached description.
    pub fn with_description(id: TemplateId, name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            description: Some(description.into()),
        }
    }
}

/// A reusable generation component.
///
/// A `Template` identifies and describes itself, determines whether it is
/// relevant to a given `ProjectManifest`, and — if so — contributes
/// operations to a `GenerationPlan`. It never performs filesystem
/// operations directly; contributing to the plan is the full extent of
/// its responsibility. Executing the resulting plan belongs to the
/// filesystem layer.
pub trait Template {
    /// Returns this template's metadata.
    fn metadata(&self) -> &TemplateMetadata;

    /// Returns this template's identifier.
    ///
    /// A default implementation is provided in terms of `metadata`, but
    /// implementors may override it if identity can be determined more
    /// cheaply than full metadata.
    fn id(&self) -> &TemplateId {
        &self.metadata().id
    }

    /// Determines whether this template applies to `manifest`.
    ///
    /// Applicability is a pure function of the manifest's contents (e.g. "the
    /// manifest's frontend framework is React"). This method must not have
    /// side effects.
    fn applies_to(&self, manifest: &ProjectManifest) -> bool;

    /// Contributes this template's operations to `plan`, given `manifest`.
    ///
    /// Called only when `applies_to(manifest)` is `true`. Implementations
    /// append to `plan` — they do not execute anything themselves, and
    /// they do not construct or modify `manifest`.
    fn contribute(&self, manifest: &ProjectManifest, plan: &mut GenerationPlan) -> Result<(), GenerationError>;
    // TODO: conditional templates — templates whose contribution depends
    // not just on `applies_to` but on finer-grained conditions evaluated
    // during contribution (e.g. sub-features of a framework). May be
    // expressible in terms of this trait already, but isn't yet exercised
    // by any real template.
}

// TODO: plugin templates — templates loaded from outside this crate at
// runtime, once a plugin system exists.
// TODO: user-defined templates — templates authored by end users (e.g. via
// `chaos edit`), distinct from templates shipped with Chaos itself.