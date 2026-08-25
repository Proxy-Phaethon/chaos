# Chaos v1 Scope Boundaries

Chaos v1 focuses on the core language and runtime.

It provides the working pipeline from `.chaos` source to tokens, AST execution, and final `RuntimeStateStore`.

## In Scope For v1

Chaos v1 includes:

* Lexing
* Parsing
* AST construction and inspection
* Runtime dispatch
* Register execution
* State storage
* Scalar state values
* List, queue, stack, and branch state values
* Collection initialization
* Collection `push`
* Collection `pop`
* Collection type validation
* Logic as the container for active computation
* Parsed constants
* Parsed conditionals
* Parsed contracts
* Parsed transitions
* Parsed contexts and rules
* Top-level `execute`

## Runtime Boundary

The v1 runtime mutates state through register execution and collection operations.

These constructs are parsed into the AST but do not perform state-changing runtime behavior in v1:

* Expression evaluation
* Conditional branch selection
* Contract resolution and invocation
* Transition application
* Context activation
* Rule evaluation
* Result propagation
* Termination control flow

They are still part of the v1 language shape because the lexer, parser, and AST represent them.

## Mathematical Functionality

Advanced mathematical functionality is outside v1.

Chaos v2 is reserved for mathematical functionality rather than CLI or tooling work.

## Tooling

CLI polish, package management, editor integrations, language-server support, debuggers, and external ecosystem tooling are outside v1.

They belong to the eventual product/ecosystem stage.

## Branch Behavior

`branch` is a distinct v1 runtime type and supports initialization, push, pop, type validation, and printing.

In v1, branch storage uses the same collection backing store as lists, queues, and stacks. Advanced tree algorithms, traversal, balancing, and search are outside the v1 core.

## External Systems

Chaos v1 runs locally through its C runtime.

It does not depend on external services, network infrastructure, hosted package registries, or third-party execution systems.
