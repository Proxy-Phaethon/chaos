# Chaos v1 Contracts

Contracts are the v1 representation for reusable executable operations.

The current parser recognizes contract calls inside logic operations and conditional branches. A contract call is stored as an `AST_CONTRACT_CALL` node.

## Contract Call Syntax

A contract call is parenthesized:

```chaos
('contract-name')
```

Arguments may follow the contract name:

```chaos
('compare' x y)
```

The contract name may be a string or identifier. Arguments may be identifiers, strings, numbers, or `result`.

## Conditional Use

Contract calls can appear as operations attached to parsed conditional forms:

```chaos
if {x > 10}, ('large' x)
else ('small' x)
```

The parser stores the condition and the contract call in the AST.

## AST Shape

A parsed contract call has this shape:

```text
CONTRACT: contract-name
  EXPRESSION: argument
  RESULT: result
```

## Runtime Boundary

Chaos v1 preserves contract calls structurally. The runtime does not resolve contract definitions or invoke contract bodies.

This keeps contracts in the core v1 vocabulary while leaving their execution model outside the current state and collection runtime.
