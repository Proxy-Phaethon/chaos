//! Defines `DatabaseManifest`, the semantic representation of a Chaos
//! project's database configuration.
//!
//! The Database entity is owned by the Backend entity. This module contains
//! data only: it describes what a database configuration *is*, not how it
//! is validated or generated. Compatibility between a given `Orm` and the
//! backend's language/framework is not enforced here.

/// The database engine in use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseEngine {
    PostgreSql,
    MySql,
    Sqlite,
    MongoDb,
}

/// The ORM, or equivalent database access layer, in use.
///
/// Variants are grouped by the backend language they are associated with in
/// the Version 1 architecture. Which variants are valid for a given backend
/// language or framework is not encoded or enforced here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orm {
    // Python
    DjangoOrm,
    SqlAlchemy,
    SqlModel,
    TortoiseOrm,

    // Go
    Gorm,
    Bun,
    Ent,
    Sqlc,

    // Rust
    SqlX,
    Diesel,
    SeaOrm,

    // Node.js
    Prisma,
    Drizzle,
    TypeOrm,
    Sequelize,
    Mongoose,

    // PHP
    Eloquent,
    Doctrine,

    // Java
    Hibernate,
    EclipseLink,

    // C#
    EntityFrameworkCore,
    Dapper,
    // TODO: additional ORM/database layer options are not yet specified in
    // the architecture.
}

/// The semantic representation of a Chaos project's database configuration.
///
/// `DatabaseManifest` owns the properties that fully describe a database's
/// configuration. It performs no validation, generation, or serialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DatabaseManifest {
    pub engine: DatabaseEngine,
    pub orm: Orm,
    // TODO: additional database properties (e.g. connection pooling,
    // migrations strategy) are not yet specified in the architecture.
}

impl DatabaseManifest {
    /// Creates a new `DatabaseManifest` from its constituent properties.
    pub fn new(engine: DatabaseEngine, orm: Orm) -> Self {
        Self { engine, orm }
    }
}