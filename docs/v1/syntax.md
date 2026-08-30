# Chaos v1 Syntax

## Overview

Chaos programs are written as plain-text `.chaos` files.

Chaos v1 uses a small declarative syntax built around state registration, logic blocks, data-structure operations, conditions, transitions, and execution.

A typical program has three main parts:

```text
register ...
logic ...
execute ...
```

A minimal program can be:

```chaos id="s8r5kj"
register:
    state: integer = 42;

logic integer > 0;
    constant: integer is positive;

execute;
```

The lexer converts this source into tokens, and the parser converts those tokens into an AST.

## Keywords

Chaos v1 recognizes the following language keywords:

```text id="u5ml6b"
register
state
logic
constant
if
else
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
result
terminate
```

These keywords have special meaning to the lexer and parser.

## Identifiers

Identifiers are names used for states and other references.

Examples:

```chaos id="zdd0lm"
integer
decimal
name
fruits
waiting
history
tree
```

An identifier can be used to refer to runtime state:

```chaos id="6b9q3e"
integer > 0
```

State names must be unique within a runtime state store.

## Values

Chaos v1 supports several forms of values.

### Numbers

Numeric values can be written directly:

```chaos id="c0rh78"
42
3.14159
-10
```

Numbers are represented by the runtime as numeric values when a state is created.

### Strings

Strings are enclosed in quotation marks:

```chaos id="x7ps2e"
"Chaos"
"hello world"
"apple"
```

The lexer removes the surrounding quotation marks before the value reaches later stages.

### Identifiers

An identifier can also appear as a value:

```chaos id="8d7g8w"
integer
```

This is preserved as a state value/reference by the parser. Runtime interpretation depends on the construct in which it appears.

### Expressions

Expressions are represented as expression tokens and AST expression nodes.

For example:

```chaos id="0fz7kj"
{integer + 8}
```

or in contexts where the lexer recognizes the expression directly:

```text id="f8q9vp"
integer + 8
```

Expressions are preserved by the parser as `AST_EXPRESSION` nodes.

The v1 runtime provides limited expression semantics. Expression support should therefore be considered one of the boundaries of the current version.

## State Registration

States are declared using:

```chaos id="o8f4h2"
state: name = value
```

For example:

```chaos id="g9n3cl"
state: integer = 42
state: decimal = 3.14159
state: name = "Zia"
```

A state declaration produces an AST structure similar to:

```text id="j6byf5"
STATE
├── NAME: integer
└── VALUE: 42
```

The runtime converts the declaration into a `RuntimeState`.

## Data Structure State

Collection states specify their type after the state name:

```chaos id="4bq1up"
state: fruits, list = ...
state: waiting, queue = ...
state: history, stack = ...
state: tree, branch = ...
```

The supported collection types are:

```text id="1x4a4m"
list
queue
stack
branch
```

For example:

```chaos id="n0jj7p"
register("everything"):
    state: fruits, list = "apple", "banana", "blueberry",
    state: waiting, queue = "first", "second", "third",
    state: history, stack = "older", "old",
    state: tree, branch = "50", "25", "75";
```

The parser represents the declared type with an `AST_DATA_TYPE` node.

## Register

The `register` construct groups state declarations.

It may optionally have a name:

```chaos id="p2w6cr"
register("everything"):
    state: integer = 42;
```

A register without a name is also syntactically possible:

```chaos id="n4gq6v"
register:
    state: integer = 42;
```

The parser creates an `AST_REGISTER` node.

Its child nodes are state declarations.

Conceptually:

```text id="0f8x7y"
REGISTER
├── STATE
├── STATE
└── STATE
```

The register itself is primarily a structural grouping mechanism in v1.

## Logic

Logic blocks begin with `logic` followed by a condition:

```chaos id="v3r6tq"
logic integer > 0;
```

The condition becomes the first child of the `AST_LOGIC` node.

Statements inside the logic block follow the condition.

For example:

```chaos id="qj3y2e"
logic integer > 0;
    constant: integer is positive;
```

The logic block ends when the parser encounters `execute` or the end of the token stream.

## Constants

Constants are declared inside a logic block:

```chaos id="e9d7k1"
constant: integer is positive;
```

The text after `constant:` is stored as the constant's value.

The resulting AST has this structure:

```text id="u6w0yk"
CONSTANT
└── VALUE: integer is positive
```

At runtime, constants are currently printed when executed.

They do not create a separate mutable runtime state.

## Data Structure Operations

Data structures can be modified inside a logic block.

The general form is:

```chaos id="b4p8mz"
list name
    (push value)
    (pop),
```

The supported operations are:

```text id="h6o4yy"
push
pop
```

### Push

`push` adds a value to a collection:

```chaos id="7h9x6s"
list fruits
    (push "strawberry"),
```

The parser produces:

```text id="5s5g6r"
DATA STRUCTURE OPERATION: fruits
├── TYPE: list
└── PUSH
    └── VALUE: strawberry
```

### Pop

`pop` removes a value from a collection:

```chaos id="g5x0jv"
queue waiting
    (pop),
```

The parser produces:

```text id="g3h1qp"
DATA STRUCTURE OPERATION: waiting
├── TYPE: queue
└── POP
```

The runtime determines which element is removed according to the collection type.

## Multiple Operations

Multiple operations can be applied to the same collection:

```chaos id="j8w5kc"
list fruits
    (push "strawberry")
    (push "raspberry")
    (pop),
```

The operations are represented as multiple children of the data-structure operation node:

```text id="6k9y2w"
DATA STRUCTURE OPERATION: fruits
├── TYPE: list
├── PUSH
│   └── VALUE: strawberry
├── PUSH
│   └── VALUE: raspberry
└── POP
```

Operations are executed in the order in which they appear.

## Conditional Statements

Chaos v1 supports:

```text id="k2g8vy"
if
else if
else
```

The general form is:

```chaos id="1x6qv8"
if condition, operation
```

For example:

```chaos id="y8m2tq"
if integer > 0, (contract)
```

The exact operation following the condition is parsed according to the operation grammar.

An `if` node can contain:

```text id="1h3c6q"
IF
├── condition
├── operation
├── ELSE IF
│   ├── condition
│   └── operation
└── ELSE
    └── operation
```

This allows an entire conditional chain to be represented by one root `AST_IF` node.

## Else If

An `else if` follows an `if`:

```chaos id="q8j5w0"
if condition, operation
else if other_condition, operation
```

The parser attaches the `AST_ELSE_IF` node to the original `AST_IF`.

## Else

An `else` provides a fallback operation:

```chaos id="c1z5ph"
if condition, operation
else operation
```

The parser represents it as an `AST_ELSE` child of the `AST_IF`.

## Contracts

Contract calls use parentheses containing a contract reference:

```chaos id="m4k9cz"
("contract")
```

A contract reference may be a string or identifier.

Arguments can also be supplied:

```chaos id="x3v8qn"
("contract" value)
```

The parser creates an `AST_CONTRACT_CALL`.

Contract arguments may be represented by:

```text id="0q5j7r"
AST_RESULT
AST_EXPRESSION
```

depending on the token encountered.

Contract syntax exists in v1's parser and AST model, but the runtime semantics are intentionally limited.

## Results

The `result` keyword can appear inside contract calls.

For example:

```chaos id="q9m5ht"
("contract" result)
```

The parser represents this as an `AST_RESULT` node.

Results are part of the v1 language representation but are not a complete general-purpose return-value system.

## Terminate

The `terminate` keyword represents termination:

```chaos id="k5v2z8"
terminate
```

The parser creates:

```text id="e4c9x1"
TERMINATE
```

The current runtime provides limited semantics for this construct.

## Transitions

Transitions use the form:

```chaos id="b7r4wx"
transition("reference")
```

The reference can be a string or identifier.

For example:

```chaos id="q0x5ns"
transition("none")
```

The parser creates:

```text id="4p6m1k"
TRANSITION: none
```

In v1, transitions are represented and executed at a basic level rather than providing a complete state-machine implementation.

## Context and Rules

A context can be declared using:

```chaos id="v6y3qk"
context condition, rule(condition)
```

For example:

```chaos id="h8r2mj"
context integer > 0, rule(integer is positive)
```

The resulting AST structure is:

```text id="1j4c7n"
CONTEXT
├── EXPRESSION: integer > 0
└── RULE
    └── EXPRESSION: integer is positive
```

Context and rule syntax is part of the v1 parser and AST model.

Their runtime behavior remains limited in v1.

## Execute

The `execute` statement marks execution:

```chaos id="w3k9tp"
execute;
```

The parser creates:

```text id="z7m4qc"
EXECUTE: execute
```

The runtime responds by executing the parsed program and printing the execution marker.

## Complete Example

The following demonstrates the major constructs used by the v1 examples:

```chaos id="5j7r2c"
register("everything"):
    state: integer = 42,
    state: decimal = 3.14159,
    state: name = "Zia",
    state: expression = {integer + 8},
    state: fruits, list = "apple", "banana", "blueberry",
    state: waiting, queue = "first", "second", "third",
    state: history, stack = "older", "old",
    state: tree, branch = "50", "25", "75", "10", "30";

logic integer > 0;
    constant: integer is positive;

    list fruits
        (push "strawberry")
        (push "raspberry")
        (pop),

    queue waiting
        (push "fourth")
        (pop),

    stack history
        (push "newest")
        (pop),

    branch tree
        (push "60")
        (push "5"),

    transition("none");

execute;
```

The resulting AST is conceptually:

```text id="x8h3qm"
PROGRAM
├── REGISTER
│   ├── STATE
│   ├── STATE
│   ├── STATE
│   ├── STATE
│   ├── STATE
│   ├── STATE
│   ├── STATE
│   └── STATE
│
├── LOGIC
│   ├── EXPRESSION
│   ├── CONSTANT
│   ├── DATA STRUCTURE OPERATION
│   ├── DATA STRUCTURE OPERATION
│   ├── DATA STRUCTURE OPERATION
│   ├── DATA STRUCTURE OPERATION
│   └── TRANSITION
│
└── EXECUTE
```

## Statement Terminators

Chaos v1 uses semicolons to terminate major statements.

Examples:

```chaos id="y1q6pz"
constant: integer is positive;

transition("none");

execute;
```

Commas are used to separate elements in several constructs, including state declarations and data-structure operation statements.

## Syntax and Runtime

The parser determines whether source conforms to the v1 syntax.

The runtime determines what successfully parsed constructs actually do.

This distinction is important.

A construct can therefore be:

```text id="f2q6zy"
Lexically recognized
        │
        ▼
Successfully parsed
        │
        ▼
Represented in AST
        │
        ▼
Have limited runtime semantics
```

This applies particularly to constructs such as contracts, contexts, rules, and some expression behavior.

## V1 Syntax Boundary

Chaos v1 intentionally has no syntax for several features normally found in general-purpose languages.

There is currently no general syntax for:

* user-defined functions
* modules
* imports
* classes
* conventional loops
* pattern matching
* a full static type system
* a standard library
* arbitrary function definitions

These features are outside the scope of the v1 syntax.

## Syntax Design

Chaos v1 uses a deliberately compact grammar.

Its syntax is intended to make several concepts explicit:

```text id="q6t3mw"
state       → persistent runtime data
logic       → conditions and operations
collection  → structured runtime data
transition  → state-flow declaration
execute     → execution boundary
```

The parser converts these constructs into an AST, allowing the runtime to process them independently of the original source representation.