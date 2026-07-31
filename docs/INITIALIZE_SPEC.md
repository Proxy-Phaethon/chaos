# Chaos Initialize Specification

**Version:** 1.0

---

# 1. Introduction

`chaos initialize` is the project construction interface of the Chaos Engine.

Its responsibility is to construct a valid semantic representation of a project from user input. Rather than generating implementation files directly, the command gathers project requirements, validates each answer, constructs a semantic project model, produces a `ProjectManifest`, and invokes the generation pipeline.

The command itself contains no language-specific logic. All architectural decisions are represented as semantic entities, dependencies, and relationships managed by the Chaos Engine.

---

# 2. Scope

This specification defines:

* The purpose of project initialization.
* The semantic initialization workflow.
* The dependency graph governing question availability.
* Input normalization.
* Semantic validation.
* Manifest construction.
* Project generation.

This specification does **not** define:

* CLI implementation.
* Parser implementation.
* Syntax translation.
* Runtime behaviour.
* Template implementation.
* Filesystem implementation.

These topics are specified independently.

---

# 3. Objectives

The initialization process shall satisfy the following objectives.

1. Construct a valid `ProjectManifest`.
2. Collect only information required to describe the project.
3. Present only contextually valid questions.
4. Prevent invalid project configurations.
5. Produce deterministic output from identical input.
6. Remain independent of implementation language.
7. Provide sufficient semantic information for subsequent Chaos commands.

---

# 4. Core Principles

Each component of the initialization system has exactly one responsibility.

| Component       | Responsibility                                                   |
| --------------- | ---------------------------------------------------------------- |
| Registry        | Defines semantic questions.                                      |
| Resolver        | Determines which questions are currently available.              |
| Normalizer      | Converts raw user input into canonical values.                   |
| Validator       | Determines whether a normalized answer is semantically valid.    |
| SemanticState   | Represents the current knowledge gathered during initialization. |
| ProjectManifest | Represents the completed semantic description of the project.    |
| Generator       | Produces implementation-specific project files.                  |

No component should perform the responsibilities of another.

---

# 5. Initialization Pipeline

Project initialization follows the semantic workflow below.

```text
Registry
        │
        ▼
Chaos Engine
        │
        ▼
Resolver
        │
        ▼
Next Available Question
        │
        ▼
CLI Prompt
        │
        ▼
Raw User Answer
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

The process repeats until no unanswered questions remain.

The completed `SemanticState` is converted into a `ProjectManifest`.

The `ProjectManifest` becomes the exclusive input to the Generation Pipeline.

---

# 6. Input Model

Project initialization consists of a sequence of semantic questions.

Each question defines:

* Identifier
* Purpose
* Prompt
* Answer type
* Available options
* Dependencies
* Validation
* Manifest mapping
* Effects

Questions are passive data structures.

They never contain project logic.

All project logic is evaluated by the Chaos Engine.

---

# 7. Question Flow

The canonical Version 1 dependency tree is:

```text
Project Name
│
├── Frontend?
│     │
│     ├── Frontend Language
│     ├── Frontend Framework
│     ├── Routing
│     ├── Styling
│     └── State Management
│
├── Backend?
│     │
│     ├── Backend Language
│     ├── Backend Framework
│     │
│     ├── Database?
│     │      │
│     │      ├── Database Engine
│     │      └── ORM
│     │
│     ├── Authentication
│     └── API Style
│
├── Git
├── Docker
└── Testing
```

Questions are presented only when every dependency evaluates to true.

---

# 8. Dependency Resolution

The Resolver evaluates dependencies dynamically throughout initialization.

Examples include:

```
Frontend Language

Requires

Frontend = Yes
```

```
Backend Framework

Requires

Backend = Yes

Backend Language selected
```

```
Database Engine

Requires

Backend = Yes

Database = Yes
```

```
ORM

Requires

Backend = Yes

Database = Yes

Backend Framework selected
```

Questions whose dependencies evaluate to false are not presented.

Dependency resolution is deterministic and side-effect free.

---

# 9. Input Normalization

Raw user input is normalized before validation.

Normalization may include:

* Whitespace trimming.
* Identifier sanitization.
* Case normalization.
* Boolean alias resolution.
* Canonical option matching.

Normalization never rejects input.

It only converts input into canonical semantic values.

---

# 10. Validation

Validation occurs immediately after normalization for each answer.

Validation determines whether a normalized answer satisfies the requirements of its corresponding question.

Examples include:

* Identifiers must not be empty.
* Boolean questions must produce boolean values.
* Choice questions must match one of the declared options.
* Required answers must be present.

When initialization is complete, a final manifest validation confirms the overall project configuration is internally consistent.

Examples include:

* At least one application layer exists.
* Selected frameworks support the selected language.
* ORM supports the selected framework.
* Unsupported combinations are rejected.

No project generation occurs if validation fails.

---

# 11. Semantic State

During initialization, all accepted answers are stored in the `SemanticState`.

The `SemanticState` represents the current semantic knowledge of the project.

It exists only during initialization.

It is independent of the `ProjectManifest`.

The Resolver evaluates dependencies exclusively against the current `SemanticState`.

---

# 12. Project Manifest

Successful initialization culminates in the construction of a `ProjectManifest`.

The manifest is the authoritative semantic representation of the completed project.

Typical sections include:

```text
Project

Frontend

Backend

Database

Tooling

State
```

All subsequent Chaos commands operate from the manifest rather than implementation-specific files.

The manifest is never reconstructed from generated source code.

---

# 13. Generation Pipeline

Following successful validation, the Generation Pipeline performs the following high-level stages.

1. Select implementation templates.
2. Construct project directory structure.
3. Generate implementation files.
4. Generate configuration files.
5. Initialize selected tooling.
6. Produce project summary.

Generation consumes the `ProjectManifest` exclusively.

---

# 14. Default Behaviour

Where a question defines a default value, selecting no option shall produce the documented default.

Version 1 defaults are:

| Question                    | Default      |
| --------------------------- | ------------ |
| Frontend Language           | TypeScript   |
| Frontend Framework          | React        |
| Routing                     | Yes          |
| Styling                     | Tailwind CSS |
| State Management            | None         |
| Backend Framework (Python)  | Django       |
| Backend Framework (Go)      | Gin          |
| Backend Framework (Rust)    | Axum         |
| Backend Framework (Node.js) | Express      |
| Backend Framework (PHP)     | Laravel      |
| Database Engine             | PostgreSQL   |
| Authentication              | None         |
| API Style                   | REST         |
| Git                         | Yes          |
| Docker                      | No           |
| Testing                     | Unit         |

Defaults shall remain deterministic across identical Chaos versions.

---

# 15. Failure Conditions

Initialization shall terminate under any of the following conditions.

* Neither Frontend nor Backend selected.
* Unsupported dependency combination.
* Invalid normalized answer.
* Manifest validation failure.
* Existing project cannot be safely overwritten.
* Generation pipeline failure.

Where practical, generation should terminate before filesystem modification.

If generation fails after modification has begun, the implementation should attempt rollback.

---

# 16. Extensibility

The initialization system is designed to evolve through extension rather than modification.

Future versions may introduce:

* Mobile application support.
* Desktop application support.
* Game development.
* Infrastructure provisioning.
* CI/CD configuration.
* Plugin-defined question registries.
* Custom project archetypes.

Extensions shall integrate through the existing Registry, Resolver, Normalizer, Validator, and Manifest systems.

No extension should require changes to the fundamental initialization architecture.

---

# 17. Conformance

An implementation conforms to this specification if it:

* Presents questions according to the dependency graph.
* Normalizes user input consistently.
* Validates semantic correctness.
* Produces a valid `ProjectManifest`.
* Rejects invalid configurations.
* Generates deterministic output.
* Maintains semantic equivalence between identical project definitions.

The implementation language, CLI framework, and internal code organization are outside the scope of this specification.