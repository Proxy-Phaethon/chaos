# Chaos v1 Tests

The Chaos v1 test suite verifies the language at several levels.

```text
tests/
├── lexer/
├── parser/
├── runtime/
└── integration/
```

## Lexer Tests

Lexer tests verify that Chaos source code is correctly converted into tokens.

They cover:

* Keywords
* Identifiers
* Numbers
* Strings
* Expressions
* Operators
* Punctuation
* Data-structure keywords

## Parser Tests

Parser tests verify that valid token streams are converted into the expected AST structure.

They cover:

* State registration
* Scalar values
* Expressions
* Logic blocks
* Constants
* Conditional branches
* Data-structure operations
* Execute statements

## Runtime Tests

Runtime tests verify execution of parsed Chaos programs.

They cover:

* Runtime state creation
* State lookup
* Expression evaluation
* Conditional execution
* Constants
* List operations
* Queue operations
* Stack operations
* Branch insertion

## Integration Tests

Integration tests exercise the complete Chaos pipeline:

```text
Source
  ↓
Lexer
  ↓
Parser
  ↓
AST
  ↓
Runtime
  ↓
State Store
```

The primary integration test is:

```text
integration/all_v1.chaos
```

It combines the major features implemented in Chaos v1.

## Test Philosophy

The v1 tests prioritize correctness and clarity over exhaustive coverage or performance benchmarking.

Each test should exercise one identifiable language or runtime behavior whenever practical.

The integration test provides a broader regression check for the complete implementation.

## Running Tests

Individual test programs can be executed using the Chaos executable:

```bash
./chaos tests/runtime/states.chaos
```

The complete integration example can be run with:

```bash
./chaos tests/integration/all_v1.chaos
```

A future test runner can automate execution and expected-output checking.

## v1 Scope

The test suite only covers functionality implemented in Chaos v1.

Features planned for later versions should not be added to the v1 test suite until their implementations exist.