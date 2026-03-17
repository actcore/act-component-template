# Copier Template Migration

## Goal

Migrate act-component-template from cargo-generate to Copier, enabling `copier update` for existing components to pull template changes.

## Template Structure

```
act-component-template/
  copier.yml                # Copier config (replaces cargo-generate.toml)
  AGENTS.md                 # Template repo docs (not copied to projects)
  CLAUDE.md -> AGENTS.md
  CHANGELOG.md
  template/                 # _subdirectory — only this gets copied
    Cargo.toml
    README.md
    src/lib.rs
    e2e/info.hurl
    e2e/tools.hurl
    justfile
    wit/
      world.wit
      deps.toml
    .github/
      workflows/ci.yml
      dependabot.yml
    .gitignore
    .cargo/config.toml
    prek.toml
    rust-toolchain.toml
    LICENSE-MIT
    LICENSE-APACHE
```

## copier.yml

```yaml
_min_copier_version: "9.4.0"
_subdirectory: template
_skip_if_exists:
  - src/lib.rs
  - e2e/*.hurl
  - .gitignore

project_name:
  type: str
  help: Component name (e.g. my-tool)

description:
  type: str
  help: Component description
```

No `_templates_suffix` — all files are processed by Jinja2. Files with literal `{{ }}` (justfile, hurl, GitHub Actions CI) use `{% raw %}` blocks.

## Placeholder Mapping

| cargo-generate | Copier (Jinja2) |
|---|---|
| `{{project-name}}` | `{{ project_name }}` |
| `{{crate_name}}` | `{{ project_name \| replace('-', '_') }}` |
| `{{description}}` | `{{ description }}` |

## Jinja2 / Runtime Variable Conflicts

Files with both Copier and runtime variables use `{% raw %}` blocks:

### justfile

```just
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

### e2e/info.hurl

```
GET {% raw %}{{baseurl}}{% endraw %}/info
HTTP 200
[Asserts]
jsonpath "$['std:name']" == "{{ project_name }}"
jsonpath "$['std:version']" isString
```

### e2e/tools.hurl (no copier vars — whole file raw)

```
{% raw %}
POST {{baseurl}}/tools
HTTP 200
[Asserts]
jsonpath "$.tools" count >= 1
{% endraw %}
```

### .github/workflows/ci.yml

GitHub Actions uses `${{ secrets.GITHUB_TOKEN }}` which Jinja2 would try to evaluate. Wrap the entire file in `{% raw %}` since it has no Copier variables.

### Notes

- `wit/deps.lock` is NOT included in the template — `just init` (wit-deps) regenerates it.
- `.gitignore` is in `_skip_if_exists` — downstream projects may add custom entries.
- `.copier-answers.yml` is auto-created by Copier and must be committed to git.

## Retroactive Adoption

For each existing component, create `.copier-answers.yml` so `copier update` works:

```yaml
_commit: <template-commit-hash>
_src_path: https://github.com/actcore/act-component-template
project_name: <component-name>
description: <component-description>
author: ACT contributors
```

Example for sqlite:
```yaml
_commit: abc1234
_src_path: https://github.com/actcore/act-component-template
project_name: sqlite
description: SQLite database operations
author: ACT contributors
```

## Usage

```bash
# New component
copier copy gh:actcore/act-component-template my-component
cd my-component && just init

# Update existing component from template
copier update

# Update without prompts (accept defaults)
copier update --defaults
```

## What Gets Removed

- `cargo-generate.toml` — replaced by `copier.yml`
- All `{{project-name}}` / `{{crate_name}}` cargo-generate syntax — replaced by Jinja2

## _skip_if_exists

`src/lib.rs` and `e2e/*.hurl` are skipped on `copier update` because they contain project-specific logic. All infra files (justfile, CI, prek, etc.) get updated.
