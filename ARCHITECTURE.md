# Chaos — Architecture Notes

This document explains how `chaos initialize` generates projects, and the
rules any new "feature" (a styling option, a JS library, a backend
framework, etc.) needs to follow to plug into the system correctly.

This is a living document — update it whenever the generation system
changes shape, not just when new features are added.

**Audience note:** Chaos is built for developers who already know their
stack — it assumes familiarity with the frameworks/tools it scaffolds,
and doesn't attempt to explain them. Release 1 scope is web development
only; other platforms (mobile, desktop, games) are planned as separate
future release tracks with their own decision trees, not extensions of
this one.

**Status:** the `BuildPlan` pattern is proven for frontend features
(Tailwind, Bootstrap, Sass, plain CSS, JS). Backend generation spans
eight language/framework combinations across two distinct patterns (see
below). **All eight backends, all four CSS options, and the
dependencies-declined edge case have been tested end to end** as of the
last build session. One real bug was found and fixed during this pass
(see the `require_tool` section below) — everything else worked as
designed on first test. `chaos write` (the syntax translation layer) is
still unbuilt. The desktop GUI shell has been dropped from the plan
entirely — Chaos is CLI-only, meant to run inside whatever IDE's
integrated terminal the developer already uses. A VS Code extension is
under consideration for later, as an addition on top of the CLI, not a
replacement for it.

---

## The core idea: composable contributions, not fixed templates

Early versions of `chaos initialize` considered generating a fixed
"template" per combination of answers (e.g. one template for
"Tailwind + JS", another for "Plain CSS + no JS"). This was rejected —
every new option would multiply against every existing option
(combinatorial explosion), making the system unmaintainable as it grows.

Instead, **each user choice contributes its own independent fragment**
to a shared build plan. Chaos assembles the final project by combining
whichever fragments apply. A feature never needs to know what any other
feature decided.

## The `BuildPlan` struct

```rust
struct BuildPlan {
    files: Vec<(String, String)>,   // (relative path, file content)
    gitignore_entries: Vec<String>,
    npm_packages: Vec<String>,
}
```

This is the shared, growing plan every feature contributes to. It gets
built up piece by piece, then written to disk in one pass at the end
(`execute_plan`).

## The contract: how a feature function must behave

Every "add this feature" function (e.g. `add_tailwind`, `add_javascript`)
follows the same shape:

```rust
fn add_my_feature(plan: &mut BuildPlan, config: &ProjectConfig) {
    // 1. Push any files this feature needs to generate
    plan.files.push(("path/to/file".to_string(), "file content".to_string()));

    // 2. Push any .gitignore lines this feature requires
    plan.gitignore_entries.push("some_folder/".to_string());

    // 3. Push any npm packages this feature needs installed
    plan.npm_packages.push("package-name".to_string());
}
```

**Rules:**

- Takes `&mut BuildPlan` always. Takes `&ProjectConfig` too if it needs
  project-specific info (e.g. the project's name).
- May contribute to `files`, `gitignore_entries`, and/or `npm_packages` —
  only what's actually relevant. Not every feature needs all three (plain
  vanilla JS, for example, contributes a file but no packages, since it
  has no dependencies).
- **Must not** read or depend on any other feature's choices. Tailwind's
  function doesn't know or care whether JavaScript was included — that
  isolation is what keeps this system from becoming unmanageable as more
  features are added.
- Does **not** write to disk itself. Only `execute_plan` touches the
  filesystem, once, after every feature has contributed.

## Wiring a new feature in

Once the function exists, it needs to be called conditionally based on
the relevant user choice, inside the generator function for that project
type (e.g. `generate_static_webpage`):

```rust
if config.styling == "My New Option" {
    add_my_feature(&mut plan, config);
}
```

That's the only place a new option needs to be "known about" structurally
— everything else (writing files, running npm install, handling
.gitignore) already works generically off the plan's contents.

## Installing dependencies

`run_npm_install` doesn't know which feature contributed which package —
it just installs whatever ended up in `plan.npm_packages`, if the user
opted in. This means adding a new JS-dependent feature never requires
touching the install logic itself, only the feature's own function.

## A second pattern: backend scaffolding (orchestration, not file-writing)

Frontend features (Tailwind, plain CSS, JS) work by **writing file content
Chaos itself authors**, via the `BuildPlan` contract above. Most backend
features work differently: rather than authoring files, Chaos **installs
the real framework and shells out to its own official scaffolding tool**
(`django-admin startproject`, `npx express-generator`, `rails new`,
`composer create-project laravel/laravel`, `nest new`, `fastify-cli
generate`) — since that's the only way to get genuinely correct,
idiomatic project structure rather than a hand-rolled imitation of it.

This means backend generation doesn't go through `BuildPlan` at all. It's
handled by a dispatcher function (`generate_backend`) which matches on
`(backend_language, backend_framework)` and routes to one of several
per-framework generator functions, each of which:

1. Checks its required toolchain is actually installed (see `require_tool`
   below) before doing anything else
2. Installs the framework itself
3. Runs that framework's own CLI generator, scoped to `backend/`
4. Returns a list of `.gitignore` entries the caller merges into the
   project's root `.gitignore`

### A third pattern: hand-authored backends (no official CLI exists)

Two backends — **Flask** (Python) and **Gin** (Go) — have no official
project-scaffolding tool at all. Real-world projects in both are just
"install the library, write the entry file by hand." Chaos follows suit:
it still installs the library/module for real, but authors the starter
file (`app.py`, `main.go`) itself, the same way frontend features do.
Gin's generated file includes a credit comment linking to the project,
since it's a community open source framework, not an official language
tool.

**Important consequence:** for every backend except the hand-authored
ones, *installation and generation are the same step* — unlike frontend,
where files exist whether or not `npm install` actually runs. If the
user opts out of dependency installation, none of the CLI-orchestrated
backends can be scaffolded at all (no Django project can exist without
Django being installed first). Chaos handles this by creating an empty
`backend/` folder and printing a clear explanation, rather than silently
failing or pretending to succeed.

## The `require_tool` guardrail

Every function that shells out to an external tool (`python3`, `npx`,
`go`, `ruby`, `rails`, `php`, `composer`, `npm`) checks first that the
tool actually exists on the user's machine:

```rust
fn require_tool(command: &str, install_hint: &str) -> bool {
    let check = Command::new(command).arg("--version").output();
    match check {
        Ok(_) => true,
        Err(_) => {
            println!("\n '{}' isn't installed or isn't on your PATH.", command);
            println!("   {}", install_hint);
            false
        }
    }
}
```

**Rule: any new backend integration must call this before attempting to
run its toolchain**, and must pass a real, working install link — not a
vague "install X" message. The goal is that someone hitting a missing
dependency gets told exactly what to do next, rather than seeing a raw
Rust panic. This was added specifically because early testing (before
this guardrail existed) crashed with a confusing OS-level error rather
than a clear message — see the lesson below.

**Confirmed real-world case:** on macOS, several deprecated system tools
(`ruby`, and gem-based commands like `rails`) are replaced with Apple
stub scripts. These print "X is not currently installed" — but exit with
code `0` (success), violating the standard convention that non-zero means
failure. This silently defeated the exit-status-only version of this
check (confirmed live: `require_tool` initially reported Rails as
installed, and `generate_rails_backend` printed a false "Generated Rails
project" success message despite nothing real being scaffolded). The
fix: also inspect the command's actual output text for that specific
phrase, treating it as failure regardless of exit code. Retested after
the fix — correctly reports Rails as not installed, no phantom files
written to `backend/`. This is a narrow, macOS-specific patch, not a
general "detect fake tools" solution — if a similarly-behaved stub is
found on another platform or tool, it likely needs its own explicit
string check added here.

### Lesson learned: relative paths + `current_dir` don't mix reliably

When chaining multiple external commands inside a freshly-created folder
(venv creation → pip install → django-admin), using a **relative** path
to the executable (e.g. `"test/backend/venv/bin/pip"`) alongside
`.current_dir(...)` caused the path to be resolved incorrectly, doubling
the folder path and failing with "No such file or directory."

**Fix:** always resolve to an absolute path with `fs::canonicalize(...)`
before building paths to executables inside a working directory you're
also passing to `.current_dir(...)`. Any new backend integration
following this pattern should do the same.

## Planned: pre-flight / post-flow questions (not yet built)

The current question flow is fully linear — every question is asked in
a fixed order, and later questions can only branch based on *earlier*
answers (e.g. backend framework choices depend on the backend language
already picked). This breaks down once the option list grows to include
choices that depend on information from a *different, unrelated* branch
— for example, offering tRPC as a frontend data-fetching option only
makes sense if the backend is *also* TypeScript, but frontend questions
are asked before the backend branch in the current flow.

**Planned fix — split question-asking into three phases instead of one
linear pass:**

1. **Pre-flight phase** — a small number of global, stack-wide questions
   asked *before* the main branching tree starts (e.g. "Will your
   frontend be TypeScript?", "Will your backend be TypeScript?"). These
   exist purely to make information available early enough for
   later branches to use it, even though the branches themselves come
   later.
2. **Main flow** — the existing linear branch-by-branch flow, largely
   unchanged, but now able to reference pre-flight answers when deciding
   what to offer (e.g. only show tRPC as a data-fetching option if both
   pre-flight TypeScript answers were yes).
3. **Post-flow phase** — a final pass, after the entire main flow
   completes, that offers additional options which only make sense in
   hindsight of choices made *later* in the flow than where they'd
   naturally be asked (e.g. "You picked Supabase as your database
   provider — want to use Supabase Auth?", where auth is conceptually
   asked before database provider in the main flow).

**This is a genuine gap in the current architecture, not yet
implemented.** `ProjectConfig` and the question flow in `run_initialize`
would both need restructuring to support this — likely `ProjectConfig`
gains a few pre-flight fields set before the main match/if chain runs,
and a new post-flow function runs after `generate_backend`/frontend
generation to offer additional, hindsight-dependent contributions to the
build plan.

**Two known options were dropped from consideration entirely** rather
than solved by this pattern, since they weren't actually timing
problems: **Astro** (framework-agnostic, doesn't fit a single-framework
slot at all) and **PocketBase** (a full alternative backend, not really
an auth provider). Both excluded from Release 1's scope. Users needing
either can use the planned `chaos edit` command (see below) to
hand-configure things Chaos doesn't offer directly.

## Planned: `chaos edit` (fifth command, not yet built)

A new command planned alongside `chaos write`/`chaos end`: lets a user
modify an already-initialized project's captured specs after the fact —
changing earlier answers, or adding custom/unlisted options entirely
outside what `chaos initialize`'s menus offer (e.g. Astro, PocketBase,
or anything else not in the standard list). Not yet designed in detail;
noted here so it isn't lost before `chaos write` work begins.



- **Windows paths are unhandled.** `venv/bin/pip` and `venv/bin/django-admin`
  are Mac/Linux paths; Windows uses `venv\Scripts\pip.exe`. Not an issue
  for current development (Mac-only), but a real gap before Chaos could
  run cross-platform.
- No handling yet for two features wanting to write to the *same* file
  (e.g. two libraries both wanting to modify `package.json`). This hasn't
  come up yet since only one frontend feature (Tailwind) writes one
  currently.
- Bootstrap's generated HTML links directly to
  `node_modules/bootstrap/dist/css/bootstrap.min.css` rather than a
  properly bundled/copied file. Works for local dev via `chaos run`
  later, but isn't how Bootstrap would be referenced in a real
  production build — worth revisiting once `chaos run` exists.