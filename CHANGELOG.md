# Changelog

All notable changes to this template are documented here.

Downstream components generated from this template should note which version they were created from and apply relevant entries when upgrading.

## [0.4.0] - 2026-03-31

### Added
- **`act.toml` manifest**: Component metadata (name, version, description, capabilities) now lives in `act.toml`. The `#[act_component]` macro reads it automatically.
- **`skill/SKILL.md`**: Embedded agent skill for tool discovery. Packed into `act:skill` WASM custom section via `embed_skill!`.
- `act.toml` and `skill/*` added to `_skip_if_exists` (preserved on `copier update`).

### Changed
- **act-sdk 0.2.7**: Adds `act.toml` manifest support, typed `Capabilities` struct, `embed_skill!` macro.
- **wit-bindgen 0.54**: Updated from 0.53.
- `#[act_component]` no longer needs inline `name`/`version`/`description` — read from `act.toml`.
- `src/lib.rs` now calls `act_sdk::embed_skill!("skill/")`.
- Removed `check-json` from prek.toml (no JSON files in typical components).
- Fixed stray `{% endraw %}` in `e2e/tools.hurl`.

### Migration from 0.3.0
1. Create `act.toml` in your component root:
   ```toml
   name = "my-component"
   version = "0.1.0"
   description = "My component description"
   ```
2. Create `skill/SKILL.md` with frontmatter (`name`, `description`, `metadata.act: {}`) and tool docs.
3. Add `act_sdk::embed_skill!("skill/");` before `#[act_component]` in `src/lib.rs`.
4. Simplify `#[act_component(name = "...", ...)]` to `#[act_component]`.
5. Update `act-sdk` to `0.2.7` and `wit-bindgen` to `0.54` in `Cargo.toml`.

## [0.3.0] - 2026-03-17

### Changed
- **Migrated from cargo-generate to Copier**: `copier copy` for new projects, `copier update` to sync existing components with template changes.
- Template files moved into `template/` subdirectory (`_subdirectory: template` in copier.yml).
- Jinja2 syntax replaces cargo-generate `{{ }}` placeholders.
- Files with runtime `{{ }}` variables (justfile, hurl, CI) use `{% raw %}` blocks.
- `_skip_if_exists` prevents overwriting `src/lib.rs`, `e2e/*.hurl`, `.gitignore` on update.
- Clippy CI job added.

### Removed
- `cargo-generate.toml` — replaced by `copier.yml`

### Migration from 0.2.0
1. Install Copier: `pipx install copier`
2. Create `.copier-answers.yml` in your component (see spec for format)
3. Run `copier update` to pull future template changes

## [0.2.0] - 2026-03-17

### Changed
- **wit-deps instead of git submodules**: WIT dependencies are now managed by `wit-deps`. Added `wit/deps.toml` manifest pointing to `act-spec` repo. `wit/deps/` is gitignored.
- **moonrepo/setup-rust in CI**: Replaced `dtolnay/rust-toolchain` + `Swatinem/rust-cache` + `taiki-e/install-action` + `taiki-e/cache-cargo-install-action` with a single `moonrepo/setup-rust@v1` step.
- **Split init/setup recipes**: `just init` fetches WIT deps only (needed for build/CI). `just setup` runs init + installs prek hooks (dev environment).
- **AGENTS.md + CLAUDE.md**: Added project instructions for AI agents. CLAUDE.md is a symlink to AGENTS.md.

### Migration from 0.1.0
1. Remove git submodule: `git submodule deinit -f wit/deps/act-core && git rm -f wit/deps/act-core`
2. Create `wit/deps.toml`:
   ```toml
   act-core = "https://github.com/actcore/act-spec/archive/main.tar.gz"
   ```
3. Add `wit/deps/` to `.gitignore`
4. Run `wit-deps` to populate `wit/deps/`
5. Update justfile: replace `init` recipe (remove submodule add, use `wit-deps`), add `setup` recipe
6. Update CI workflow: replace toolchain/cache/install actions with `moonrepo/setup-rust@v1`, add `just init` before `just build`

## [0.1.0] - 2026-03-16

### Added
- Initial template with act-sdk 0.2.2, wit-bindgen 0.53
- `#[act_component]` + `#[act_tool]` mod-based macros
- WIT world exporting `act:core/tool-provider@0.2.0`
- justfile with build, test recipes
- hurl e2e smoke tests (info + tools endpoints)
- GitHub Actions CI (build, e2e, fmt)
- prek pre-commit hooks (clippy, fmt, yaml, toml)
- dependabot config for cargo + github-actions
- MIT + Apache-2.0 dual license
