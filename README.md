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
- [x] File/folder generation from captured config
- [x] `.gitignore` auto-generation per stack
- [x] Dependency installation (npm + pip, real framework scaffolding via
      `django-admin` / `express-generator`)
- [ ] Live chaos-syntax translation (`chaos write`)
- [ ] Local dev server (`chaos run`)
- [ ] Terminal-styled desktop GUI

`chaos initialize` is feature-complete for both v1 project types as of the
last build session — the remaining work is the syntax translation layer
itself (`chaos write`), which is the real core of the project.

## Commands (v1 scope)

| Command | Purpose |
|---|---|
| `chaos initialize` | Scaffolds a new project through a guided Q&A |
| `chaos write` | Enters live mode — chaos syntax translates into real code as you type |
| `chaos end` | Exits live mode, back to a normal terminal |
| `chaos run` | Serves the project locally for testing |

## Supported stacks (v1)

- **Static Webpage** — HTML, CSS (Tailwind, Plain CSS, Bootstrap, or Sass/SCSS), optional JavaScript
- **Basic Webapp** — frontend as above, plus a real backend:

| Language | Frameworks | Scaffolding method |
|---|---|---|
| Python | Django, Flask | Django via `django-admin`; Flask hand-authored (no official scaffolder exists) |
| TypeScript | Express, Fastify, NestJS | Each framework's own official CLI generator |
| Ruby | Rails | `rails new` |
| PHP | Laravel | `composer create-project laravel/laravel` |
| Go | Gin | Hand-authored — [Gin](https://github.com/gin-gonic/gin) is an open source community framework, credited in the generated code |

Every backend checks that its required toolchain (Python, Node, Ruby, PHP,
Go, Composer) is actually installed before attempting to scaffold, and
prints a clear install link if it's missing, rather than failing silently
or crashing.

The CSS additions and five of the eight backend paths (Fastify, NestJS,
Flask, Go/Gin, Rails, Laravel) are newly built and **not yet verified by a
real test run** — Django and Express are the two paths that have been
fully tested end to end so far.

More languages, frameworks, and project types are planned — the system is
built so adding them is a data change, not a rewrite.

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