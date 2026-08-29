<div align="center"> <pre> 
  ░██████  ░██     ░██    ░███      ░██████     ░██████   
 ░██   ░██ ░██     ░██   ░██░██    ░██   ░██   ░██   ░██  
░██        ░██     ░██  ░██  ░██  ░██     ░██ ░██         
░██        ░██████████ ░█████████ ░██     ░██  ░████████  
░██        ░██     ░██ ░██    ░██ ░██     ░██         ░██ 
 ░██   ░██ ░██     ░██ ░██    ░██  ░██   ░██   ░██   ░██  
  ░██████  ░██     ░██ ░██    ░██   ░██████     ░██████   
</pre>

[![Typing SVG](https://readme-typing-svg.demolab.com?font=Fira+Code&size=20&pause=1000&color=00FF9C&center=true&vCenter=true&width=440&lines=please+save+me.;actually+i+love+building+this+thing.;praise+chaos+the+primordial.)](https://git.io/typing-svg)

</div>

<div align="center">

![C](https://img.shields.io/badge/C-000000?style=for-the-badge&logo=c&logoColor=white)

</div>

---

# Chaos

Chaos is a custom programming language implemented in C.

Chaos v1 is built around a small set of computational structures:

* `register`
* `state`
* `logic`
* `constant`
* `list`
* `queue`
* `stack`
* `branch`
* `transition`
* `context`
* `rule`
* `execute`

The language is intentionally small. A `.chaos` program is lexed into tokens, parsed into an AST, executed by the runtime, and reflected in the `RuntimeStateStore`.

```text
.chaos source
  -> lexer
  -> tokens
  -> parser
  -> AST
  -> runtime
  -> RuntimeStateStore
```

## Chaos v1

Chaos v1 provides a functional core language and runtime.

The implemented runtime supports:

* Register declarations
* Scalar states
* Number and string state values
* Brace-delimited expression values stored as runtime state
* List, queue, stack, and branch state declarations
* Collection initialization
* Collection type validation
* `push` and `pop` operations
* FIFO queue popping
* LIFO stack popping
* Ordered list storage
* Branch storage using the v1 collection runtime
* Constant parsing and runtime reporting
* Parsed logic, transition, context, rule, contract-call, result, terminate, and execute structures
* Runtime execution over the AST
* Final Runtime State Store inspection

The lexer, parser, AST, runtime, and state store are the source of truth for v1 behavior.

## Example

```chaos
register ('everything'):

    state: integer = 42,
    state: name = 'Zia',
    state: fruits, list = {'apple', 'banana'};

logic {x > 0};

    constant: x < y;

    list fruits
        (push 'strawberry')
        (pop),

execute
```

Running the interpreter prints the parsed AST, executes supported runtime operations, and displays the final Runtime State Store.

```sh
make
./chaos examples/all_v1.chaos
```

## Documentation

The v1 documentation lives in `docs/v1/`:

* `architecture.md`
* `syntax.md`
* `logic.md`
* `runtime-and-state-systems.md`
* `data-structures.md`
* `contracts.md`
* `transitions.md`
* `contexts-and-rules.md`
* `examples.md`
* `limitations.md`

## Version Scope

Chaos v1 focuses on the core language and runtime.

Chaos v2 is reserved for mathematical functionality. CLI tooling, package management, editors, and ecosystem integrations belong to a later product stage rather than the v2 language priority.

## Why Chaos Exists

Chaos explores whether programming can be expressed through computational concepts instead of through a large surface area of unrelated syntax conventions.

It is a language experiment, a runtime experiment, and a way to learn how programming languages work from the inside.
