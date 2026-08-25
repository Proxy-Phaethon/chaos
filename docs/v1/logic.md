# Chaos v1 Logic

`logic` is the v1 structure that groups active computation.

The parser represents logic as an `AST_LOGIC` node with an initial expression followed by logic children.

## Form

A logic structure starts with an expression and a semicolon:

```chaos
logic {x > 0};
```

After this header, the parser attaches following logic statements until it reaches top-level `execute` or end of file.

```chaos
logic {x > 0};

    constant: x < y;

    list fruits
        (push 'strawberry'),

execute
```

## Logic Children

Chaos v1 parses these structures inside logic:

```text
constant
list operation
queue operation
stack operation
branch operation
transition
context/rule
if
else if
else
contract call
result
terminate
```

## Runtime Behavior

The v1 runtime executes the stable runtime operations inside logic:

* Constants are printed.
* Data-structure operation groups mutate runtime state.
* Parsed conditionals, transitions, contexts/rules, contract calls, results, and termination markers are preserved in the AST without additional state mutation.

This means logic is both the container for executable v1 collection behavior and the AST home for the rest of the core computational vocabulary.

## Conditions

Logic conditions are parsed as expression nodes:

```chaos
logic {health > 0};
```

Conditional branches are also parsed:

```chaos
if {health > 50}, ('healthy')
else if {health > 0}, ('injured')
else terminate
```

In v1, these conditions are represented structurally. Expression evaluation and branch selection belong to the mathematical/runtime expansion after the v1 core.

## Constants

Constants are local logic entries:

```chaos
constant: x < y;
```

The parser stores the expression text. The runtime reports it during logic execution:

```text
CONSTANT: x < y
```

## Collection Operations

Collection operations are the main mutating operations executed from logic in v1:

```chaos
queue waiting
    (push 'fourth')
    (pop),
```

The runtime resolves `waiting`, checks that it is a queue, pushes `fourth`, pops the oldest item, and updates the `RuntimeStateStore`.
