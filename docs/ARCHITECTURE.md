# Chaos Architecture

**Version:** 0.1

---

# 1. Purpose

Chaos is a semantic software engineering engine.

Its command-line interface, language, project tools, and translators are all interfaces for constructing, modifying, validating, and translating a language-independent semantic model.

Chaos is **not** built around Rust, Python, Go, or any other programming language. Those languages are outputs. The semantic model is the source of truth.

---

# 2. Design Philosophy

Chaos is built on a small number of reusable concepts.

Every feature should be decomposed into simple semantic building blocks rather than specialised implementations.

The engine should understand *meaning*, not syntax.

Whenever possible:

* Data should replace code.
* Rules should replace special cases.
* Entities should replace hardcoded logic.
* Relationships should replace nested conditionals.

---

# 3. Core Concepts

Chaos is composed of six fundamental concepts.

## Entity

An Entity represents a semantic object.

Examples include:

* Project
* Frontend
* Backend
* Database
* Function
* Loop
* Worker
* Event

Every entity may contain:

* Properties
* Child entities
* Relationships
* Rules

---

## Property

A Property stores information about an entity.

Examples:

* backend.language
* backend.framework
* frontend.framework
* database.engine

Properties do not contain behaviour.

They only represent state.

---

## Relationship

Relationships describe how entities connect.

Examples:

* Backend owns Framework.
* Project contains Frontend.
* Database belongs to Backend.

Relationships define structure.

---

## Dependency

Dependencies determine whether an entity is allowed to exist.

Example:

Backend Language

Requires:

* Backend = Yes

Example:

Database Layer

Requires:

* Backend = Yes
* Database = Yes

Dependencies determine visibility.

---

## Rule

Rules describe behaviour.

Examples:

If Frontend = No
and Backend = No

Abort project generation.

If Backend Framework = Django

Default ORM = Django ORM.

Rules determine actions.

---

## Manifest

The Manifest represents the complete semantic state of a project.

Every Chaos command reads from and writes to the Manifest.

The Manifest is the single source of truth.

---

# 4. Chaos Engine

The Chaos Engine is responsible for interpreting entities.

The engine performs the following steps:

1. Read entities.
2. Read current Manifest.
3. Evaluate dependencies.
4. Apply rules.
5. Validate project.
6. Produce semantic model.
7. Execute requested operation.

The engine does not understand programming languages.

It only understands entities and their relationships.

---

# 5. Engine Architecture

Chaos
│
├── CLI Layer
│     User interaction
│     Command dispatch
│
├── Engine Layer
│     Orchestrates initialization
│     Maintains semantic state
│
│     ├── Registry
│     │     Question definitions
│     │
│     ├── Dependency
│     │     Dependency language
│     │
│     ├── Resolver
│     │     Determines available questions
│     │
│     ├── Normalizer
│     │     Converts raw input into canonical values
│     │
│     ├── Validator
│     │     Confirms semantic validity
│     │
│     └── SemanticState
│           Current knowledge during initialization
│
├── Manifest Layer
│     ProjectManifest
│     FrontendManifest
│     BackendManifest
│     DatabaseManifest
│     ToolingManifest
│
└── Generator Layer
      (planned)

---

# 6. Commands

Every Chaos command is an interface to the same engine.

## initialize

Purpose

Construct a new semantic project.

Output

A valid Manifest and generated project.

---

## write

Purpose

Construct semantic code entities.

Output

Language-independent semantic code.

---

## edit

Purpose

Modify an existing Manifest.

Output

Updated semantic model.

---

## run

Purpose

Execute a project.

Output

Running application.

---

## translate

Purpose

Convert semantic code into a target programming language.

Output

Generated source code.

---

## doctor

Purpose

Validate project integrity.

Output

Errors, warnings, and recommendations.

---

# 7. Project Model

A Project is the root entity.

```
Project
├── Metadata
├── Frontend
├── Backend
├── Database
├── Tooling
└── State
```

Every project command ultimately modifies this object.

---

# 8. Entity Lifecycle

Every entity follows the same lifecycle.

```
Create

↓

Assign Properties

↓

Resolve Dependencies

↓

Validate

↓

Apply Rules

↓

Store

↓

Generate Output
```

No entity should bypass this process.

---

# 9. Question System

The CLI does not contain project logic.

Instead, it presents entities to the user.

Each question contains:

* Prompt
* Options
* Dependencies
* Validation
* Effects
* Manifest Key

The engine determines whether a question should be displayed.

Questions never determine architecture.

They only collect information.

---

# 10. Manifest

The Manifest stores the semantic state of a project.

Example structure:

```
Project

Frontend

Backend

Database

Tooling

State
```

Future commands such as `edit`, `doctor`, and `run` operate on the Manifest rather than scanning project files.

---

# 11. Generation

Generation is a consequence of the semantic model.

```
Manifest

↓

Template Selection

↓

Project Generator

↓

Filesystem

↓

Dependency Installation

↓

Ready
```

Templates should contain implementation details.

The engine should not.

---

# 12. Translation

Translation is a separate pipeline.

```
Chaos Source

↓

Parser

↓

Semantic Tree

↓

Language Generator

↓

Target Language
```

The parser produces semantic entities.

Generators convert those entities into language-specific syntax.

---

# 13. Guiding Principles

When adding a feature, ask:

* Does this introduce a new semantic concept?
* Can this be represented as an entity?
* Does it belong as data instead of code?
* Can another command reuse it?
* Does it simplify the engine?

If the answer is no, reconsider the design.

---

# 14. Future Work

Future versions may include:

* Plugin system
* Additional project types
* Event system
* Worker system
* AI-assisted generation
* Multiple language backends
* Incremental project migration
* Semantic optimisation

These should extend the existing architecture rather than replace it.