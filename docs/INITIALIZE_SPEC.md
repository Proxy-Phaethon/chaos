# Chaos Initialize Specification

**Version:** 1.0

---

# 1. Introduction

`chaos initialize` is the project construction interface of the Chaos Engine.

Its responsibility is to construct a valid semantic project model from user input. Rather than generating files directly, `chaos initialize` gathers information about the intended project, validates the resulting configuration, constructs a Project Manifest, and invokes the generation pipeline.

The command contains no language-specific behaviour beyond presenting available options to the user. All architectural decisions are represented as semantic entities and relationships within the Project Manifest.

---

# 2. Scope

This specification defines:

* The purpose of project initialisation.
* The sequence of user interaction.
* The dependency graph governing available questions.
* Validation rules.
* Manifest construction.
* Project generation.

This specification does **not** define:

* CLI implementation.
* Parser implementation.
* Language translation.
* Runtime behaviour.
* Template implementation.

These topics are specified independently.

---

# 3. Objectives

The initialisation process shall satisfy the following objectives.

1. Construct a valid Project Manifest.
2. Collect only information required to describe the project.
3. Present only contextually valid questions.
4. Prevent invalid project configurations.
5. Produce deterministic output from identical input.
6. Remain independent of implementation language.
7. Provide sufficient information for subsequent Chaos commands.

---

# 4. Initialisation Pipeline

Project construction follows the sequence below.

```text
User Input
      │
      ▼
Question Engine
      │
      ▼
Dependency Resolution
      │
      ▼
Validation
      │
      ▼
Project Manifest
      │
      ▼
Generation Pipeline
      │
      ▼
Completed Project
```

Each stage has a single responsibility.

The Question Engine gathers information.

The Dependency Resolver determines which questions are applicable.

The Validator confirms the resulting project is internally consistent.

The Manifest records the semantic state of the project.

The Generation Pipeline converts the semantic model into implementation-specific files.

---

# 5. Input Model

Project initialisation is performed through a sequence of semantic questions.

Each question defines:

* Identifier
* Purpose
* Prompt
* Available options
* Dependencies
* Validation
* Manifest mapping
* Side effects

Questions shall never contain project logic.

All project logic is evaluated by the Chaos Engine.

---

# 6. Question Flow

The following sequence represents the canonical initialisation flow for Version 1.

```
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
│     ├── Database?
│     │      │
│     │      ├── Database Engine
│     │      └── Database Layer
│     │
│     ├── Authentication
│     └── API Style
│
├── Git
├── Docker
└── Testing
```

Questions are presented only when their dependency conditions are satisfied.

---

# 7. Question Definitions

Each question shall conform to the following structure.

---

## Question Identifier

A globally unique identifier.

Example

```
backend.language
```

---

## Purpose

Describes the semantic role of the question.

Example

Select the implementation language of the backend application.

---

## Prompt

The user-facing prompt.

Example

```
Select backend language
```

---

## Options

The permitted responses.

Example

* Python
* Go
* Rust
* Node.js
* PHP
* Java
* C#

---

## Dependencies

Conditions that must evaluate to true before the question is presented.

Example

```
Backend = Yes
```

---

## Validation

Rules governing acceptable responses.

Example

The selected language must exist within the supported language registry.

---

## Manifest Mapping

Destination within the Project Manifest.

Example

```
manifest.backend.language
```

---

## Effects

Semantic consequences of the selected option.

Example

Selecting a backend language determines the available backend frameworks.

---

# 8. Dependency Graph

The Question Engine shall evaluate dependencies before presenting each question.

Dependencies are evaluated dynamically throughout initialisation.

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
Database Layer

Requires

Backend = Yes

Database = Yes

Backend Framework selected
```

Questions whose dependencies evaluate to false shall not be presented.

---

# 9. Validation

Validation occurs after all required questions have been answered.

Validation ensures that the resulting semantic model is internally consistent.

Validation includes, but is not limited to:

* At least one application layer must exist.
* Selected framework must support the selected language.
* Selected database layer must support the selected database engine.
* Required values must be present.
* Unsupported combinations must be rejected.

Validation failure terminates project generation.

No files shall be generated for an invalid project.

---

# 10. Project Manifest

Successful initialisation produces a Project Manifest.

The Manifest represents the complete semantic state of the project.

The Manifest is the authoritative representation of the project and shall be used by all subsequent Chaos commands.

Typical sections include:

```
Project

Frontend

Backend

Database

Tooling

State
```

Implementation-specific configuration files are generated from the Manifest.

The Manifest is never generated from implementation-specific files.

---

# 11. Generation Pipeline

Following successful validation, the Generation Pipeline performs the following operations.

1. Select implementation templates.
2. Construct project directory structure.
3. Generate implementation files.
4. Generate configuration files.
5. Install project dependencies.
6. Initialise optional tooling.
7. Produce project summary.

Generation consumes the Project Manifest exclusively.

---

# 12. Default Behaviour

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

# 13. Failure Conditions

Project generation shall terminate under any of the following conditions.

* Neither Frontend nor Backend selected.
* Unsupported dependency combination.
* Invalid user input.
* Existing project cannot be safely overwritten.
* Manifest validation failure.
* Generation pipeline failure.

Where practical, generation shall terminate before filesystem modification.

Where generation fails after modification has begun, the implementation should attempt rollback.

---

# 14. Extensibility

The initialisation process is designed to evolve through extension rather than modification.

Future versions may introduce:

* Additional project categories.
* Mobile application support.
* Desktop application support.
* Game development support.
* Infrastructure provisioning.
* CI/CD configuration.
* Plugin-defined question sets.

Extensions shall integrate through the existing dependency and validation systems.

No extension should require changes to the fundamental architecture of project initialisation.

---

# 15. Conformance

An implementation conforms to this specification if it:

* Presents questions according to the dependency graph.
* Produces a valid Project Manifest.
* Rejects invalid configurations.
* Generates deterministic output.
* Maintains semantic equivalence between identical project definitions.

The implementation language, CLI framework, and code organisation are outside the scope of this specification.