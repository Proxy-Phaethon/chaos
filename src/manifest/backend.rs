//! Defines `BackendManifest`, the semantic representation of a Chaos
//! project's backend.
//!
//! This module contains data only: it describes what a backend *is*, not
//! how it is validated or generated. Those responsibilities belong to other
//! modules. `Database` is an optional child of `BackendManifest`.

use super::database::DatabaseManifest;

/// The language a backend is written in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendLanguage {
    Python,
    Go,
    Rust,
    NodeJs,
    Php,
    Java,
    CSharp,
}

/// The backend framework in use.
///
/// Which variants are valid for a given `BackendLanguage` is not encoded or
/// enforced here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendFramework {
    // Python
    Django,
    FastApi,
    Flask,

    // Go
    Gin,
    Echo,
    Fiber,

    // Rust
    Axum,
    ActixWeb,
    Rocket,

    // Node.js
    Express,
    Fastify,
    NestJs,

    // PHP
    Laravel,
    Symfony,

    // Java
    SpringBoot,

    // C#
    AspNetCore,
    // TODO: additional backend frameworks are not yet specified in the
    // architecture.
}

/// The authentication strategy used by the backend, if any.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Authentication {
    None,
    Jwt,
    Sessions,
    OAuth,
}

/// The API style exposed by the backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiStyle {
    Rest,
    GraphQl,
}

/// The semantic representation of a Chaos project's backend.
///
/// `BackendManifest` owns the properties that fully describe a backend's
/// configuration, including an optional `DatabaseManifest`. It performs no
/// validation, generation, or serialization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendManifest {
    pub language: BackendLanguage,
    pub framework: BackendFramework,
    pub database: Option<DatabaseManifest>,
    pub authentication: Authentication,
    pub api_style: ApiStyle,
    // TODO: additional backend properties are not yet specified in the
    // architecture.
}

impl BackendManifest {
    /// Creates a new `BackendManifest` from its constituent properties.
    pub fn new(
        language: BackendLanguage,
        framework: BackendFramework,
        database: Option<DatabaseManifest>,
        authentication: Authentication,
        api_style: ApiStyle,
    ) -> Self {
        Self {
            language,
            framework,
            database,
            authentication,
            api_style,
        }
    }
}