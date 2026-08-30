# Chaos v1 Architecture

## Overview

Chaos v1 is implemented as a small interpreter pipeline written in C.

A `.chaos` source file passes through several distinct stages before producing runtime behavior:

```text
┌─────────────────┐
│  Chaos Source   │
│    .chaos       │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│     Lexer       │
│   lexer.c/.h    │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   Token Stream  │
│     TokenList   │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│     Parser      │
│  parser.c/.h    │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│       AST       │
│    ast.c/.h     │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│     Runtime     │
│ runtime.c/.h    │
└────────┬────────┘
         │
         ▼
┌──────────────────────┐
│ Runtime State Store  │
│ runtime_state.c/.h   │
└──────────────────────┘
```

Each stage has a specific responsibility. The lexer does not interpret programs, the parser does not execute them, and the runtime does not reconstruct source syntax.

This separation is the central architectural principle of Chaos v1.

## 1. Source Layer

Chaos programs are plain-text files using the `.chaos` extension.

For example:

```chaos
register("example"):
    state: integer = 42,
    state: numbers, list = "one", "two";

logic integer > 0;
    constant: integer is positive;

execute;
```

The source file is passed to the Chaos executable:

```bash
./chaos examples/example.chaos
```

`main.c` is responsible for starting this process and connecting the individual stages together.

## 2. Lexer

The lexer is implemented in:

```text
src/lexer.c
include/lexer.h
```

Its responsibility is to convert raw source text into a sequence of tokens.

Conceptually:

```text
Source text
     │
     ▼
┌────────────┐
│   Lexer    │
└─────┬──────┘
      │
      ▼
TokenList
```

The lexer recognizes the vocabulary required by Chaos v1, including keywords such as:

```text
register
state
logic
constant
transition
context
rule
execute
push
pop
list
queue
stack
branch
if
else
```

It also recognizes identifiers, numbers, strings, expressions, punctuation, and other syntax elements.

Each token carries information needed by later stages, including its token type, textual value, and source location.

The lexer does not determine what a sequence of tokens means. That is the parser's responsibility.

## 3. Token Stream

The lexer produces a `TokenList`.

The token stream acts as the interface between lexical analysis and parsing.

For example, a statement such as:

```chaos
state: integer = 42
```

is conceptually represented as:

```text
STATE
COLON
IDENTIFIER("integer")
EQUALS
NUMBER("42")
```

The parser consumes these tokens sequentially.

The parser maintains its current position using a `Parser` structure:

```text
Parser
├── tokens
├── current
└── had_error
```

This allows parsing to proceed deterministically through the token stream.

## 4. Parser

The parser is implemented in:

```text
src/parser.c
include/parser.h
```

The parser consumes the token stream and constructs an Abstract Syntax Tree.

```text
TokenList
    │
    ▼
┌────────────┐
│   Parser   │
└─────┬──────┘
      │
      ▼
    AST
```

The parser is responsible for understanding the structure of a Chaos program.

For example:

```chaos
state: integer = 42
```

becomes a structure conceptually equivalent to:

```text
STATE
└── NAME: integer
    └── VALUE: 42
```

The parser also validates the expected syntax.

If a required token is missing, it reports a parser error containing the source location and nearby token.

## 5. Abstract Syntax Tree

The AST is implemented in:

```text
src/ast.c
include/ast.h
```

The AST provides a tree representation of the parsed program.

The root node is:

```text
PROGRAM
```

Its descendants represent declarations, logic, operations, and execution statements.

A simplified example:

```text
PROGRAM
├── REGISTER
│   ├── STATE
│   │   ├── NAME: integer
│   │   └── VALUE: 42
│   │
│   └── STATE
│       ├── NAME: name
│       └── VALUE: Chaos
│
├── LOGIC
│   ├── EXPRESSION: integer > 0
│   └── CONSTANT
│       └── VALUE: integer is positive
│
└── EXECUTE: execute
```

Each AST node contains:

```text
ASTNode
├── type
├── value
├── data_type
├── children
├── child_count
└── child_capacity
```

Children are stored dynamically, allowing AST nodes to contain an arbitrary number of child nodes.

The AST is therefore both the parser's output and the runtime's input.

## 6. AST Node Types

Chaos v1 defines node types for its supported language constructs.

These include:

```text
AST_PROGRAM
AST_REGISTER
AST_STATE_DECLARATION
AST_STATE_NAME
AST_STATE_VALUE
AST_DATA_TYPE
AST_DATA_ITEMS
AST_LOGIC
AST_EXPRESSION
AST_CONSTANT
AST_IF
AST_ELSE_IF
AST_ELSE
AST_CONTRACT_CALL
AST_RESULT
AST_TERMINATE
AST_DATA_STRUCTURE_OPERATION
AST_PUSH
AST_POP
AST_TRANSITION
AST_CONTEXT
AST_RULE
AST_EXECUTE
```

Not every AST node represents a complex runtime operation.

Some nodes primarily preserve the syntactic structure of the source language. Runtime semantics are defined separately by the runtime layer.

## 7. Runtime

The runtime is implemented in:

```text
src/runtime.c
include/runtime.h
```

Its responsibility is to execute the AST.

Conceptually:

```text
          AST
           │
           ▼
     ┌───────────┐
     │  Runtime  │
     └─────┬─────┘
           │
           ▼
   Runtime State Store
```

The runtime traverses the program and dispatches behavior based on AST node types.

For example:

```text
AST_REGISTER
     │
     ▼
Create RuntimeState objects
     │
     ▼
Add states to RuntimeStateStore
```

A data-structure operation follows a similar path:

```text
AST_DATA_STRUCTURE_OPERATION
            │
            ▼
       Find state
            │
            ▼
    Execute PUSH / POP
            │
            ▼
     Update state value
```

Runtime behavior is deliberately separate from AST construction.

The AST describes what the program contains. The runtime determines what those constructs do.

## 8. Runtime State

Runtime state is implemented in:

```text
src/runtime_state.c
include/runtime_state.h
```

The runtime state system stores values created and modified during execution.

The central structure is:

```text
RuntimeStateStore
└── RuntimeState
    ├── name
    ├── value
    └── next
```

The store maintains a linked list of registered states.

Conceptually:

```text
RuntimeStateStore
      │
      ▼
┌──────────────┐
│ State        │
│ integer      │
│ number: 42   │
└──────┬───────┘
       │
       ▼
┌──────────────┐
│ State        │
│ name         │
│ string       │
│ Chaos        │
└──────┬───────┘
       │
       ▼
      NULL
```

State names must be unique within the store.

`runtime_state_find()` is used by the runtime to locate a state by name.

## 9. Runtime Values

A `RuntimeState` contains a `RuntimeValue`.

A runtime value can represent:

```text
number
string
expression
list
queue
stack
branch
```

The representation is broadly:

```text
RuntimeValue
├── type
├── scalar
├── items
├── item_count
├── item_capacity
├── branch_root
└── branch_count
```

Scalar values use the `scalar` field.

Collections use dynamically allocated `items`.

Branches use a tree rooted at `branch_root`.

This allows one state abstraction to represent multiple kinds of runtime data.

## 10. Collections

Lists, queues, and stacks share an array-based storage representation.

```text
RuntimeValue
│
├── items
├── item_count
└── item_capacity
```

Capacity grows dynamically as elements are added.

The semantic difference comes from how operations access the array.

```text
List
└── push → append
    pop  → remove first element

Queue
└── push → append
    pop  → remove first element

Stack
└── push → append
    pop  → remove last element
```

The v1 runtime therefore uses the same underlying storage mechanism while applying different removal semantics.

## 11. Branches

Branches are represented internally as binary search trees.

```text
             50
           /    \
         25      75
        /  \
      10    30
```

Each node contains:

```text
RuntimeBranchNode
├── value
├── left
└── right
```

Insertion uses string comparison.

Duplicate values are rejected.

The runtime also provides branch membership testing and in-order printing.

For example:

```text
{
    10,
    25,
    30,
    50,
    75
}
```

The current v1 implementation treats branch values as strings. This is an intentional limitation of the initial runtime.

## 12. Execution Flow

A complete Chaos program follows this sequence:

```text
                .chaos file
                    │
                    ▼
              Read source
                    │
                    ▼
                  Lexer
                    │
                    ▼
               TokenList
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
          ┌─────────┴─────────┐
          ▼                   ▼
   AST execution       State lookup/update
                              │
                              ▼
                    RuntimeStateStore
```

During execution, state declarations establish the initial runtime environment.

Logic statements then operate against that environment.

Finally, the state store contains the resulting runtime state.

## 13. AST and Runtime Separation

One of the important architectural decisions in Chaos v1 is keeping the AST and runtime state separate.

The AST represents the program:

```text
AST
│
├── What states exist?
├── What operations are requested?
├── What conditions are present?
└── What execution structure exists?
```

The runtime state represents the current data:

```text
RuntimeStateStore
│
├── integer = 42
├── name = "Chaos"
├── fruits = {...}
└── history = {...}
```

Executing an operation therefore does not require modifying the AST.

Instead:

```text
AST operation
      │
      ▼
Runtime
      │
      ▼
RuntimeStateStore
      │
      ▼
Updated state
```

This separation provides a foundation for future features such as more sophisticated expression evaluation, state transitions, and alternative execution mechanisms.

## 14. Error Boundaries

Chaos v1 has errors at several stages.

Lexical errors occur while source text is converted into tokens.

Parser errors occur when the token sequence does not conform to the expected syntax.

Runtime errors occur when a syntactically valid construct cannot be executed correctly.

For example:

```text
Source
  │
  ├── invalid character ──────► Lexer error
  │
  ├── invalid structure ──────► Parser error
  │
  └── invalid runtime state ──► Runtime error
```

Keeping these errors associated with their respective stages makes debugging the implementation simpler.

## 15. Module Responsibilities

The v1 source tree is intentionally small.

| Module            | Responsibility                                    |
| ----------------- | ------------------------------------------------- |
| `main.c`          | Program entry point and pipeline orchestration    |
| `lexer.c`         | Source-to-token conversion                        |
| `parser.c`        | Token-to-AST conversion                           |
| `ast.c`           | AST creation, manipulation, printing, and cleanup |
| `runtime.c`       | AST execution and runtime behavior                |
| `runtime_state.c` | Runtime state and data-structure storage          |

Their relationships can be summarized as:

```text
main.c
  │
  ├── lexer.c
  │      │
  │      ▼
  │   TokenList
  │      │
  │      ▼
  ├── parser.c
  │      │
  │      ▼
  │    ast.c
  │      │
  │      ▼
  └── runtime.c
          │
          ▼
   runtime_state.c
```

## 16. Design Philosophy

Chaos v1 deliberately avoids unnecessary implementation complexity.

The architecture favors:

* small modules
* explicit data structures
* direct control flow
* manual memory management
* clear separation of responsibilities
* understandable runtime behavior
* minimal dependencies

The objective is not to build the fastest interpreter possible.

The objective is to establish a comprehensible language implementation that can be extended in later versions.

## 17. V1 Architectural Boundary

Chaos v1 ends at runtime execution and mutable state management.

It does not currently include:

```text
Source
  │
  ▼
Lexer
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
State Store
```

followed by optimization, bytecode generation, JIT compilation, or a virtual machine.

Those would represent substantially different architectural layers and are outside the scope of the current version.

The v1 architecture is therefore intentionally modest: a source program is transformed into a structured representation and then interpreted directly against a runtime state store.