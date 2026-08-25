# Chaos v1 Examples

This document provides representative Chaos v1 programs using syntax accepted by the current parser.

## Basic State

```chaos
register ('main'):

    state: x = 42,
    state: name = 'Zia';

execute
```

The runtime creates two states and prints them in the final state store.

## Expressions As State

```chaos
register ('main'):

    state: x = 10,
    state: expression = {x + 5};

execute
```

The expression is preserved as state text:

```text
expression [string] = x + 5
```

## List

```chaos
register ('main'):

    state: fruits, list = {'apple', 'banana'};

logic {x > 0};

    list fruits
        (push 'blueberry')
        (push 'strawberry'),

execute
```

Final state:

```text
fruits [list] = {apple, banana, blueberry, strawberry}
```

## Queue

```chaos
register ('main'):

    state: waiting, queue = {'first', 'second', 'third'};

logic {x > 0};

    queue waiting
        (push 'fourth')
        (pop),

execute
```

The popped value is:

```text
POP waiting: first
```

Final queue:

```text
waiting [queue] = {second, third, fourth}
```

## Stack

```chaos
register ('main'):

    state: history, stack = {'older', 'old'};

logic {x > 0};

    stack history
        (push 'new')
        (pop),

execute
```

The popped value is:

```text
POP history: new
```

Final stack:

```text
history [stack] = {older, old}
```

## Branch

```chaos
register ('main'):

    state: tree, branch = {'50', '25', '75'};

logic {x > 0};

    branch tree
        (push '10')
        (push '30'),

execute
```

Final branch state:

```text
tree [branch] = {50, 25, 75, 10, 30}
```

## Constants

```chaos
register ('main'):

    state: x = 42;

logic {x > 0};

    constant: x < y;

execute
```

Runtime output includes:

```text
CONSTANT: x < y
```

## Context And Rule

```chaos
register ('main'):

    state: x = 1;

logic {x > 0};

    context {x + 1 = 2},
    rule ('x not equal 0');

execute
```

The parser stores the context and nested rule in the AST.

## Complete v1 Demonstration

The repository includes `examples/all_v1.chaos`, which demonstrates the v1 vocabulary and runtime functionality in one program.

Run it with:

```sh
make
./chaos examples/all_v1.chaos
```
