# Chaos v1 Data Structures

Chaos v1 supports four collection types:

```text
list
queue
stack
branch
```

Collection states are declared inside a register:

```chaos
register ('collections'):

    state: fruits, list = {'apple', 'banana'},
    state: waiting, queue = {'first', 'second'},
    state: history, stack = {'older', 'old'},
    state: tree, branch = {'50', '25', '75'};
```

The runtime stores collection items as strings in an expandable array.

## Initialization

Collection initializers are written as brace-delimited expressions:

```chaos
state: fruits, list = {'apple', 'banana', 'blueberry'}
```

During runtime registration, the initializer text is split on commas. Surrounding whitespace and single quotes are removed before each item is pushed into the collection.

## Operations

Collection operations appear inside `logic`:

```chaos
logic {x > 0};

    list fruits
        (push 'strawberry')
        (pop),

execute
```

Each operation group identifies both the collection type and the target state name. The runtime checks that the target state's actual type matches the operation type before executing.

## Push

`push` appends an item to the collection's storage:

```chaos
list fruits
    (push 'orange'),
```

After the operation, `orange` is stored after the existing items.

## Pop

`pop` removes and prints one item.

Queue pop uses FIFO behavior:

```text
[first, second, third] -> pop -> first
```

Stack pop uses LIFO behavior:

```text
[older, old, newest] -> pop -> newest
```

List pop removes the oldest stored item in v1:

```text
[apple, banana] -> pop -> apple
```

Branch pop also removes the oldest stored item in v1:

```text
[50, 25, 75] -> pop -> 50
```

## Branch

`branch` is a distinct v1 runtime type. It represents tree-oriented data at the language level while using the same collection storage as the other v1 data structures.

Branch values can be initialized, pushed, popped, printed, and type-checked:

```chaos
branch tree
    (push '60')
    (push '5'),
```

## Type Safety

The runtime rejects operations that target the wrong collection type.

For example, this is invalid when `waiting` was declared as a queue:

```chaos
stack waiting
    (pop),
```

The runtime reports a type mismatch instead of mutating the state.

## Empty Collections

Popping an empty collection is a runtime error.

This applies to all v1 collection types.
