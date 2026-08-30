# Chaos v1 Logic and Control Flow

## Overview

Chaos v1 provides a small logic system for evaluating conditions and executing operations based on those conditions.

Logic blocks are declared using the `logic` keyword. A logic block contains:

1. A condition or question.
2. One or more operations.
3. Optional conditional branches.
4. Optional constants.
5. Optional data-structure operations.
6. Optional transitions and contexts.

A logic block is executed when the program reaches `execute`.

The general structure is:

```chaos
logic <condition>;
    <operation>
    <operation>
    ...
execute
```

For example:

```chaos
logic integer > 0;
    constant: integer is positive
execute
```

---

## Conditions

A condition is an expression evaluated by the runtime.

Conditions may reference registered states.

```chaos
register:
    state: integer = 42;

logic integer > 0;
    constant: integer is positive
execute
```

The runtime resolves `integer` through the runtime state store and evaluates:

```text
integer > 0
```

If the condition evaluates to true, the operations associated with the condition are executed.

If a condition cannot be evaluated, the runtime reports an error.

For example, referencing an unknown state:

```chaos
logic unknown > 0;
    constant: this will fail
execute
```

produces a runtime error because `unknown` does not exist in the state store.

---

## Expressions

Expressions are represented by Chaos as expression values.

An expression may contain:

* Numeric values
* State identifiers
* Arithmetic operators
* Comparison operators

Examples:

```chaos
integer + 8
```

```chaos
integer > 0
```

```chaos
integer + decimal
```

Expression evaluation is performed by the runtime rather than by the parser.

The parser is responsible for representing the expression in the AST. The runtime is responsible for resolving state references and determining the result.

---

## Constants

Constants are messages produced when a condition succeeds.

The syntax is:

```chaos
constant: <value>;
```

For example:

```chaos
logic integer > 0;
    constant: integer is positive
execute
```

When the condition evaluates to true, the runtime prints:

```text
CONSTANT: integer is positive
```

Constants do not currently create mutable runtime state.

They function as observable output from a logic block.

---

## Conditional Execution

Chaos v1 supports:

```chaos
if
else if
else
```

Conditional branches are represented in the AST as a single conditional chain.

The basic form is:

```chaos
if <condition>, <operation>
```

For example:

```chaos
logic integer > 0;
    if integer > 10, constant: large
execute
```

The runtime evaluates the condition and executes the associated operation when the condition is true.

---

## Else-If

An `else if` branch provides an alternative condition.

Conceptually:

```chaos
if condition_a, operation_a
else if condition_b, operation_b
else operation_c
```

Only the first successful branch is executed.

For example:

```chaos
logic integer > 0;
    if integer > 100, constant: very large
    else if integer > 50, constant: large
    else constant: positive
execute
```

The conditional chain is evaluated from top to bottom.

---

## Else

An `else` branch executes when none of the preceding conditions succeed.

Example:

```chaos
logic integer > 0;
    if integer > 100, constant: very large
    else constant: not very large
execute
```

The `else` branch does not have a condition.

---

## Data-Structure Operations

Logic blocks can manipulate registered data structures.

Supported operations are:

```chaos
push
pop
```

For example:

```chaos
register:
    state: fruits, list = 'apple', 'banana';

logic true;
    list fruits (push 'orange')
    list fruits (pop)
execute
```

The runtime performs the operations when the logic block executes.

For collections, the runtime maintains the underlying collection state in the runtime state store.

---

## Push

`push` adds a value to a collection.

Example:

```chaos
list fruits (push 'strawberry')
```

The value is added to the collection.

For example:

```text
fruits [list] = {apple, banana}
```

becomes:

```text
fruits [list] = {apple, banana, strawberry}
```

---

## Pop

`pop` removes and returns an element according to the collection's semantics.

For lists, `pop` removes the first element.

For queues, `pop` removes the first element.

For stacks, `pop` removes the most recently added element.

For example:

```chaos
stack history (push 'newest') (pop)
```

produces output similar to:

```text
POP history: newest
```

The removed value is no longer present in the runtime state.

---

## Queue and Stack Semantics

Chaos v1 gives queues and stacks different removal behavior.

A queue follows FIFO behavior:

```text
first in
↓
first out
```

A stack follows LIFO behavior:

```text
last in
↓
first out
```

For example, given:

```text
queue = {first, second, third}
```

then:

```chaos
queue waiting (pop)
```

removes:

```text
first
```

Given:

```text
stack = {older, old, newest}
```

then:

```chaos
stack history (pop)
```

removes:

```text
newest
```

---

## Branch Operations

The `branch` data structure is implemented as a binary search tree.

Values are inserted according to their comparison with existing nodes.

For example:

```text
        50
       /  \
     25    75
    /  \
  10    30
```

Additional values can be inserted through:

```chaos
branch tree (push 60) (push 5)
```

The runtime maintains the tree structure internally.

Branch values are printed using an in-order traversal.

---

## Transitions

Chaos v1 recognizes transition statements:

```chaos
transition(none)
```

Transitions are represented in the AST and executed by the runtime.

In v1, transitions are observational rather than a complete state-machine system. The runtime reports the transition:

```text
TRANSITION: none
```

More advanced transition semantics are outside the scope of v1.

---

## Context and Rules

Chaos v1 also supports the syntax for contexts and rules.

A context associates an expression with a rule:

```chaos
context <expression>, rule(<expression>)
```

The parser represents this relationship in the AST.

Context and rule functionality is intentionally limited in v1. They establish the language structure required for future rule-based execution but do not constitute a complete inference or policy engine.

---

## Execution Order

A Chaos program is processed in several stages:

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
State Store
```

At runtime, registered states are created first.

Logic blocks then operate on the state store.

Finally, the `execute` statement marks execution of the program's operational logic.

For a program such as:

```chaos
register:
    state: integer = 42;

logic integer > 0;
    constant: integer is positive
execute
```

the conceptual execution flow is:

```text
Create integer
      ↓
Store integer = 42
      ↓
Evaluate integer > 0
      ↓
Condition succeeds
      ↓
Execute constant
      ↓
Print result
```

---

## Runtime Errors

Logic evaluation may fail when a required state does not exist or an expression cannot be evaluated.

For example:

```chaos
logic missing_state > 0;
    constant: unreachable
execute
```

The runtime reports the missing state rather than silently inventing a value.

Runtime errors are reported to standard error.

A failed runtime operation does not become a valid state mutation.

---

## v1 Scope

The logic system in Chaos v1 is intentionally small.

Implemented functionality includes:

* Logic blocks
* Conditions
* State references
* Expression evaluation
* Constants
* `if`
* `else if`
* `else`
* Collection `push`
* Collection `pop`
* Queue behavior
* Stack behavior
* Branch insertion
* Transitions
* Basic context/rule representation
* Runtime error reporting

The system is designed as the execution foundation for later versions of Chaos.

---

## Limitations

Chaos v1 does not attempt to provide a complete general-purpose control-flow language.

In particular:

* Expression evaluation is intentionally limited.
* Conditions are limited to the expression forms supported by the runtime.
* Constants are output operations rather than persistent constant declarations.
* Context and rule semantics are limited.
* Transitions do not implement a complete state-machine model.
* There is no loop construct in v1.
* There is no function system in v1.
* There is no user-defined procedure system in v1.
* There is no exception-handling system.
* Type checking is minimal.
* Collections primarily operate on string representations of values.
* The branch structure is a binary search tree rather than a general-purpose tree.
* Runtime behavior is intentionally simple rather than optimized.

These limitations define the boundary of the v1 implementation rather than errors in the language architecture.

---

## Summary

The Chaos v1 logic system connects expressions to runtime operations.

Expressions provide conditions.

Conditions determine whether operations execute.

Operations can produce output, modify collections, or trigger runtime actions.

The runtime state store provides the mutable state against which expressions and operations are evaluated.

The resulting model is:

```text
             ┌──────────────┐
             │ Logic Block  │
             └──────┬───────┘
                    │
                    ▼
             ┌──────────────┐
             │   Condition  │
             └──────┬───────┘
                    │
              ┌─────┴─────┐
              │           │
            true         false
              │           │
              ▼           ▼
        ┌──────────┐  ┌──────────┐
        │ Operation│  │ Else/Else│
        └────┬─────┘  │   If     │
             │        └────┬─────┘
             ▼             │
       ┌────────────┐      │
       │Runtime State│◄────┘
       │    Store    │
       └────────────┘
```

Chaos v1 therefore provides the first complete connection between the language's declarative state model and executable runtime behavior.