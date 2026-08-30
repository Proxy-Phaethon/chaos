# Chaos v1 Limitations

## Overview

Chaos v1 is the first functional version of the language.

Its purpose is to establish the core language pipeline:

```text
Source
  ↓
Lexer
  ↓
Tokens
  ↓
Parser
  ↓
AST
  ↓
Runtime
  ↓
Runtime State Store
```

v1 is intentionally limited. It provides a working foundation for future versions rather than attempting to implement every planned language feature.

---

## Language Limitations

Chaos v1 has a small language surface.

The currently implemented concepts are:

* `register`
* `state`
* `logic`
* `constant`
* `if`
* `else if`
* `else`
* `execute`
* Lists
* Queues
* Stacks
* Branches

Features that are planned for later versions are not considered part of the v1 language.

---

## Expression Limitations

Expressions are supported, but expression evaluation is intentionally simple.

v1 supports basic expressions involving:

* Numeric literals
* State references
* Arithmetic operators
* Comparison operators
* Parentheses
* Unary operators where implemented by the evaluator

Expressions are not a general-purpose expression language.

The evaluator does not currently provide:

* Functions
* Function calls
* Complex type coercion
* User-defined operators
* Advanced mathematical operations
* Boolean algebra beyond the supported condition semantics
* String manipulation expressions

Expression handling will be expanded in future versions.

---

## Type System Limitations

Chaos v1 has only minimal type handling.

Runtime values currently include:

```text
number
string
expression
list
queue
stack
branch
```

The runtime primarily distinguishes values by their declared runtime type.

There is no comprehensive static type-checking system.

Consequently, some invalid operations are detected only during runtime.

---

## State Limitations

States are stored in a runtime state store.

Each state has:

* A name
* A runtime value
* A runtime value type

State lookup is performed by name.

v1 does not provide:

* State scopes
* Local variables
* Nested scopes
* State namespaces
* Immutable state semantics
* State persistence between program executions
* Serialization of runtime state

The state store exists only for the lifetime of the running Chaos program.

---

## Collection Limitations

Chaos v1 provides four collection types:

```text
list
queue
stack
branch
```

Lists, queues, and stacks are internally represented using dynamically allocated arrays of strings.

As a result, collection elements are currently represented primarily as textual values rather than strongly typed runtime values.

### List

The list implementation supports insertion and removal operations.

It does not currently provide:

* Indexed access syntax
* Sorting
* Searching
* Slicing
* Iteration
* Higher-order operations

### Queue

The queue follows FIFO semantics.

It currently supports:

* `push`
* `pop`

It does not provide advanced queue operations.

### Stack

The stack follows LIFO semantics.

It currently supports:

* `push`
* `pop`

It does not provide:

* Random access
* Stack inspection operations
* Advanced stack manipulation

### Branch

The branch structure is implemented as a binary search tree.

It currently supports:

* Insertion
* Membership checking
* In-order printing

It does not currently provide:

* Node deletion
* Tree balancing
* Rotations
* Traversal selection
* Height calculation
* Minimum/maximum operations
* Generic tree structures

---

## Control-Flow Limitations

Chaos v1 provides basic conditional execution:

```text
if
else if
else
```

Conditional branches can execute supported operations based on evaluated expressions.

However, v1 does not provide:

* `while` loops
* `for` loops
* Iteration over collections
* `break`
* `continue`
* Functions
* Recursion
* User-defined procedures
* Pattern matching
* Switch-style control flow

This means that v1 programs are intentionally small and linear.

---

## Runtime Limitations

The runtime executes the AST directly.

There is no intermediate representation or bytecode layer.

The runtime does not currently provide:

* Bytecode compilation
* Virtual-machine execution
* JIT compilation
* Optimization passes
* Garbage collection
* Automatic memory management
* Runtime profiling
* Debugger support

Memory management is performed explicitly through the C implementation.

---

## Error Handling Limitations

Chaos v1 provides basic parser and runtime error reporting.

Parser errors include source location information where available.

Runtime errors are reported when operations cannot be performed, such as attempting to reference an unknown state.

There is currently no language-level error handling.

Chaos v1 does not provide:

* `try`
* `catch`
* `throw`
* Recoverable runtime exceptions
* User-defined error types
* Structured error propagation

---

## Input and Output Limitations

Runtime output is currently primarily diagnostic.

Operations such as constants and collection `pop` produce textual output.

There is no general-purpose input/output system.

v1 does not provide:

* User input
* File I/O
* Network I/O
* Standard library I/O functions
* Formatted output expressions
* User-defined output operations

---

## Functionality Not Included in v1

Several concepts may appear in the broader design of Chaos but are deliberately outside the v1 implementation.

These include:

* Contracts
* Transitions
* Advanced context/rule systems
* Functions
* Modules
* Imports
* User-defined types
* Advanced mathematics
* External libraries
* Concurrency
* Parallel execution
* Persistent state
* Networking

These should not be interpreted as partially implemented v1 features.

They belong to future development.

---

## Performance Limitations

Performance is not a primary goal of v1.

The implementation prioritizes:

1. Correctness
2. Simplicity
3. Understandable C code
4. A complete source-to-runtime pipeline

rather than aggressive optimization.

Some operations therefore use straightforward implementations.

For example, queue removal shifts subsequent elements in the underlying array rather than maintaining a more sophisticated circular-buffer structure.

Similarly, the branch structure is an ordinary binary search tree and is not self-balancing.

---

## Memory Management

Chaos is implemented in C and therefore manages memory manually.

The implementation dynamically allocates:

* AST nodes
* Token data
* Runtime states
* Runtime values
* Collection elements
* Branch nodes

The v1 implementation includes explicit cleanup functions for these structures.

However, memory safety depends on correct ownership and cleanup throughout the implementation.

There is no garbage collector.

---

## Parser Limitations

The parser is intentionally lightweight.

It constructs the AST directly from the token stream and performs basic syntactic validation.

It does not currently provide:

* Sophisticated error recovery
* Detailed syntax diagnostics
* A formal grammar implementation
* AST optimization
* Semantic analysis
* Static type checking

A malformed program may therefore produce a relatively simple parser error rather than a detailed compiler-style diagnostic.

---

## Standard Library Limitations

Chaos v1 has effectively no standard library.

The language runtime provides only the functionality required by the implemented core language features.

There are currently no built-in libraries for:

* Mathematics
* Strings
* Files
* Networking
* Random numbers
* Time
* Operating-system interaction

These are candidates for later versions.

---

## Platform Limitations

The current implementation is written in standard C and uses a conventional command-line build process.

The project is intended to remain portable, but v1 has only been practically tested on the development environment used for the project.

Platform-specific compatibility has not been exhaustively validated.

---

## Project Scope

Chaos v1 should be understood as a functional language prototype rather than a production programming language.

Its primary achievement is establishing a complete execution pipeline:

```text
Chaos Source
     │
     ▼
   Lexer
     │
     ▼
 Token Stream
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

The v1 implementation demonstrates that source code can be tokenized, parsed into an AST, executed by a runtime, and represented through mutable runtime state.

That foundation is the basis for subsequent versions of Chaos.

---

## Version Boundary

The following rule defines the v1 boundary:

> If a feature is not implemented and executable in the v1 runtime, it is not a v1 feature.

Planned functionality should therefore be documented separately rather than presented as existing functionality.

This keeps the v1 specification synchronized with the actual implementation.