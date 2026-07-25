# Chaos — Architecture Notes

This document explains how `chaos initialize` generates projects, and the
rules any new "feature" (a styling option, a JS library, a backend
framework, etc.) needs to follow to plug into the system correctly.

This is a living document — update it whenever the generation system
changes shape, not just when new features are added.

**Status:** the pattern below is currently proven by one real example
(Tailwind CSS). It's expected to hold as more features are added, but
hasn't been stress-tested against many contributors yet — treat this as
the intended design, and update this doc immediately if a new feature
reveals a gap in it.

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

## Known limitations / open questions

- This pattern is currently only proven for **Static Webpage** generation.
  Basic Webapp (frontend + backend + docs folder structure) hasn't been
  built yet, and may reveal cases this contract doesn't cleanly cover
  (e.g. pip packages alongside npm packages, or files that need to live
  in a specific subfolder like `backend/` rather than project root).
- No handling yet for two features wanting to write to the *same* file
  (e.g. two libraries both wanting to modify `package.json`). This hasn't
  come up yet since only one feature (Tailwind) writes one currently.