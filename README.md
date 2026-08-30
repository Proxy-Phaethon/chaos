<div align="center"> <pre> 
  ░██████  ░██     ░██    ░███      ░██████     ░██████   
 ░██   ░██ ░██     ░██   ░██░██    ░██   ░██   ░██   ░██  
░██        ░██     ░██  ░██  ░██  ░██     ░██ ░██         
░██        ░██████████ ░█████████ ░██     ░██  ░████████  
░██        ░██     ░██ ░██    ░██ ░██     ░██         ░██ 
 ░██   ░██ ░██     ░██ ░██    ░██  ░██   ░██   ░██   ░██  
  ░██████  ░██     ░██ ░██    ░██   ░██████     ░██████   
</pre>

[![Typing SVG](https://readme-typing-svg.demolab.com?font=Fira+Code&size=20&pause=1000&color=00FF9C&center=true&vCenter=true&width=440&lines=please+save+me.;actually+i+love+building+this+thing.;praise+chaos+the+primordial.)](https://git.io/typing-svg)

</div>

<div align="center">

![C](https://img.shields.io/badge/C-000000?style=for-the-badge&logo=c&logoColor=white)

</div>

---

# Chaos

Chaos is a small experimental programming language implemented in C.

It is designed as a research-oriented project for exploring programming-language implementation, parsing, abstract syntax trees, runtime execution, mutable state, and data structures.

Chaos programs use the `.chaos` file extension.

## Current Version

**Version:** 1.0
**Status:** Experimental / functional v1
**Implementation language:** C
**Standard:** C11

Chaos v1 provides a complete source-to-runtime pipeline:

```text
Chaos source
     │
     ▼
   Lexer
     │
     ▼
 Token stream
     │
     ▼
   Parser
     │
     ▼
     AST
     │
     ▼
  Runtime
     │
     ▼
Runtime State Store
```

The v1 implementation includes lexical analysis, parsing, AST construction, runtime execution, mutable runtime state, scalar values, collections, conditional logic, and basic control-flow constructs.

## Features

Chaos v1 currently supports:

* State registration
* Numeric and string values
* Expression values
* Lists
* Queues
* Stacks
* Binary-search-tree-style branches
* Push and pop operations
* Constants
* Conditional execution
* `if`, `else if`, and `else`
* Transitions
* Context and rule syntax
* Execute statements
* Runtime state inspection
* Duplicate state protection
* Dynamic collection storage

The language is intentionally small. The goal of v1 is to establish a working language pipeline and runtime rather than provide a large general-purpose standard library.

## Example

A small Chaos program:

```chaos
register("example"):
    state: integer = 42,
    state: name = "Chaos",
    state: numbers, list = "one", "two", "three";

logic integer > 0;
    constant: integer is positive;

    list numbers
        (push "four")
        (pop),

    execute;
```

Chaos first parses the source into an abstract syntax tree.

The runtime then executes the resulting program and maintains its state separately from the AST.

A runtime state store might contain:

```text
numbers [list] = {two, three, four}
name [string] = Chaos
integer [number] = 42
```

## Data Structures

Chaos v1 includes four collection types.

### List

A list stores values in insertion order.

```chaos
list fruits
    (push "apple")
    (push "banana")
    (pop),
```

`pop` removes the first element.

### Queue

A queue follows FIFO behavior.

```chaos
queue waiting
    (push "first")
    (push "second")
    (pop),
```

The first inserted element is removed first.

### Stack

A stack follows LIFO behavior.

```chaos
stack history
    (push "older")
    (push "newer")
    (pop),
```

The most recently inserted element is removed first.

### Branch

A branch is represented internally as a binary search tree.

```chaos
branch tree
    (push "50")
    (push "25")
    (push "75"),
```

Values are ordered using string comparison in the current v1 runtime.

Duplicate values are not inserted.

## Repository Structure

```text
chaos/
├── include/
│   ├── ast.h
│   ├── lexer.h
│   ├── parser.h
│   ├── runtime.h
│   └── runtime_state.h
│
├── src/
│   ├── ast.c
│   ├── lexer.c
│   ├── main.c
│   ├── parser.c
│   ├── runtime.c
│   └── runtime_state.c
│
├── docs/
│   └── v1/
│       ├── architecture.md
│       ├── syntax.md
│       ├── runtime-and-state-systems.md
│       ├── data-structures.md
│       ├── logic.md
│       ├── contracts.md
│       └── transitions.md
│
├── examples/
│   └── *.chaos
│
├── Makefile
├── LICENSE
└── README.md
```

## Building

Chaos uses a small Makefile-based build system.

On a system with a C11-compatible compiler:

```bash
make
```

This produces the Chaos executable:

```text
./chaos
```

To remove the compiled executable:

```bash
make clean
```

## Running a Program

Pass a `.chaos` source file to the executable:

```bash
./chaos examples/all_v1.chaos
```

A successful parse produces the AST, followed by runtime output and the final runtime state store.

For example:

```text
Parsed successfully.

PROGRAM
  REGISTER: everything
    STATE
      NAME: integer
      VALUE: 42
    ...

Runtime:
CONSTANT: integer is positive
POP fruits: apple
POP waiting: first
POP history: newest
TRANSITION: none
EXECUTE

State store:
...
```

## Implementation

Chaos is implemented entirely in C.

The lexer converts source text into tokens. The parser consumes those tokens and constructs an abstract syntax tree. The runtime walks the AST and performs the operations represented by it.

Runtime state is maintained separately through the `RuntimeStateStore`.

This separation gives Chaos a simple architecture:

```text
Source
  │
  ▼
Lexer
  │
  ▼
Tokens
  │
  ▼
Parser
  │
  ▼
AST
  │
  ├──────────────┐
  ▼              ▼
Runtime      AST inspection
  │
  ▼
RuntimeStateStore
```

The runtime state system handles scalar values and collection-specific storage. Lists, queues, and stacks use dynamically allocated arrays, while branches use dynamically allocated tree nodes.

## Design Goals

Chaos v1 focuses on a few core ideas:

1. Build a programming language from the ground up.
2. Keep the implementation small enough to understand.
3. Separate syntax from runtime behavior.
4. Represent programs explicitly through an AST.
5. Maintain mutable runtime state independently of the AST.
6. Provide several fundamental data structures directly within the language.
7. Establish a foundation for future language research.

The project favors clarity over optimization in v1.

## Limitations

Chaos v1 is intentionally incomplete as a general-purpose programming language.

Current limitations include:

* Expression evaluation is limited compared with a conventional programming language.
* Runtime semantics for some parsed constructs remain minimal.
* Contracts are represented by the parser but do not provide a general contract execution system.
* Context and rule constructs have limited runtime behavior.
* Transitions currently provide basic runtime behavior rather than a complete state-transition system.
* Branch values currently use string comparison.
* Collections store values as strings.
* There is no standard library.
* There is no module or import system.
* There is no user-defined function system.
* There is no sophisticated type system.
* Error recovery in the parser is limited.
* Memory management is intentionally straightforward rather than optimized.
* There is no bytecode compiler or virtual machine.
* There is no optimization pass.
* The language is not intended to be production-ready.

These limitations define the boundary of v1 rather than bugs that v1 is expected to hide.

## Version 1 Scope

The primary objective of v1 is a working language implementation with the following pipeline:

```text
Lexing
  ↓
Parsing
  ↓
AST construction
  ↓
Runtime execution
  ↓
Mutable state
```

Future versions can build on this foundation with more sophisticated expression evaluation, control flow, typing, state transitions, mathematical functionality, and other language features.

## Documentation

Detailed v1 documentation is available in `docs/v1/`:

* `architecture.md` - compiler/interpreter architecture and execution pipeline
* `syntax.md` - Chaos v1 syntax
* `runtime-and-state-systems.md` - runtime execution and state management
* `data-structures.md` - lists, queues, stacks, and branches
* `logic.md` - logic and control flow
* `contracts.md` - contract syntax and current semantics
* `transitions.md` - transition syntax and current semantics

## Status

Chaos v1 is a functional experimental implementation.

The project is primarily intended for learning and research into programming-language implementation rather than production software development.

The v1 milestone establishes the core language architecture. Future development will build on this foundation rather than expanding the v1 scope indefinitely.