# Chaos v1 Limitations

Chaos v1 establishes the core language and runtime model. It deliberately does not attempt to provide every capability that may eventually become part of the language.

## 1. Mathematical Functionality

Advanced mathematical functionality is outside the scope of v1.

Basic expression evaluation is part of the core language, but specialized mathematical operations and a broader mathematical computation system belong to later language development.

The distinction is:

```text
V1
│
├── expressions
├── state evaluation
└── basic computation

V2+
│
└── mathematical functionality
```

## 2. Tooling

Chaos v1 is focused on the language implementation itself.

A full development toolchain is not required for the v1 language/runtime:

```text
IDE
Debugger
Package manager
Language server
Build tooling
```

These belong to the eventual product ecosystem rather than the core v1 implementation.

## 3. Branch Extensions

The branch data structure provides hierarchical storage and tree-oriented semantics.

Advanced tree algorithms are outside the core requirements of the v1 runtime.

Future implementations may extend branch functionality with additional traversal, search, balancing, or specialized tree operations.

## 4. Standard Library

Chaos v1 provides core language functionality rather than a large standard library.

The language runtime establishes the mechanisms through which additional functionality can eventually be implemented.

## 5. External Systems

Chaos v1 does not depend on external services or network infrastructure.

The language executes locally using its C runtime.

```text
.chaos
   │
   ▼
Chaos Runtime
   │
   ▼
Local State
```

This keeps the core language self-contained.

## 6. Stability Boundary

The v1 boundary is intentionally focused:

```text
                 CHAOS V1
                    │
        ┌───────────┴───────────┐
        │                       │
     Language                Runtime
        │                       │
      Lexer                    State
      Parser                   Logic
      AST                      Rules
      Syntax                   Context
                               Contracts
                               Transitions
                               Collections
                               Expressions
```

Features outside this boundary should not be added merely for the sake of increasing the feature count.

Chaos v1 is intended to provide a complete foundation on which later language functionality can be built.