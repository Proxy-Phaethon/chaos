# Chaos v1 Runtime and State Systems

## Overview

The Chaos v1 runtime is responsible for taking the abstract syntax tree produced by the parser and executing the operations supported by the language.

The execution pipeline is:

```text
Chaos Source
     │
     ▼
   Lexer
     │
     ▼
 Token Stream
     │
     ▼
   Parser
     │
     ▼
    AST
     │
     ▼
  Runtime
     │
     ▼
Runtime State Store
```

The runtime does not operate directly on the original source code. By the time execution begins, the source has already been converted into an AST.

---

## Runtime Responsibilities

The v1 runtime is responsible for:

* creating runtime state from register declarations
* storing scalar values
* storing collections
* evaluating supported expressions
* evaluating logic conditions
* executing constants
* performing list operations
* performing queue operations
* performing stack operations
* performing branch operations
* executing supported conditional control flow
* reporting transitions
* reaching the program's execution point
* printing runtime state

The runtime is deliberately small. It provides the execution layer necessary for the v1 language without attempting to implement a complete virtual machine.

---

## Runtime State

Runtime state represents the mutable values available to a Chaos program during execution.

Each state has:

```text
RuntimeState
├── name
├── value
└── next
```

The `name` identifies the state.

The `value` contains its runtime value.

The `next` pointer connects the state to the next state in the runtime state store.

This creates a linked-list-based state store.

```text
RuntimeStateStore
       │
       ▼
   ┌─────────┐
   │ State A │
   └────┬────┘
        │
        ▼
   ┌─────────┐
   │ State B │
   └────┬────┘
        │
        ▼
   ┌─────────┐
   │ State C │
   └─────────┘
```

---

## RuntimeValue

Every `RuntimeState` owns a `RuntimeValue`.

A runtime value has one of the following types:

```text
number
string
expression
list
queue
stack
branch
```

The runtime representation is divided into scalar values and collection values.

Scalar values use:

```text
scalar
```

Collection values use:

```text
items
item_count
item_capacity
```

Branch values use:

```text
branch_root
branch_count
```

This allows the runtime to represent different types without requiring a separate state implementation for each one.

---

## Scalar State

Scalar values are stored as dynamically allocated strings.

For example:

```chaos id="9eg0hf"
state: integer = 42
state: decimal = 3.14159
state: name = "Zia"
```

The runtime stores the values internally as text while retaining their runtime type.

Conceptually:

```text
integer
├── type: number
└── value: "42"

decimal
├── type: number
└── value: "3.14159"

name
├── type: string
└── value: "Zia"
```

This keeps the v1 runtime implementation simple.

---

## Expression State

Expressions have their own runtime value type:

```text
RUNTIME_VALUE_EXPRESSION
```

For example:

```chaos id="q3d9e7"
state: expression = integer + 8
```

The expression is stored as text and evaluated by the runtime when necessary.

An expression may reference another runtime state:

```chaos id="h6z9m4"
state: integer = 42
state: expression = integer + 8
```

The runtime resolves `integer` through the runtime state store before performing the calculation.

---

## Runtime State Store

The runtime state store contains all registered runtime states.

States are inserted into the store with:

```c
runtime_state_store_add()
```

States can be retrieved by name with:

```c
runtime_state_find()
```

A lookup walks the linked list until a matching name is found.

```text
find("integer")

State A
   │
   ├── name = "tree"
   │
   ▼
State B
   │
   ├── name = "integer"  ← match
   │
   ▼
State C
```

State names must be unique within the store.

Attempting to add a state with an existing name fails.

---

## State Lifetime

Runtime states are dynamically allocated.

The general lifetime is:

```text
AST state declaration
        │
        ▼
RuntimeState creation
        │
        ▼
RuntimeStateStore
        │
        ▼
Runtime execution
        │
        ▼
Runtime state cleanup
```

When the state store is destroyed, each state is freed along with its associated runtime value.

Collection elements and branch nodes are also recursively freed.

---

## Lists

A Chaos list is represented as a dynamically sized array of strings.

```text
RuntimeValue
├── type: LIST
├── items
├── item_count
└── item_capacity
```

Capacity begins at a small initial value and grows when necessary.

The runtime doubles the capacity when the current storage is full.

```text
capacity = 4
       ↓
capacity = 8
       ↓
capacity = 16
       ↓
...
```

This provides simple dynamic storage without requiring the programmer to specify the list size beforehand.

---

## Queue

A queue uses the same underlying dynamic array representation as a list.

Its behavior differs in how elements are removed.

For example:

```text
queue = {first, second, third}
```

After:

```chaos id="x6h8uw"
queue waiting (pop)
```

the runtime removes:

```text
first
```

and the queue becomes:

```text
{second, third}
```

The v1 implementation removes the first element and shifts the remaining elements toward the beginning of the array.

---

## Stack

A stack also uses the dynamic array representation.

Unlike a queue, a stack removes the most recently inserted element.

For:

```text
stack = {older, old, newest}
```

the operation:

```chaos id="f1c7qv"
stack history (pop)
```

removes:

```text
newest
```

The remaining stack is:

```text
{older, old}
```

---

## Push

`push` appends an element to a list, queue, or stack.

For example:

```chaos id="t5x4bm"
list fruits (push "strawberry")
```

The runtime:

1. locates the state
2. verifies that it is a collection
3. ensures sufficient capacity
4. copies the value
5. appends it to the collection
6. increments the item count

The same operation is used for queues and stacks.

---

## Pop

`pop` removes an element from a list, queue, or stack.

For lists and queues, the first element is removed.

For stacks, the last element is removed.

The removed value is returned by the runtime and may be printed as part of runtime execution.

Example:

```text
POP fruits: apple
POP waiting: first
POP history: newest
```

An empty collection produces no valid value to remove.

---

## Branch

A `branch` is implemented as a binary search tree.

Its runtime representation is different from the array-backed collections.

```text
RuntimeValue
├── type: BRANCH
├── branch_root
└── branch_count
```

Each branch node contains:

```text
RuntimeBranchNode
├── value
├── left
└── right
```

Conceptually:

```text
          50
         /  \
       25    75
      /  \
    10    30
```

---

## Branch Insertion

Branch values are inserted according to lexical comparison.

For a new value:

```text
value < node
    → left

value > node
    → right

value == node
    → duplicate, do not insert
```

For example:

```chaos id="k7x1lq"
branch tree (push "60")
branch tree (push "5")
```

results in the new nodes being placed according to the branch's ordering rules.

Duplicate values are ignored.

The runtime tracks the number of successfully inserted nodes with `branch_count`.

---

## Branch Lookup

The runtime can search a branch for a value.

The lookup follows the binary-search-tree structure:

```text
             root
            /    \
       smaller   larger
```

At each node, the runtime compares the requested value with the current node and follows either the left or right subtree.

The search terminates when:

* the value is found, or
* a `NULL` child is reached.

---

## Branch Output

Branches are printed using an in-order traversal.

For example, an internal tree such as:

```text
          50
         /  \
       25    75
      /  \
    10    30
```

is printed by traversing:

```text
left → node → right
```

The runtime therefore produces:

```text
{10, 25, 30, 50, 75}
```

The tree's internal structure is not exposed by ordinary state printing.

---

## Runtime Execution

Execution begins with the AST's program node.

The runtime walks the program's children in order.

Conceptually:

```text
PROGRAM
   │
   ├── REGISTER
   │
   ├── LOGIC
   │
   ├── LOGIC
   │
   └── EXECUTE
```

The runtime processes each node according to its AST type.

Register nodes create runtime states.

Logic nodes execute their supported operations.

The `execute` node marks the end of the described program flow and produces the runtime execution output.

---

## Register Execution

A register contains one or more state declarations.

For each declaration, the runtime determines:

```text
state name
state type
state value
```

and creates the corresponding `RuntimeState`.

For example:

```chaos id="5uhhfl"
register "example":
    state: integer = 42,
    state: name = "Zia";
```

produces runtime state conceptually equivalent to:

```text
integer [number] = 42
name [string] = Zia
```

The register name itself does not become a runtime state.

---

## Logic Execution

A logic block has a condition followed by operations.

```chaos id="5nqf6k"
logic integer > 0;
    constant: integer is positive;
```

The runtime evaluates the condition.

If the condition is true, the operations inside the logic block execute.

If the condition is false, the operations are skipped.

Conceptually:

```text
             LOGIC
               │
               ▼
          evaluate condition
           /             \
        true             false
         │                 │
         ▼                 ▼
    execute body       skip body
```

---

## Conditional Execution

`if`, `else if`, and `else` nodes are represented in the AST as a single conditional chain.

The runtime evaluates the branches in order.

```text
IF
 │
 ├── condition
 ├── operation
 ├── ELSE IF
 │    ├── condition
 │    └── operation
 │
 └── ELSE
      └── operation
```

Once one branch executes, the remaining branches are skipped.

---

## Constants at Runtime

A constant currently represents runtime output.

For example:

```chaos id="6h2x1m"
constant: integer is positive;
```

produces:

```text
CONSTANT: integer is positive
```

Constants do not create immutable runtime variables in v1.

The keyword describes an operation in the AST rather than a separately stored constant object.

---

## Transitions at Runtime

Transitions are recognized by the parser and runtime.

For example:

```chaos id="9r8j4p"
transition("none");
```

produces runtime output:

```text
TRANSITION: none
```

The v1 implementation treats the transition as an executable reference/output operation.

It does not yet implement a complete graph-based state-transition system.

---

## Runtime Errors

The runtime reports errors when an operation cannot be executed.

Examples include:

```text
Runtime error: unknown state 'name'
```

and:

```text
Runtime error: could not evaluate expression
```

Other invalid operations include attempting to:

* push to a scalar state
* pop from a scalar state
* use an unknown state
* perform an operation on the wrong data-structure type
* evaluate an expression containing an unknown identifier

Runtime errors do not represent parser errors. Parser errors occur before execution, while runtime errors occur while executing an already parsed AST.

---

## State Store Output

After execution, the runtime can print the contents of the state store.

A scalar state is displayed as:

```text
integer [number] = 42
```

A collection is displayed as:

```text
fruits [list] = {banana, blueberry, strawberry}
```

A branch is displayed using its in-order representation:

```text
tree [branch] = {10, 25, 30, 50, 75}
```

The state store reflects mutations that occurred during runtime execution.

For example, after:

```chaos id="l0l5cx"
queue waiting (push "fourth")
queue waiting (pop)
```

the removed element is no longer present in the stored queue.

---

## Memory Management

Chaos v1 uses explicit C memory management.

The runtime dynamically allocates:

* runtime states
* runtime scalar strings
* collection storage
* collection elements
* branch nodes

Every allocation has a corresponding cleanup path.

Collection elements are individually freed before their containing array is freed.

Branch nodes are recursively freed from the tree.

The complete runtime state store is freed when execution finishes.

---

## Runtime Model

The complete v1 runtime model can be summarized as:

```text
                 ┌──────────────┐
                 │   AST Root   │
                 └──────┬───────┘
                        │
                        ▼
                ┌──────────────┐
                │   Runtime    │
                └──────┬───────┘
                       │
          ┌────────────┼────────────┐
          │            │            │
          ▼            ▼            ▼
      REGISTER       LOGIC       EXECUTE
          │            │
          ▼            ▼
   Create State    Evaluate
          │         Condition
          ▼            │
 RuntimeStateStore     ▼
                   Operations
                       │
          ┌────────────┼─────────────┐
          │            │             │
          ▼            ▼             ▼
       Scalar      Collections     Branch
                     │               │
                 list/queue/      BST nodes
                    stack
```

The runtime state store is therefore the central mutable component of Chaos v1.

The AST describes what should happen. The runtime interprets that structure, while the state store holds the values that change as execution proceeds.