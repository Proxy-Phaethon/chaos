<div align="center"> <pre> 
 ██████╗██╗  ██╗ █████╗  ██████╗ ███████╗
██╔════╝██║  ██║██╔══██╗██╔═══██╗██╔════╝
██║     ███████║███████║██║   ██║███████╗
██║     ██╔══██║██╔══██║██║   ██║╚════██║
╚██████╗██║  ██║██║  ██║╚██████╔╝███████║
 ╚═════╝╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝ ╚══════╝
</pre>

[![Typing SVG](https://readme-typing-svg.demolab.com?font=Fira+Code&size=20&pause=1000&color=00FF9C&center=true&vCenter=true&width=440&lines=One+syntax%2C+every+language.;No+boilerplate.;No+generative+AI.;Just+architecture.)](https://git.io/typing-svg)

</div>

<div align="center">

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)

</div>

---

# Chaos

> A semantic software engineering engine.

Chaos is an experimental project exploring a language-independent representation of software.

Rather than treating programming languages as the source of truth, Chaos treats them as *targets*. Projects and code are first represented as semantic entities with defined relationships, dependencies, and rules. Those semantic models can then be generated, translated, validated, or modified through different interfaces.

The long-term goal is to simplify software development by separating **intent** from **implementation**.

---

## Current Status

Chaos is currently in active development.

Version 1 focuses on building the semantic engine and the project initialisation pipeline.

Current priorities include:

- Semantic architecture
- Project initialisation
- Manifest system
- Dependency engine
- Validation engine
- Project generation

Language translation and the Chaos programming language will be introduced after the semantic foundation is complete.

---

## Philosophy

Chaos is built around a small number of reusable concepts.

- Everything is an entity.
- Entities have properties.
- Entities are connected by relationships.
- Dependencies determine validity.
- Rules determine behaviour.

The engine understands these concepts rather than individual programming languages.

---

## Planned Commands

### `chaos initialize`

Creates a new project by constructing a semantic Project Manifest through a dependency-driven question system.

### `chaos write`

Creates semantic source code that can later be translated into supported programming languages.

### `chaos edit`

Modifies an existing Project Manifest while preserving compatible project structure and translating affected implementation where possible.

### `chaos run`

Executes a project using its semantic configuration rather than individual framework commands.

### Future Commands

- `chaos translate`
- `chaos doctor`
- `chaos build`
- `chaos test`

---

## Roadmap

### Version 1

- Semantic engine
- Project manifest
- Dependency resolver
- Validator
- `chaos initialize`
- Project generation

### Version 1.5

- `chaos edit`
- Project migration
- Manifest upgrades

### Version 2

- Chaos language
- Semantic parser
- Language generators
- Multi-language translation

---

## Project Status

Chaos is an experimental research project.

The architecture is expected to evolve significantly as the semantic model matures.

Early versions prioritise correctness, extensibility, and architectural clarity over feature count.

## Built with

Rust, chosen for safety, speed, and a single self-contained binary with
zero runtime dependencies for the end user. Also it just sounds cool asf.

## Note

This project is actually a result of my incredible laziness to memorize different syntax for all the languages we
have to use in our daily lives even as a simple web developer. Forget creating video games (my dreams of making a
cosy life simulator long trashed), I couldn't even be bothered to fix the website I'd created myself. This, of
course, wasn't due to my inability to 'fix' any bugs - I was simply insistent on the fact that if I am doing
programming, I must be able to do it like in the movies. Just open a terminal and start typing, like a genius. 

Chaos serves that purpose, but in making it, I've certainly felt like a total loser, not knowing how most of Rust
works (WHY am I borrowing things is this a fucking bank vs Real Estate guide), or figuring out what would work for a
somewhat universal translation layer, and so on. 

But in the end, I hope this achieves for me the dream of becoming a disney-channel type 'hacker' - effortless
programming, no need to spend hours scrolling through google or relying on generative AI for simple syntax (that my
brain refuses to remember).

---

<div align="center">
<sub>Chaos is a work in progress. Star the repo to follow along.</sub>
</div>