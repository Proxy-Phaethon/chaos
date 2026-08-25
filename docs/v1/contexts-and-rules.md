# Chaos v1 Contexts and Rules

Contexts and rules are v1 structures for describing the environment and constraints around logic.

The parser recognizes a context paired with a rule and stores both in the AST.

## Syntax

A context/rule pair appears inside logic:

```chaos
context {x + 1 = 2},
rule ('x not equal 0');
```

The context expression may be a brace-delimited expression, string, identifier, number, or `result`.

The rule expression is placed inside parentheses after `rule`.

## AST Shape

The parser stores the pair as:

```text
CONTEXT
  EXPRESSION: x + 1 = 2
  RULE
    EXPRESSION: x not equal 0
```

## Runtime Boundary

Chaos v1 preserves contexts and rules structurally. The runtime recognizes these nodes during logic execution and leaves the state store unchanged.

This lets v1 programs carry contextual information in their AST while the executable runtime remains focused on registers, states, and collection operations.

## Role In v1

Contexts and rules are part of the completed v1 core vocabulary:

* `context` describes the computational situation.
* `rule` describes a condition associated with that situation.
* The AST keeps the relationship between the two explicit.

They do not perform condition evaluation or action dispatch in the current runtime.
