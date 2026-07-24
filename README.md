<div align="center">

```
 ██████╗██╗  ██╗ █████╗  ██████╗ ███████╗
██╔════╝██║  ██║██╔══██╗██╔═══██╗██╔════╝
██║     ███████║███████║██║   ██║███████╗
██║     ██╔══██║██╔══██║██║   ██║╚════██║
╚██████╗██║  ██║██║  ██║╚██████╔╝███████║
 ╚═════╝╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝ ╚══════╝
```

[![Typing SVG](https://readme-typing-svg.demolab.com?font=Fira+Code&size=20&pause=1000&color=00FF9C&center=true&vCenter=true&width=440&lines=One+syntax%2C+every+language.;No+boilerplate.;No+generative+AI.;Just+architecture.)](https://git.io/typing-svg)

![Rust](https://img.shields.io/badge/built%20with-Rust-orange?style=flat-square&logo=rust)
![Status](https://img.shields.io/badge/status-early%20development-yellow?style=flat-square)
![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)

</div>

---

## What is Chaos?

Chaos is a command-line tool that removes the redundancy of modern
programming — without generative AI, and without hiding the architecture
from you.

Building even a simple website today means juggling HTML, CSS, JavaScript,
npm, frameworks, folder conventions, and boilerplate that has nothing to do
with the actual idea you're trying to express. Chaos collapses that ceremony
into a single, minimal syntax that translates directly into clean,
real, editable code — in whatever language the target requires.

You still design the system. You still own the logic. Chaos just refuses
to make you type the same fifty characters of setup to say one simple thing.

## Philosophy

- **No generative AI.** Chaos is a deterministic translator, not a
  predictor. What you type is exactly what gets built — every time.
- **Real, inspectable output.** Chaos never hides code behind a black box.
  Every generated file is plain HTML/CSS/JS/Python/etc — open it, read it,
  edit it directly if you want.
- **One syntax, many targets.** The same chaos verb compiles differently
  depending on what you're building, but always cuts the ceremony down to
  the actual idea being expressed.

## Status

Chaos is early — currently in active development as a personal long-term
project (yes, all the way to a PhD, eventually). Right now:

- [x] Core CLI with four commands: `initialize`, `write`, `end`, `run`
- [x] Interactive project scaffolding flow (`chaos initialize`)
- [ ] File/folder generation from captured config
- [ ] `.gitignore` auto-generation per stack
- [ ] Dependency installation
- [ ] Live chaos-syntax translation (`chaos write`)
- [ ] Local dev server (`chaos run`)
- [ ] Terminal-styled desktop GUI

## Commands (v1 scope)

| Command | Purpose |
|---|---|
| `chaos initialize` | Scaffolds a new project through a guided Q&A |
| `chaos write` | Enters live mode — chaos syntax translates into real code as you type |
| `chaos end` | Exits live mode, back to a normal terminal |
| `chaos run` | Serves the project locally for testing |

## Supported stacks (v1)

- **Static Webpage** — HTML, CSS (Tailwind or plain), optional JavaScript
- **Basic Webapp** — frontend as above + backend (Python/Django or TypeScript/Express)

More languages, frameworks, and project types are planned — the system is
built so adding them is a data change, not a rewrite.

## Built with

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white) — chosen for safety, speed, and a single self-contained binary with
zero runtime dependencies for the end user.

---

<div align="center">
<sub>Chaos is a work in progress. Star the repo to follow along.</sub>
</div>