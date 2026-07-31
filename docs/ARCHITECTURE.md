# Chaos Architecture

**Version:** 0.1

---

# 1. Purpose

Chaos is a semantic software engineering engine.

Its command-line interface, language, project tools, and translators are all interfaces for constructing, modifying, validating, and translating a language-independent semantic model.

Chaos is not built around Rust, Python, Go, or any other programming language. Those languages are outputs.

The semantic model is the source of truth.

---

# 2. Design Philosophy

Chaos is built around a small number of reusable semantic concepts.

Every feature should be decomposed into simple semantic building blocks rather than specialized implementations.

The engine should understand meaning, not syntax.

Whenever possible:

* Data should replace code.
* Rules should replace special cases.
* Entities should replace hardcoded logic.
* Relationships should replace nested conditionals.
* Semantic models should replace implementation details.

---

# 3. Core Concepts

Chaos is composed of several fundamental semantic concepts.

## Entity

An Entity represents a semantic object.

Examples include:

* Project
* Frontend
* Backend
* Database
* Function
* Worker
* Event
* Loop

Every entity may contain:

* Properties
* Child entities
* Relationships
* Dependencies

---

## Property

A Property stores information about an entity.

Examples:

* backend.language
* backend.framework
* frontend.framework
* database.engine

Properties never contain behaviour.

They represent state only.

---

## Relationship

Relationships describe how entities connect.

Examples:

* Project contains Frontend.
* Project contains Backend.
* Project contains Database.
* Backend uses Database.
* Backend exposes API.

Relationships define structure.

---

## Dependency

Dependencies determine whether an entity, property, or question is applicable.

Example:

Backend Language

Requires:

* Backend = Yes

Example:

Database Engine

Requires:

* Backend = Yes
* Database = Yes

Dependencies determine availability.

---

## Semantic State

The Semantic State represents the current knowledge accumulated while the engine is reasoning.

It exists only while an operation is in progress.

Unlike the Manifest, the Semantic State is temporary.

It allows the engine to progressively evaluate dependencies as additional information becomes available.

---

## Manifest

The Project Manifest represents the completed semantic state of a project.

It is the authoritative representation of the project.

Every Chaos command ultimately reads from or writes to the Manifest.

---

# 4. Chaos Engine

The Chaos Engine is responsible for interpreting semantic entities.

It performs reasoning independently of any implementation language.

Its responsibilities include:

1. Loading semantic entities.
2. Resolving dependencies.
3. Determining available questions.
4. Normalizing user input.
5. Validating semantic correctness.
6. Constructing semantic state.
7. Producing a Project Manifest.

The engine never generates implementation-specific code directly.

---

# 5. Engine Architecture

```text
Chaos
│
├── CLI Layer
│     User interaction
│     Command dispatch
│
├── Engine Layer
│     Semantic reasoning
│
│     ├── Registry
│     │     Declarative question definitions
│     │
│     ├── Dependency
│     │     Semantic dependency language
│     │
│     ├── Resolver
│     │     Evaluates dependencies
│     │     Determines available questions
│     │
│     ├── Normalizer
│     │     Converts raw input into canonical values
│     │
│     ├── Validator
│     │     Confirms semantic correctness
│     │
│     └── SemanticState
│           Current semantic knowledge
│
├── Manifest Layer
│     ProjectManifest
│     FrontendManifest
│     BackendManifest
│     DatabaseManifest
│     ToolingManifest
│
└── Generator Layer
      Project generation
      (planned)
```

The initialization workflow follows the pipeline below.

```text
Registry
        │
        ▼
Resolver
        │
        ▼
Available Question
        │
        ▼
CLI Prompt
        │
        ▼
Raw Answer
        │
        ▼
Normalizer
        │
        ▼
Validator
        │
        ▼
SemanticState
        │
        └──────────────┐
                       │
                       ▼
                  Resolver
```

The cycle repeats until no unanswered questions remain.

The completed Semantic State is converted into a Project Manifest.

The Generator consumes the Manifest to produce implementation-specific projects.

---

# 6. Commands

Every Chaos command is an interface to the same semantic engine.

## initialize

Construct a new semantic project.

Produces a validated Project Manifest and generates a project.

---

## write

Construct semantic program entities.

Produces language-independent semantic code.

---

## edit

Modify an existing Project Manifest.

Produces an updated semantic model.

---

## run

Execute an existing project.

Produces a running application.

---

## translate

Convert semantic code into a target programming language.

Produces generated source code.

---

## doctor

Validate project integrity.

Produces errors, warnings, and recommendations.

---

# 7. Project Model

Every project is represented by a root Project entity.

```text
Project
├── Metadata
├── Frontend
├── Backend
├── Database
├── Tooling
└── State
```

Every project command ultimately modifies this structure.

---

# 8. Entity Lifecycle

Every semantic entity follows the same lifecycle.

```text
Create
   │
   ▼
Assign Properties
   │
   ▼
Resolve Dependencies
   │
   ▼
Validate
   │
   ▼
Store
   │
   ▼
Generate Output
```

No entity should bypass this lifecycle.

---

# 9. Question System

The CLI contains no architectural knowledge.

Instead, it presents semantic questions supplied by the Registry.

Each question defines:

* Identifier
* Prompt
* Answer Type
* Options
* Dependencies
* Manifest Mapping
* Effects

The Resolver determines whether a question is currently available.

Questions never determine project architecture.

They only collect semantic information.

---

# 10. Manifest

The Project Manifest stores the complete semantic state of a project.

Typical structure:

```text
Project
├── Metadata
├── Frontend
├── Backend
├── Database
├── Tooling
└── State
```

Future commands such as `edit`, `doctor`, `run`, and `translate` operate on the Manifest rather than scanning implementation files.

---

# 11. Generation

Project generation is a consequence of the Manifest.

```text
ProjectManifest
        │
        ▼
Template Selection
        │
        ▼
Project Generator
        │
        ▼
Filesystem
        │
        ▼
Dependency Installation
        │
        ▼
Ready
```

Templates contain implementation-specific knowledge.

The engine does not.

---

# 12. Translation

Translation is an independent pipeline.

```text
Chaos Source
        │
        ▼
Parser
        │
        ▼
Semantic Tree
        │
        ▼
Language Generator
        │
        ▼
Target Language
```

The parser produces semantic entities.

Language generators convert semantic entities into implementation-specific syntax.

---

# 13. Guiding Principles

When introducing a new feature, ask:

* Does this introduce a new semantic concept?
* Can it be represented as an entity?
* Should it exist as data rather than implementation logic?
* Can another command reuse it?
* Does it simplify the engine?

If the answer is no, reconsider the design.

---

# 14. Future Work

Future versions may introduce:

* Plugin architecture
* Additional project types
* Event system
* Worker system
* Semantic optimizer
* AI-assisted generation
* Multiple language generators
* Incremental project migration
* Distributed execution

These capabilities should extend the existing semantic architecture rather than replace it.