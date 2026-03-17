# AGENTS.md

## Project Overview

**act-component-template** is a [Copier](https://copier.readthedocs.io/) template for scaffolding new ACT (Agent Component Tools) components in Rust.

Generated projects include: SDK wiring, WIT bindings, justfile, e2e tests (hurl), CI (GitHub Actions), pre-commit hooks (prek), and licenses. Existing components can pull template updates via `copier update`.

## Scaffold a new component

```bash
copier copy gh:actcore/act-component-template my-component
cd my-component
just init    # fetch WIT deps
just build   # build wasm component
just test    # run e2e tests
```

## Repository Structure

```
copier.yml              # Copier config (questions, _subdirectory, _skip_if_exists)
AGENTS.md               # This file (not copied to projects)
CLAUDE.md -> AGENTS.md
CHANGELOG.md            # Template version history (not copied)
docs/                   # Specs and plans (not copied)
template/               # _subdirectory — only this gets copied to new projects
  Cargo.toml            # Package template ({{ project_name }} placeholder)
  README.md
  src/lib.rs            # Component source with #[act_component] + #[act_tool]
  wit/
    world.wit           # WIT world definition (exports act:core/tool-provider@0.2.0)
    deps.toml           # wit-deps manifest (fetches act-core from act-spec)
  e2e/
    info.hurl           # Smoke test: /info endpoint
    tools.hurl          # Smoke test: /tools endpoint
  justfile              # Recipes: init, setup, build, test
  prek.toml             # Pre-commit hooks: clippy, fmt, yaml, toml
  rust-toolchain.toml   # Nightly + wasm32-wasip2 target
  .github/workflows/ci.yml  # CI: build, clippy, e2e, fmt
  .github/dependabot.yml    # Dependabot for cargo + github-actions
  .gitignore            # target/ + wit/deps/
  .cargo/config.toml    # Default build target
  LICENSE-MIT
  LICENSE-APACHE
```

## Template Variables

| Variable | Prompt | Used in |
|----------|--------|---------|
| `project_name` | "Component name (e.g. my-tool)" | Cargo.toml, src/lib.rs, justfile, e2e/info.hurl |
| `description` | "Component description" | src/lib.rs, README.md |

Derived: `project_name | replace('-', '_')` for crate name in justfile wasm path.

## Jinja2 / Runtime Variable Conflicts

Files with runtime `{{ }}` variables (justfile, hurl, GitHub Actions) use `{% raw %}` blocks to prevent Jinja2 from interpreting them. Only Copier placeholders are outside raw blocks.

## Development Patterns

- **WIT deps**: managed by `wit-deps` (not git submodules). Run `just init` to fetch.
- **Build target**: `wasm32-wasip2` (nightly toolchain via `rust-toolchain.toml`)
- **SDK**: `act-sdk` with `#[act_component]` mod-based macro, `#[act_tool]` per function
- **Testing**: hurl HTTP tests against `act serve` on a random port
- **CI**: `moonrepo/setup-rust@v1` for toolchain + bins
- **Pre-commit**: `prek` (clippy for wasm32-wasip2, fmt, yaml/toml checks)
- **Template sync**: `copier update` pulls template changes into existing components

## Commands (in generated projects)

```bash
just init    # wit-deps: fetch WIT dependencies
just setup   # init + prek install (dev environment)
just build   # cargo build --target wasm32-wasip2 --release
just test    # act serve + hurl e2e tests (does NOT build, run just build first)
```

## Conventions

- Conventional commits (`feat:`, `fix:`, `refactor:`, `docs:`, `chore:`)
- Always include "act" in crate keywords
- Components target `wasm32-wasip2` with nightly toolchain
- `rust-toolchain.toml` over `+nightly` flags
- `baseurl` variable in hurl tests, dynamic port via Python socket

## Changelog

This template is versioned. Downstream components should reference the template version they were generated from and apply relevant changelog entries when upgrading.

See [CHANGELOG.md](CHANGELOG.md) for the full history.
