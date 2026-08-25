# Chaos v1 Syntax

This document describes the syntax accepted by the Chaos v1 lexer and parser.

## Program Structure

A Chaos program is a sequence of top-level structures:

```text
register
logic
execute
```

The parser accepts these structures in source order and stores them under a `PROGRAM` AST node.

## Values

Chaos v1 recognizes four value forms.

Numbers:

```chaos
42
3.14159
```

Strings:

```chaos
'Chaos'
```

Identifiers:

```chaos
current-state
```

Expressions:

```chaos
{x + 1 = 2}
```

Expressions are stored as expression text. The v1 runtime preserves them as state values or AST conditions.

## Registers

A register contains state declarations and ends with a semicolon.

```chaos
register ('main'):

    state: integer = 42,
    state: name = 'Chaos';
```

The register name is optional:

```chaos
register:

    state: x = 10;
```

## States

A state associates a name with a value:

```chaos
state: x = 42
```

A state may also declare a collection type:

```chaos
state: fruits, list = {'apple', 'banana'}
```

Inside a register, state declarations are separated by commas. The final state is followed by the register's semicolon.

## Collections

Chaos v1 supports four collection declarations:

```text
list
queue
stack
branch
```

Collection states use brace-delimited expression syntax for their initial contents:

```chaos
state: fruits, list = {'apple', 'banana', 'blueberry'}
state: waiting, queue = {'first', 'second', 'third'}
state: history, stack = {'older', 'old'}
state: tree, branch = {'50', '25', '75'}
```

The runtime initializes collection contents by splitting the expression text on commas and trimming surrounding single quotes.

## Collection Operations

Collection operation groups appear inside `logic`.

```chaos
list fruits
    (push 'strawberry')
    (push 'raspberry'),
```

Each operation group begins with the collection type, then the target state name, then one or more parenthesized operations. The group ends with a comma.

Supported operations:

```text
push
pop
```

Examples:

```chaos
queue waiting
    (push 'fourth')
    (pop),

stack history
    (push 'newest')
    (pop),
```

## Logic

A logic structure starts with an initial condition and a semicolon:

```chaos
logic {x > 0};
```

After that, the parser attaches following logic statements until it reaches `execute` or end of file.

Supported logic children:

```text
constant
list operation
queue operation
stack operation
branch operation
transition
context/rule
if / else if / else
```

## Constants

A constant stores the tokens between `constant:` and `;` as a constant expression.

```chaos
constant: x < y;
```

The v1 runtime prints constants during logic execution.

## Conditional Forms

Conditional forms are parsed into the AST.

```chaos
if {x > 10}, ('contract-name' result)
else if {x > 5}, terminate
else ('fallback')
```

The operation after the comma is either a parenthesized contract call or `terminate`.

## Contract Calls

Contract calls are parsed as parenthesized operations:

```chaos
('compare' x y)
```

The first value is the contract name. Additional identifiers, strings, numbers, and `result` tokens are stored as arguments.

## Transitions

Transitions are parsed inside logic:

```chaos
transition ('none');
```

The transition reference may be a string or identifier.

## Contexts and Rules

A context pairs an expression with one rule expression:

```chaos
context {x + 1 = 2},
rule ('x not equal 0');
```

The parser stores the context expression and the nested rule expression in the AST.

## Execute

`execute` is a top-level structure:

```chaos
execute
```

The v1 runtime reports `EXECUTE` when this node is reached.

## Complete Example

See `examples/all_v1.chaos` for a source file that exercises the v1 vocabulary and runtime behavior together.
