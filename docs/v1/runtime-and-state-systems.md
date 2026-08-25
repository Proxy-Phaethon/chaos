# Chaos v1 Runtime and State System

The Chaos v1 runtime turns an AST into live program state.

Its central component is the `RuntimeStateStore`.

```text
AST
  -> Runtime
  -> RuntimeStateStore
```

## Runtime

The runtime owns a state store and dispatches over top-level AST nodes:

```text
REGISTER -> create states
LOGIC    -> execute supported logic children
EXECUTE  -> report execution
```

Unsupported top-level nodes are ignored by the dispatcher.

## Runtime State Store

The state store is a linked list of runtime states.

Each runtime state contains:

```text
name
value
next
```

States are looked up by name when operations need to mutate an existing value.

## Runtime Values

Runtime values have an explicit type.

Scalar types:

```text
number
string
expression
```

Collection types:

```text
list
queue
stack
branch
```

Scalar states store one string value. Collection states store an array of string items.

## Register Execution

Register execution creates runtime states from parsed state declarations:

```chaos
register ('main'):

    state: score = 100,
    state: player = 'Zia',
    state: fruits, list = {'apple', 'banana'};
```

Scalar state types are inferred:

```text
100   -> number
Zia   -> string
x + 1 -> string in the current runtime representation
```

Declared collection types become matching runtime collection values.

## Collection Initialization

Collection initializers are stored by the parser as expression text:

```chaos
{'apple', 'banana'}
```

The runtime initializes the collection by splitting this text on commas, trimming whitespace, removing surrounding single quotes, and pushing each item into the collection storage.

## State Mutation

Collection operation groups mutate runtime state:

```chaos
list fruits
    (push 'strawberry')
    (push 'raspberry'),
```

Execution order is source order inside the operation group.

## Pop Semantics

Queue pop removes the oldest item:

```text
[first, second, third] -> first
```

Stack pop removes the newest item:

```text
[older, old, newest] -> newest
```

List and branch pop remove the oldest item in v1.

## Type Validation

Before executing a collection operation, the runtime checks that the source operation type matches the target state's runtime type.

```chaos
queue waiting
    (pop),
```

This succeeds only when `waiting` is a queue. A mismatch produces a runtime error.

## Runtime Errors

The v1 runtime reports errors for:

* Missing state names
* Failed state creation
* Unknown collection states
* Collection type mismatches
* Invalid push values
* Popping empty collections
* Failed collection initialization

Execution stops when a runtime operation fails.

## Runtime Output

The current executable prints:

* A parse success message
* The AST
* Runtime output from constants, pops, and execute
* The final state store

This makes the interpreter useful as both a v1 runtime and an inspection tool for the language pipeline.
