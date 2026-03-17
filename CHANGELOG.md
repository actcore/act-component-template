# Changelog

All notable changes to this template are documented here.

Downstream components generated from this template should note which version they were created from and apply relevant entries when upgrading.

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
