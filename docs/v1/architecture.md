# Chaos v1 Architecture

Chaos v1 is implemented as a direct language pipeline in C.

```text
.chaos source
  -> lexer
  -> tokens
  -> parser
  -> AST
  -> runtime
  -> RuntimeStateStore
```

## Source

Chaos source files use the `.chaos` extension.

The v1 source model is built from named computational structures:

```text
register
state
logic
constant
list
queue
stack
branch
transition
context
rule
execute
```

There is no `block` construct in Chaos v1.

## Lexer

The lexer converts source text into tokens.

It recognizes:

* Keywords
* Identifiers
* Numbers
* Single-quoted strings
* Brace-delimited expressions
* Operator symbols
* Parentheses
* Colons
* Commas
* Semicolons
* Equals signs

Expressions are tokenized as complete expression regions. Nested braces are tracked while lexing:

```chaos
{x + {y * 2}}
```

## Parser

The parser consumes the token stream and builds an AST.

At the top level, the parser accepts:

```text
REGISTER
LOGIC
EXECUTE
```

Within those structures, the parser recognizes state declarations, collection declarations, constants, collection operation groups, conditional forms, transitions, contexts, rules, contract calls, results, and termination markers.

The AST preserves the source-level computational structure so the runtime can dispatch by node type.

## AST

The AST is made of `ASTNode` values.

Each node has:

* A node type
* An optional string value
* An optional data-structure type
* Child nodes

Common v1 node shapes include:

```text
PROGRAM
  REGISTER
    STATE
      NAME
      VALUE
  LOGIC
    EXPRESSION
    CONSTANT
    DATA STRUCTURE OPERATION
      TYPE
      PUSH
      POP
  EXECUTE
```

The AST is also used to represent parsed constructs whose runtime behavior is intentionally minimal in v1, including transitions, contexts/rules, conditional branches, and contract calls.

## Runtime

The runtime walks the AST and executes supported node types.

The v1 runtime performs these operations:

* Creates runtime states from register declarations
* Infers scalar number and string values
* Creates list, queue, stack, and branch values from declared state types
* Initializes collection contents
* Resolves collection states by name
* Validates collection operation types
* Executes `push`
* Executes `pop`
* Prints constants during logic execution
* Reports top-level `execute`
* Prints final runtime state

The runtime keeps program structure and live values separate. The AST describes the program; the `RuntimeStateStore` contains the current state produced by execution.

## RuntimeStateStore

`RuntimeStateStore` is a linked list of `RuntimeState` values.

Each state contains:

```text
name
value
next
```

Each runtime value contains a type plus either scalar storage or collection storage.

Scalar runtime types:

```text
number
string
expression
```

Collection runtime types:

```text
list
queue
stack
branch
```

Collection storage uses an expandable array of string items.

## Execution Boundary

Chaos v1 is a functional core language and runtime. Its strongest runtime behavior is state registration plus collection mutation.

Several constructs are parsed and represented in the AST but do not yet mutate runtime state:

* `if`
* `else if`
* `else`
* Contract calls
* `result`
* `terminate`
* `transition`
* `context`
* `rule`

This boundary is part of v1's architecture: the parser accepts the core language vocabulary, and the runtime executes the stable state and collection semantics.
