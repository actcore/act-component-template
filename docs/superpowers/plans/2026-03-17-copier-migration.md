# Copier Template Migration Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Migrate act-component-template from cargo-generate to Copier with `{% raw %}` blocks for runtime variable conflicts, enabling `copier update` for existing components.

**Architecture:** Move all template files into `template/` subdirectory. Create `copier.yml` at repo root with `_subdirectory: template`. Convert cargo-generate `{{ }}` placeholders to Jinja2 syntax. Wrap runtime `{{ }}` variables (justfile, hurl, GitHub Actions) in `{% raw %}` blocks. Keep AGENTS.md, CLAUDE.md, CHANGELOG.md at repo root (outside template/).

**Tech Stack:** Copier >= 9.4.0, Jinja2 templating

**Spec:** `docs/specs/2026-03-17-copier-migration.md`

---

## File Structure

| File | Action | Responsibility |
|------|--------|---------------|
| `copier.yml` | Create | Copier config: questions, _subdirectory, _skip_if_exists |
| `template/` | Create (directory) | All files that get copied to new projects |
| `template/Cargo.toml` | Move + modify | Jinja2 placeholders |
| `template/README.md` | Move + modify | Jinja2 placeholders |
| `template/src/lib.rs` | Move + modify | Jinja2 placeholders |
| `template/e2e/info.hurl` | Move + modify | Mixed: Jinja2 + {% raw %} for hurl vars |
| `template/e2e/tools.hurl` | Move + modify | Whole file in {% raw %} |
| `template/justfile` | Move + modify | First line Jinja2, rest in {% raw %} |
| `template/.github/workflows/ci.yml` | Move + modify | Whole file in {% raw %} |
| `template/.github/dependabot.yml` | Move (as-is) | No placeholders |
| `template/.gitignore` | Move (as-is) | No placeholders |
| `template/.cargo/config.toml` | Move (as-is) | No placeholders |
| `template/prek.toml` | Move (as-is) | No placeholders |
| `template/rust-toolchain.toml` | Move (as-is) | No placeholders |
| `template/LICENSE-MIT` | Move (as-is) | No placeholders |
| `template/LICENSE-APACHE` | Move (as-is) | No placeholders |
| `template/wit/world.wit` | Move (as-is) | No placeholders |
| `template/wit/deps.toml` | Move (as-is) | No placeholders |
| `cargo-generate.toml` | Delete | Replaced by copier.yml |
| `wit/deps.lock` | Delete | Not part of template |

Files staying at repo root (NOT moved): `AGENTS.md`, `CLAUDE.md`, `CHANGELOG.md`, `docs/`

---

## Chunk 1: Create template directory and copier.yml

### Task 1: Create copier.yml and template/ directory structure

**Files:**
- Create: `copier.yml`
- Create: `template/` (directory)

- [ ] **Step 1: Create `copier.yml` at repo root**

```yaml
_min_copier_version: "9.4.0"
_subdirectory: template
_skip_if_exists:
  - src/lib.rs
  - "e2e/*.hurl"
  - .gitignore

project_name:
  type: str
  help: Component name (e.g. my-tool)

description:
  type: str
  help: Component description
```

- [ ] **Step 2: Create `template/` directory**

```bash
mkdir -p template/src template/e2e template/wit template/.github/workflows template/.cargo
```

- [ ] **Step 3: Commit**

```bash
git add copier.yml
GIT_AUTHOR_DATE="2026-03-17T23:00:00+03:00" GIT_COMMITTER_DATE="2026-03-17T23:00:00+03:00" \
git commit -m "feat: add copier.yml configuration"
```

---

## Chunk 2: Move and convert template files

### Task 2: Move static files (no placeholder changes needed)

**Files:**
- Move: `.cargo/config.toml` → `template/.cargo/config.toml`
- Move: `prek.toml` → `template/prek.toml`
- Move: `rust-toolchain.toml` → `template/rust-toolchain.toml`
- Move: `LICENSE-MIT` → `template/LICENSE-MIT`
- Move: `LICENSE-APACHE` → `template/LICENSE-APACHE`
- Move: `.gitignore` → `template/.gitignore`
- Move: `.github/dependabot.yml` → `template/.github/dependabot.yml`
- Move: `wit/world.wit` → `template/wit/world.wit`
- Move: `wit/deps.toml` → `template/wit/deps.toml`

- [ ] **Step 1: Move all static files**

```bash
cd /mnt/devenv/workspace/act/act-component-template
git mv .cargo/config.toml template/.cargo/config.toml
git mv prek.toml template/prek.toml
git mv rust-toolchain.toml template/rust-toolchain.toml
git mv LICENSE-MIT template/LICENSE-MIT
git mv LICENSE-APACHE template/LICENSE-APACHE
git mv .gitignore template/.gitignore
git mv .github/dependabot.yml template/.github/dependabot.yml
git mv wit/world.wit template/wit/world.wit
git mv wit/deps.toml template/wit/deps.toml
```

- [ ] **Step 2: Remove files that don't belong in template**

```bash
rm -f wit/deps.lock
git rm -f cargo-generate.toml
```

- [ ] **Step 3: Commit**

```bash
git add -A
GIT_AUTHOR_DATE="2026-03-17T23:05:00+03:00" GIT_COMMITTER_DATE="2026-03-17T23:05:00+03:00" \
git commit -m "refactor: move static files into template/ subdirectory"
```

---

### Task 3: Convert Cargo.toml — Jinja2 placeholders

**Files:**
- Move + modify: `Cargo.toml` → `template/Cargo.toml`

- [ ] **Step 1: Move and convert**

Move the file, then replace cargo-generate placeholders with Jinja2:

```bash
git mv Cargo.toml template/Cargo.toml
```

Edit `template/Cargo.toml` — change line 2 from:
```toml
name = "component-{{project-name}}"
```
To:
```toml
name = "component-{{ project_name }}"
```

- [ ] **Step 2: Commit**

```bash
git add template/Cargo.toml
GIT_AUTHOR_DATE="2026-03-17T23:10:00+03:00" GIT_COMMITTER_DATE="2026-03-17T23:10:00+03:00" \
git commit -m "feat: convert Cargo.toml to Jinja2 placeholders"
```

---

### Task 4: Convert README.md — Jinja2 placeholders

**Files:**
- Move + modify: `README.md` → `template/README.md`

- [ ] **Step 1: Move and convert**

```bash
git mv README.md template/README.md
```

Edit `template/README.md` — the full content should be:

```markdown
# {{ project_name }}

{{ description }}

## Usage

```bash
just init   # first time: fetch WIT deps + install prek hooks
just build  # build wasm component
just test   # run e2e tests
```

## License

MIT OR Apache-2.0
```

(Replace `{{project-name}}` → `{{ project_name }}`, `{{description}}` → `{{ description }}`)

- [ ] **Step 2: Commit**

```bash
git add template/README.md
GIT_AUTHOR_DATE="2026-03-17T23:15:00+03:00" GIT_COMMITTER_DATE="2026-03-17T23:15:00+03:00" \
git commit -m "feat: convert README.md to Jinja2 placeholders"
```

---

### Task 5: Convert src/lib.rs — Jinja2 placeholders

**Files:**
- Move + modify: `src/lib.rs` → `template/src/lib.rs`

- [ ] **Step 1: Move and convert**

```bash
git mv src/lib.rs template/src/lib.rs
```

Edit `template/src/lib.rs` — replace placeholders:
- `{{project-name}}` → `{{ project_name }}`
- `{{description}}` → `{{ description }}`

Full content:
```rust
use act_sdk::prelude::*;

#[act_component(
    name = "{{ project_name }}",
    version = "0.1.0",
    description = "{{ description }}",
)]
mod component {
    use super::*;

    #[act_tool(description = "Say hello", read_only)]
    fn hello(
        /// Name to greet
        name: Option<String>,
    ) -> ActResult<String> {
        let who = name.unwrap_or_else(|| "world".to_string());
        Ok(format!("Hello, {who}!"))
    }
}
```

- [ ] **Step 2: Commit**

```bash
git add template/src/lib.rs
GIT_AUTHOR_DATE="2026-03-17T23:20:00+03:00" GIT_COMMITTER_DATE="2026-03-17T23:20:00+03:00" \
git commit -m "feat: convert src/lib.rs to Jinja2 placeholders"
```

---

### Task 6: Convert justfile — Jinja2 + raw blocks

**Files:**
- Move + modify: `justfile` → `template/justfile`

- [ ] **Step 1: Move and convert**

```bash
git mv justfile template/justfile
```

Edit `template/justfile` — first line uses Jinja2, rest wrapped in `{% raw %}`:

```
wasm := "target/wasm32-wasip2/release/component_{{ project_name | replace('-', '_') }}.wasm"
{% raw %}
act := env("ACT", "act")
port := `python3 -c 'import socket; s=socket.socket(socket.AF_INET, socket.SOCK_STREAM); s.bind(("", 0)); print(s.getsockname()[1]); s.close()'`
addr := "[::1]:" + port
baseurl := "http://" + addr

init:
    wit-deps

setup: init
    prek install

build:
    cargo build --target wasm32-wasip2 --release

test:
    #!/usr/bin/env bash
    {{act}} serve {{wasm}} --listen "{{addr}}" &
    trap "kill $!" EXIT
    npx wait-on {{baseurl}}/info
    hurl --test --variable "baseurl={{baseurl}}" e2e/*.hurl
{% endraw %}
```

- [ ] **Step 2: Commit**

```bash
git add template/justfile
GIT_AUTHOR_DATE="2026-03-17T23:25:00+03:00" GIT_COMMITTER_DATE="2026-03-17T23:25:00+03:00" \
git commit -m "feat: convert justfile to Jinja2 with raw blocks"
```

---

### Task 7: Convert e2e tests — raw blocks for hurl variables

**Files:**
- Move + modify: `e2e/info.hurl` → `template/e2e/info.hurl`
- Move + modify: `e2e/tools.hurl` → `template/e2e/tools.hurl`

- [ ] **Step 1: Move and convert info.hurl**

```bash
git mv e2e/info.hurl template/e2e/info.hurl
```

Edit `template/e2e/info.hurl` — wrap `{{baseurl}}` in raw, keep `{{ project_name }}` as Jinja2:

```
GET {% raw %}{{baseurl}}{% endraw %}/info
HTTP 200
[Asserts]
jsonpath "$['std:name']" == "{{ project_name }}"
jsonpath "$['std:version']" isString
```

- [ ] **Step 2: Move and convert tools.hurl**

```bash
git mv e2e/tools.hurl template/e2e/tools.hurl
```

Edit `template/e2e/tools.hurl` — whole file in raw (no Copier vars):

```
{% raw %}
POST {{baseurl}}/tools
HTTP 200
[Asserts]
jsonpath "$.tools" count >= 1
{% endraw %}
```

- [ ] **Step 3: Commit**

```bash
git add template/e2e/
GIT_AUTHOR_DATE="2026-03-17T23:30:00+03:00" GIT_COMMITTER_DATE="2026-03-17T23:30:00+03:00" \
git commit -m "feat: convert e2e tests to Jinja2 with raw blocks"
```

---

### Task 8: Convert CI workflow — whole file in raw block

**Files:**
- Move + modify: `.github/workflows/ci.yml` → `template/.github/workflows/ci.yml`

- [ ] **Step 1: Move and wrap in raw**

```bash
git mv .github/workflows/ci.yml template/.github/workflows/ci.yml
```

Wrap the ENTIRE file content in `{% raw %}...{% endraw %}` since it contains `${{ secrets.GITHUB_TOKEN }}` (GitHub Actions syntax) and no Copier variables:

```yaml
{% raw %}
name: CI

on:
  push:
    branches: [main]
  pull_request:

env:
  CARGO_TERM_COLOR: always

jobs:
  build:
    name: Build
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v6
      - uses: moonrepo/setup-rust@v1
        with:
          channel: nightly
          targets: wasm32-wasip2
          bins: wit-deps-cli, just
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
      - run: just init
      - run: just build
      - uses: actions/upload-artifact@v7
        with:
          name: component.wasm
          path: target/wasm32-wasip2/release/*.wasm

  clippy:
    name: Clippy
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v6
      - uses: moonrepo/setup-rust@v1
        with:
          channel: nightly
          targets: wasm32-wasip2
          components: clippy
          bins: wit-deps-cli
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
      - run: wit-deps
      - run: cargo clippy --target wasm32-wasip2 -- -D warnings

  e2e:
    name: E2E Test
    needs: build
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v6
      - uses: actions/download-artifact@v4
        with:
          name: component.wasm
          path: target/wasm32-wasip2/release/
      - uses: actcore/act-cli/setup@main
      - uses: moonrepo/setup-rust@v1
        with:
          bins: just, hurl
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
      - run: just test

  fmt:
    name: Format
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v6
      - uses: moonrepo/setup-rust@v1
        with:
          channel: nightly
          components: rustfmt
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
      - run: cargo fmt -- --check
{% endraw %}
```

- [ ] **Step 2: Commit**

```bash
git add template/.github/workflows/ci.yml
GIT_AUTHOR_DATE="2026-03-17T23:35:00+03:00" GIT_COMMITTER_DATE="2026-03-17T23:35:00+03:00" \
git commit -m "feat: convert CI workflow to Jinja2 with raw block"
```

---

## Chunk 3: Cleanup and verification

### Task 9: Clean up empty directories and update AGENTS.md

**Files:**
- Delete: empty directories left after moves (`src/`, `e2e/`, `wit/`, `.cargo/`, `.github/`)
- Modify: `AGENTS.md` — update structure docs

- [ ] **Step 1: Remove empty directories and leftover files**

```bash
cd /mnt/devenv/workspace/act/act-component-template
# Git tracks files not dirs, but clean up any leftover empty dirs
rmdir src e2e .cargo 2>/dev/null || true
rm -rf wit  # deps.lock + deps/ dir
rmdir .github/workflows .github 2>/dev/null || true
```

- [ ] **Step 2: Update AGENTS.md — reflect new structure**

Update the "Repository Structure" section to show the `template/` subdirectory layout and mention Copier instead of cargo-generate. Update the "Scaffold" section to show `copier copy` instead of `cargo generate`. Remove references to `cargo-generate.toml`.

- [ ] **Step 3: Update CHANGELOG.md — add 0.3.0 entry**

Add entry for Copier migration:
```markdown
## [0.3.0] - 2026-03-17

### Changed
- **Migrated from cargo-generate to Copier**: Template now uses Copier for scaffolding and `copier update` for syncing changes to existing components.
- Template files moved into `template/` subdirectory.
- `copier.yml` replaces `cargo-generate.toml`.
- Jinja2 syntax replaces cargo-generate `{{ }}` placeholders.
- Files with runtime `{{ }}` variables (justfile, hurl, CI) use `{% raw %}` blocks.

### Added
- `_skip_if_exists` for `src/lib.rs`, `e2e/*.hurl`, `.gitignore` — these won't be overwritten on `copier update`.

### Removed
- `cargo-generate.toml`

### Migration
Existing components can adopt Copier by creating `.copier-answers.yml` — see spec for details.
```

- [ ] **Step 4: Commit**

```bash
git add -A
GIT_AUTHOR_DATE="2026-03-17T23:40:00+03:00" GIT_COMMITTER_DATE="2026-03-17T23:40:00+03:00" \
git commit -m "chore: clean up empty dirs, update AGENTS.md and CHANGELOG.md"
```

---

### Task 10: Test the template with Copier

- [ ] **Step 1: Install Copier if not present**

```bash
pipx install copier
```

- [ ] **Step 2: Test generating a new project**

```bash
cd /tmp
copier copy /mnt/devenv/workspace/act/act-component-template test-component \
  --data project_name=test-component \
  --data description="A test component"
```

- [ ] **Step 3: Verify generated files**

Check that:
- `Cargo.toml` has `name = "component-test-component"`
- `src/lib.rs` has `name = "test-component"` and `description = "A test component"`
- `justfile` first line has `component_test_component.wasm`, rest has literal `{{act}}` etc.
- `e2e/info.hurl` has `{{baseurl}}` (literal) and `"test-component"` (rendered)
- `e2e/tools.hurl` has `{{baseurl}}` (literal)
- `.github/workflows/ci.yml` has `${{ secrets.GITHUB_TOKEN }}` (literal)
- No `copier.yml`, `AGENTS.md`, `CHANGELOG.md`, `CLAUDE.md`, or `docs/` in generated project
- `.copier-answers.yml` exists

```bash
cd /tmp/test-component
grep 'component-test-component' Cargo.toml
grep 'test-component' src/lib.rs
grep 'A test component' src/lib.rs
grep 'component_test_component' justfile
grep '{{act}}' justfile
grep '{{baseurl}}' e2e/info.hurl
grep 'test-component' e2e/info.hurl
grep 'GITHUB_TOKEN' .github/workflows/ci.yml
test ! -f copier.yml
test ! -f AGENTS.md
test -f .copier-answers.yml
echo "ALL CHECKS PASSED"
```

- [ ] **Step 4: Clean up test project**

```bash
rm -rf /tmp/test-component
```

- [ ] **Step 5: Final commit if any fixes were needed**

```bash
cd /mnt/devenv/workspace/act/act-component-template
# Only if changes were made during testing
git add -A && git diff --cached --quiet || \
GIT_AUTHOR_DATE="2026-03-17T23:45:00+03:00" GIT_COMMITTER_DATE="2026-03-17T23:45:00+03:00" \
git commit -m "fix: template rendering issues found during testing"
```
