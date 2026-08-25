# Chaos v1 Transitions

Transitions represent movement from one computational state or context reference to another.

In v1, transitions are parsed and represented in the AST.

## Syntax

A transition appears inside logic:

```chaos
transition ('none');
```

The transition reference may be a string or identifier:

```chaos
transition (next-state);
```

## AST Shape

The parser stores a transition as:

```text
TRANSITION: none
```

## Runtime Boundary

The v1 runtime recognizes transition nodes during logic execution and preserves them without changing runtime state.

This makes transitions part of the completed v1 language vocabulary while keeping active transition machinery outside the current runtime mutation surface.

## Relationship To State

The `RuntimeStateStore` remains the place where live state is held.

Transitions do not duplicate state in v1. They are structural markers in the AST that can be inspected and extended by later runtime behavior.
