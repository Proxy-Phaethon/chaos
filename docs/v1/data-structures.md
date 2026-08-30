# Chaos v1 Data Structures

## Overview

Chaos v1 provides four built-in data structures:

```text
list
queue
stack
branch
```

The first three are dynamic, array-backed collections. `branch` is implemented as a binary search tree.

Data structures are declared as runtime states and can be modified during logic execution.

```text
State
  │
  └── RuntimeValue
        ├── list
        ├── queue
        ├── stack
        └── branch
```

All collection elements are represented as strings internally in v1.

---

## Declaration

A data structure is declared by specifying its type after the state name:

```chaos id="b7m2q1"
state: fruits, list = 'apple', 'banana', 'blueberry'
state: waiting, queue = 'first', 'second', 'third'
state: history, stack = 'older', 'old'
state: tree, branch = '50', '25', '75', '10', '30'
```

The general form is:

```text
state: name, type = values
```

where `type` is one of:

```text
list
queue
stack
branch
```

---

# List

A list is an ordered, dynamically sized collection.

Example:

```chaos id="h8k4p2"
state: fruits, list = 'apple', 'banana', 'blueberry'
```

The runtime representation is:

```text
RuntimeValue
├── type
├── items
├── item_count
└── item_capacity
```

`items` is a dynamically allocated array of strings.

---

## List Ordering

List elements retain their insertion order.

Given:

```text
{apple, banana, blueberry}
```

then:

```chaos id="q5v1n8"
list fruits (push "strawberry")
```

produces:

```text
{apple, banana, blueberry, strawberry}
```

A list `pop` removes the first element.

```chaos id="z2c6r9"
list fruits (pop)
```

produces:

```text
POP fruits: apple
```

and leaves:

```text
{banana, blueberry, strawberry}
```

---

## List Operations

### Push

```chaos id="s4x7m2"
list fruits (push "raspberry")
```

The value is appended to the end of the list.

### Pop

```chaos id="p9k3d6"
list fruits (pop)
```

The first element is removed.

Therefore, v1 list operations behave similarly to a simple ordered collection with front removal.

---

# Queue

A queue is an ordered collection using first-in, first-out behavior.

Example:

```chaos id="c7m1v4"
state: waiting, queue = 'first', 'second', 'third'
```

Its initial contents are:

```text
{first, second, third}
```

---

## FIFO Behavior

When an element is pushed:

```chaos id="n3q8w5"
queue waiting (push "fourth")
```

the queue becomes:

```text
{first, second, third, fourth}
```

When an element is popped:

```chaos id="r6t2y9"
queue waiting (pop)
```

the first element is removed:

```text
POP waiting: first
```

The remaining queue becomes:

```text
{second, third, fourth}
```

This gives the queue its first-in, first-out behavior.

---

## Queue Operations

### Push

```chaos id="u4f8j1"
queue waiting (push "fourth")
```

Adds an element to the back.

### Pop

```chaos id="e7b3k5"
queue waiting (pop)
```

Removes an element from the front.

Internally, the v1 runtime shifts the remaining elements toward the beginning of the backing array after a front removal.

---

# Stack

A stack is an ordered collection using last-in, first-out behavior.

Example:

```chaos id="m2r7c4"
state: history, stack = 'older', 'old'
```

Initial contents:

```text
{older, old}
```

---

## LIFO Behavior

A value is pushed onto the end:

```chaos id="w8n5q2"
stack history (push "newest")
```

producing:

```text
{older, old, newest}
```

A subsequent pop removes `newest`:

```chaos id="k6p1s9"
stack history (pop)
```

Runtime output:

```text
POP history: newest
```

The stack returns to:

```text
{older, old}
```

---

## Stack Operations

### Push

```chaos id="d4h7x3"
stack history (push "newest")
```

Adds an element to the top of the stack.

### Pop

```chaos id="v9m2q6"
stack history (pop)
```

Removes the most recently inserted element.

The runtime implements this by accessing the final element in the backing array.

---

# Branch

A branch is a binary search tree.

Unlike `list`, `queue`, and `stack`, it is not represented by a dynamic array.

Example:

```chaos id="a5k8r1"
state: tree, branch = '50', '25', '75', '10', '30'
```

The resulting structure is conceptually:

```text
          50
         /  \
       25    75
      /  \
    10    30
```

Each node contains:

```text
RuntimeBranchNode
├── value
├── left
└── right
```

The branch state stores a pointer to its root node and the number of nodes currently contained in the tree.

---

## Branch Ordering

Branch insertion uses lexical string comparison.

For every new value:

```text
value < current node
        │
        └── insert into left subtree

value > current node
        │
        └── insert into right subtree

value == current node
        │
        └── do not insert
```

For example:

```chaos id="x3c7n2"
branch tree (push "60")
```

compares `"60"` against the existing nodes and places it according to the branch ordering rules.

---

## Duplicate Values

Duplicate values are not inserted.

If the branch already contains:

```text
50
```

then:

```chaos id="y8p4m6"
branch tree (push "50")
```

does not create another node.

The branch count is only incremented when a new node is successfully inserted.

---

## Branch Lookup

The runtime can determine whether a branch contains a particular value.

Conceptually:

```text
             50
            /  \
          25    75
         /  \
       10    30
```

Searching for `30` follows:

```text
50
 ↓
25
 ↓
30
```

Searching for a value that does not exist eventually reaches an empty subtree.

---

## Branch Output

Branches are printed using an in-order traversal.

The traversal order is:

```text
left
  ↓
node
  ↓
right
```

For:

```text
          50
         /  \
       25    75
      /  \
    10    30
```

the traversal produces:

```text
{10, 25, 30, 50, 75}
```

The printed representation therefore represents the branch's ordered contents rather than its exact tree shape.

---

# Collection Storage

Lists, queues, and stacks share the same underlying storage model.

```text
                 RuntimeValue
                      │
              ┌───────┴────────┐
              │                │
          item_count       item_capacity
              │                │
              └───────┬────────┘
                      ▼
                  char **items
                      │
          ┌───────────┼───────────┐
          ▼           ▼           ▼
       "apple"     "banana"   "blueberry"
```

The backing array grows dynamically.

The initial capacity is four elements.

When the array becomes full, the runtime doubles its capacity:

```text
4 → 8 → 16 → 32 → ...
```

This keeps insertion simple while allowing collections to grow at runtime.

---

# Push and Pop Model

The four structures do not all support identical semantics.

| Structure | Push        | Pop                        | Ordering                    |
| --------- | ----------- | -------------------------- | --------------------------- |
| List      | Append      | Remove first               | Insertion order             |
| Queue     | Append      | Remove first               | FIFO                        |
| Stack     | Append      | Remove last                | LIFO                        |
| Branch    | Insert node | Not implemented as removal | Binary-search-tree ordering |

The shared `push`/`pop` syntax therefore maps onto different runtime behavior depending on the structure.

---

# Empty Collections

Attempting to pop from an empty list, queue, or stack does not produce an element.

Internally, the runtime returns no value for an empty collection.

The collection remains valid and its item count remains zero.

---

# Type Checking

Data-structure operations verify that the referenced state is actually the expected collection type.

For example, a scalar state such as:

```chaos id="j1v7c9"
state: integer = 42
```

cannot be manipulated as a list:

```chaos id="f5q2m8"
list integer (push "value")
```

The runtime rejects operations performed on incompatible state types.

This prevents collection operations from modifying scalar storage incorrectly.

---

# State Mutation

Data structures are mutable.

For example:

```chaos id="n8c4x2"
register "example":
    state: fruits, list = 'apple', 'banana';

logic true;
    list fruits (push "orange");
```

After execution, the runtime state contains:

```text
fruits [list] = {apple, banana, orange}
```

The AST describes the operation, but the runtime state store contains the resulting value.

---

# Memory Management

Collection elements are dynamically allocated strings.

When a value is pushed, the runtime creates a copy of the supplied value.

When an element is removed with `pop`, ownership of the allocated string is returned to the caller.

The caller is responsible for releasing the returned value.

When a collection is destroyed, all remaining elements are freed.

Branches use recursive cleanup:

```text
branch
  │
  ├── left subtree
  │    ├── left
  │    └── right
  │
  └── right subtree
       ├── left
       └── right
```

Every branch node and its stored value is released when the branch state is destroyed.

---

# Complexity

The current v1 implementations prioritize simplicity over optimization.

For the array-backed structures:

| Operation |  List | Queue | Stack |
| --------- | ----: | ----: | ----: |
| Push      | O(1)* | O(1)* | O(1)* |
| Pop       |  O(n) |  O(n) |  O(1) |
| Lookup    |  O(n) |  O(n) |  O(n) |

`*` Push is amortized O(1) because the backing array occasionally needs to grow.

Queue and list pops are O(n) because removing the first element shifts the remaining elements.

For branches:

| Operation |  Average | Worst case |
| --------- | -------: | ---------: |
| Insert    | O(log n) |       O(n) |
| Search    | O(log n) |       O(n) |

The worst case occurs when the binary search tree becomes highly unbalanced.

Chaos v1 does not implement a self-balancing tree.

---

# Current v1 Limitations

The data-structure system intentionally has a limited scope.

Currently:

* collection elements are represented as strings
* lists do not provide indexed access syntax
* lists do not provide explicit sorting operations
* queues use element shifting rather than a circular buffer
* stacks have no explicit `peek` operation
* branches do not support deletion
* branches are not self-balancing
* branches use lexical comparison rather than numeric comparison
* collection operations are limited to `push` and `pop`
* there is no generic user-defined data-structure system

These are deliberate boundaries for v1 rather than unfinished requirements for the initial release.

---

# Example

The following program exercises all four structures:

```chaos id="c8m5r1"
register "structures":
    state: fruits, list = 'apple', 'banana', 'blueberry',
    state: waiting, queue = 'first', 'second', 'third',
    state: history, stack = 'older', 'old',
    state: tree, branch = '50', '25', '75', '10', '30';

logic true;
    list fruits (push "strawberry") (push "raspberry") (pop);

    queue waiting (push "fourth") (pop);

    stack history (push "newest") (pop);

    branch tree (push "60") (push "5");

execute
```

After execution, the resulting state is conceptually:

```text
fruits [list] = {banana, blueberry, strawberry, raspberry}
waiting [queue] = {second, third, fourth}
history [stack] = {older, old}
tree [branch] = {50, 25, 75, 10, 30, 60, 5}
```

The branch's printed representation reflects its traversal behavior in the actual runtime.

---

# Summary

Chaos v1 provides four built-in mutable data structures:

```text
LIST
 │
 └── ordered dynamic collection

QUEUE
 │
 └── FIFO collection

STACK
 │
 └── LIFO collection

BRANCH
 │
 └── binary search tree
```

All four are exposed through the same state system but use different runtime representations and semantics.

The implementation favors transparency and simplicity, providing a small foundation that can be extended with richer collection operations and additional structures in later versions.