# AGENTS.md

## Project Overview

**act-component-template** is a [cargo-generate](https://cargo-generate.github.io/cargo-generate/) template for scaffolding new ACT (Agent Component Tools) components in Rust.

Generated projects include: SDK wiring, WIT bindings, justfile, e2e tests (hurl), CI (GitHub Actions), pre-commit hooks (prek), and licenses.

## Scaffold a new component

```bash
cargo generate --git https://github.com/actcore/act-component-template
cd <project-name>
just init    # fetch WIT deps
just build   # build wasm component
just test    # run e2e tests
```

## Repository Structure

```
Cargo.toml              # Package template ({{project-name}}, {{crate_name}} placeholders)
cargo-generate.toml     # cargo-generate config + placeholders
src/lib.rs              # Component source with #[act_component] + #[act_tool]
wit/
  world.wit             # WIT world definition (exports act:core/tool-provider@0.2.0)
  deps.toml             # wit-deps manifest (fetches act-core from act-spec)
  deps/                 # (gitignored) populated by wit-deps
e2e/
  smoke.hurl            # Smoke tests: /info + /tools endpoints
justfile                # Recipes: init, setup, build, test
prek.toml               # Pre-commit hooks: clippy, fmt, yaml, toml
rust-toolchain.toml     # Nightly + wasm32-wasip2 target
.github/workflows/ci.yml  # CI: build, e2e, fmt
.github/dependabot.yml    # Dependabot for cargo + github-actions
.gitignore              # target/ + wit/deps/
```

## Template Placeholders

| Placeholder | Source | Used in |
|-------------|--------|---------|
| `{{project-name}}` | cargo-generate (from dir name) | Cargo.toml, src/lib.rs, e2e/smoke.hurl |
| `{{crate_name}}` | auto (project-name with `-` → `_`) | justfile (wasm path) |
| `{{description}}` | prompted | Cargo.toml, src/lib.rs |
| `{{author}}` | prompted (default: "ACT contributors") | — |

## Development Patterns

- **WIT deps**: managed by `wit-deps` (not git submodules). Run `just init` to fetch.
- **Build target**: `wasm32-wasip2` (nightly toolchain via `rust-toolchain.toml`)
- **SDK**: `act-sdk` with `#[act_component]` mod-based macro, `#[act_tool]` per function
- **Testing**: hurl HTTP tests against `act serve` on a random port
- **CI**: `moonrepo/setup-rust@v1` for toolchain + bins (replaces dtolnay + rust-cache + install-action)
- **Pre-commit**: `prek` (clippy for wasm32-wasip2, fmt, yaml/toml checks)

## Commands

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
