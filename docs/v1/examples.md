# Chaos Examples

This document provides representative Chaos v1 programs.

## 1. Basic State

```chaos
register main
    state: x = 42,
    state: name = 'Zia',
```

The runtime creates two states:

```text
main
├── x = 42
└── name = 'Zia'
```

## 2. Expressions

```chaos
register main
    state: x = 10,
    state: result = {x + 5},
```

The runtime resolves `x` and evaluates the expression.

Result:

```text
x      = 10
result = 15
```

## 3. List

```chaos
register main
    state: fruits, list = {
        'apple',
        'banana'
    },

    list fruits
        (push 'blueberry')
        (push 'strawberry'),
```

Result:

```text
fruits = [
    apple,
    banana,
    blueberry,
    strawberry
]
```

## 4. Queue

```chaos
register main
    state: waiting, queue = {
        'first',
        'second',
        'third'
    },

    queue waiting
        (push 'fourth')
        (pop),
```

The popped value is:

```text
first
```

The resulting queue is:

```text
second
third
fourth
```

## 5. Stack

```chaos
register main
    state: history, stack = {
        'older',
        'old'
    },

    stack history
        (push 'new')
        (pop),
```

The popped value is:

```text
new
```

## 6. Branch

```chaos
register main
    state: tree, branch = {
        '50',
        '25',
        '75',
        '10',
        '30'
    },
```

The runtime represents the values as a branch structure.

Conceptually:

```text
        50
       /  \
     25    75
    /  \
  10    30
```

## 7. Conditional Execution

A logic construct can use runtime state to determine execution:

```chaos
register main
    state: health = 75,

    logic
        if {health > 50}
            execute healthy,
```

The condition evaluates against the current value of `health`.

## 8. Contextual Behavior

A context groups state-dependent behavior:

```text
context
│
├── rule
│   └── condition
│       └── execute
│
└── rule
    └── condition
        └── execute
```

The runtime evaluates the rules against the active state.

## 9. Complete v1 Demonstration

The repository includes `all_v1.chaos`, which demonstrates the v1 vocabulary and runtime functionality in a single program.

Its purpose is not to represent an idiomatic Chaos application, but to provide a feature-level integration test for the language.

The progression is:

```text
Declarations
     │
     ▼
Data Structures
     │
     ▼
Operations
     │
     ▼
Logic
     │
     ▼
Contexts / Rules
     │
     ▼
Transitions
     │
     ▼
Execution
```

This provides a compact demonstration that the language constructs can coexist inside the same runtime.