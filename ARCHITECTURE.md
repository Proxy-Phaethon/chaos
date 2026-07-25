# Chaos — Architecture Notes

This document explains how `chaos initialize` generates projects, and the
rules any new "feature" (a styling option, a JS library, a backend
framework, etc.) needs to follow to plug into the system correctly.

This is a living document — update it whenever the generation system
changes shape, not just when new features are added.

**Status:** the file/gitignore/npm-package pattern is proven by two real
examples now (Tailwind CSS for frontend, and — see below — the backend
scaffolding for Django/Express, which follows a related but distinct
pattern of its own). Static Webpage and Basic Webapp generation are both
fully working as of the last build session. `chaos write` (the syntax
translation layer) is still unbuilt.

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
Chaos itself authors**, via the `BuildPlan` contract above. Backend
features work differently: rather than authoring files, Chaos **installs
the real framework and shells out to its own official scaffolding tool**
(`django-admin startproject`, `npx express-generator`) — since that's the
only way to get genuinely correct, idiomatic project structure rather
than a hand-rolled imitation of it.

This means backend generation doesn't go through `BuildPlan` at all. It's
handled by its own function (`generate_backend`) which:

1. Creates the `backend/` folder
2. Installs the framework (into a virtual environment, for Python)
3. Runs that framework's own CLI generator, scoped to `backend/`
4. Returns a list of `.gitignore` entries the caller merges into the
   project's root `.gitignore`

**Important consequence:** for backend scaffolding, *installation and
generation are the same step* — unlike frontend, where files exist
whether or not `npm install` actually runs. If the user opts out of
dependency installation, the backend genuinely cannot be scaffolded at
all (no Django project can exist without Django being installed first).
Chaos handles this by creating an empty `backend/` folder and printing a
clear explanation, rather than silently failing or pretending to succeed.

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

## Known limitations / open questions

- **Windows paths are unhandled.** `venv/bin/pip` and `venv/bin/django-admin`
  are Mac/Linux paths; Windows uses `venv\Scripts\pip.exe`. Not an issue
  for current development (Mac-only), but a real gap before Chaos could
  run cross-platform.
- No handling yet for two features wanting to write to the *same* file
  (e.g. two libraries both wanting to modify `package.json`). This hasn't
  come up yet since only one frontend feature (Tailwind) writes one
  currently.
- Backend scaffolding is currently proven for exactly two frameworks
  (Django, Express). A third backend framework/language would be the
  real test of whether `generate_backend`'s `match` structure holds up
  as a general pattern, or needs further generalizing.