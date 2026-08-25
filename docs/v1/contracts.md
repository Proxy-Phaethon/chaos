# Chaos Contracts

Contracts provide reusable executable behavior in Chaos.

A contract separates an operation from the location where it is invoked.

## 1. Contract Model

A contract consists conceptually of:

```text
CONTRACT
│
├── NAME
├── PARAMETERS
├── BODY
└── RESULT
```

Contracts can therefore be invoked from other executable constructs.

## 2. Contract Invocation

A contract is executed through an execution construct.

```text
execute
    contract
```

The runtime resolves the contract and executes its body.

```mermaid
flowchart LR
    A["execute"] --> B["Resolve Contract"]
    B --> C["Bind Arguments"]
    C --> D["Execute Contract Body"]
    D --> E["Produce Result"]
    E --> F["Return to Caller"]
```

## 3. Parameters

Contracts may accept values supplied by the caller.

Conceptually:

```text
contract operation
    parameter A
    parameter B
```

At execution time:

```text
argument A → parameter A
argument B → parameter B
```

Parameter binding occurs within the contract's execution environment.

## 4. Contract State

Contracts operate against the runtime environment while maintaining their own execution context.

```text
Caller
  │
  ▼
Contract
  │
  ├── parameters
  ├── operations
  └── result
  │
  ▼
Caller
```

State explicitly exposed to the contract can therefore be used during execution.

## 5. Results

A contract may produce a result.

```text
Contract
   │
   ▼
Result
   │
   ▼
Caller state
```

Results can subsequently be consumed by other runtime operations.

## 6. Reusability

The primary purpose of contracts is to prevent repeated executable logic.

Instead of encoding the same operations repeatedly:

```text
operation
operation
operation
```

the behavior can be defined once:

```text
contract operation
```

and invoked wherever required.

## 7. Contract Errors

The runtime validates contract execution.

Possible errors include:

```text
Unknown contract
Invalid argument count
Invalid argument type
Invalid parameter binding
Invalid result
```

An invalid invocation terminates the operation with a runtime error rather than executing partially.