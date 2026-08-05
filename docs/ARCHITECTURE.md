# Chaos Architecture

> *Everything in Chaos exists for one reason: to transform information into action.*

---

# Philosophy

Chaos is built from the observation that every digital system, regardless of its complexity, is simply many small computational devices working together.

A calculator.
A compiler.
A web server.
A spacecraft guidance computer.

They differ only in scale.

Chaos therefore models software as a hierarchy of increasingly capable computational entities.

Each layer has one responsibility.

No layer should perform the responsibilities of another.

---

# Hierarchy

```
Logic
    ↓
Contract
    ↓
Block
    ↓
Calculator
    ↓
Engine
    ↓
Brain
```

Each layer exists because it introduces a new capability.

---

# Logic

Logic is the smallest unit of computation.

Chaos recognizes three categories of logic.

## Logic 0

Represents binary physical reality.

```
0
1
```

This is the level closest to hardware.

---

## Logic 1

Represents deterministic computational operations built upon Logic 0.

Examples include:

* copy
* compare
* branch
* store
* transmit
* invert

Logic 1 performs work.

---

## Logic 2

Represents semantic intent.

Logic 2 exists only inside Chaos.

It describes what the programmer wants to accomplish rather than how hardware performs it.

Logic 2 is eventually translated into Logic 1 operations.

---

# Contracts

A contract defines the rules under which computation occurs.

A contract specifies:

* required inputs
* produced outputs
* assumptions
* guarantees

A contract never describes implementation.

It only describes behavior.

If two implementations satisfy the same contract, they are interchangeable.

---

# Blocks

A block is the smallest reusable implementation unit.

A block satisfies exactly one contract.

Internally, a block combines Logic 0, Logic 1, and Logic 2 to perform its work.

Blocks should remain small enough that their purpose can be understood immediately.

---

# Calculators

A calculator performs one complete computation.

A calculator is composed from one or more blocks.

A calculator should solve exactly one problem.

Examples:

* Validate Project
* Generate README
* Install Dependencies
* Parse Syntax
* Compile Project

A calculator does not coordinate other calculators.

Its responsibility ends once its own computation is complete.

Version 1 represents each calculator as a single file.

---

# Calculator Pipelines

Multiple calculators may be connected together through a calculator pipeline.

```
c.pipeline
```

A calculator pipeline determines execution order.

It allows independent calculators to cooperate while remaining individually reusable.

The output of one calculator may become the input of another.

---

# Engines

An engine is a collection of calculators focused on one domain.

Examples may include:

* Initialize Engine
* Write Engine
* Run Engine
* Language Engine
* Documentation Engine

Each engine exposes one coherent capability.

Internally, engines organize calculators using one or more calculator pipelines.

Version 1 represents each engine as a folder.

---

# Engine Pipelines

Engines communicate through an engine pipeline.

```
e.pipeline
```

Unlike calculator pipelines, engine pipelines coordinate entire systems rather than individual computations.

---

# Brain

The brain is the complete Chaos project.

It coordinates every engine.

It understands the user's request and delegates work to the appropriate engines.

The brain performs no low-level computation itself.

Its responsibility is orchestration.

Version 1 represents the brain as the project's root `.chaos` file.

---

# Information Flow

Every computation inside Chaos follows the same path.

```
Input
    ↓
Logic
    ↓
Contract
    ↓
Block
    ↓
Calculator
    ↓
Engine
    ↓
Brain
    ↓
Output
```

No computation should skip layers.

---

# Design Principles

## Single Responsibility

Every architectural layer exists for exactly one purpose.

---

## Explicit Contracts

Behavior is always described before implementation.

---

## Composability

Small computational units should combine to create larger systems.

Large systems should decompose naturally into smaller ones.

---

## Replaceability

Any implementation satisfying a contract may replace another implementation without changing the surrounding architecture.

---

## Hardware First

Chaos treats software as a physical computational system.

The language is inspired by digital logic rather than traditional programming language syntax.

Abstractions exist to simplify interaction with hardware, never to hide the existence of computation itself.

---

# Version 1 Scope

Version 1 is a web developer's pocket tool.

The architecture exists independently of its features.

Version 1 focuses on project initialization, scaffolding, dependency installation, repository setup, documentation generation, and developer tooling.

Future versions may expand the architecture without changing its fundamental hierarchy.

The hierarchy is intended to remain constant even as new engines, calculators, and blocks are introduced.