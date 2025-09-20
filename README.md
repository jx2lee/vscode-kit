# vscode-kit

A small Rust CLI to generate and manage VS Code workspace files (.vscode) for Python projects.

## Install
- Build from source: `cargo install --path .` (binary: `vscode-kit`)
- Or run without installing: `cargo run -- <args>`

## Commands
- `generate`: Create VS Code templates under `<project-root>/.vscode`.
- `validate` (experimental): Validate a template directory structure. Note: currently exits non-zero.

## Usage
Generate:
```
vscode-kit generate \
  --project-root <PATH> \
  [--selected <launch,tasks,settings>] \
  [--preset <python>] \
  [--template-dir <DIR>] \
  [--interactive]
```

Validate:
```
vscode-kit validate \
  --template-root <TEMPLATE_PATH>
```

## Options
- `--project-root <PATH>`: Project root. Files are written to `<PATH>/.vscode/`.
- `--selected`: Which files to create. Comma-separated or repeated. Default: all.
  - Values: `launch`, `tasks`, `settings`
- `--preset <python>`: Template preset. Default: `python`.
- `--template-dir <DIR>`: Optional external templates directory. If present, files here override embedded templates.
- `--interactive`: Prompt before overwriting existing files. When omitted, prompts automatically if running in an interactive TTY; otherwise, skips existing files.

## Behavior
- Creates `.vscode/` when missing.
- Overwrite policy:
  - Interactive (TTY or `--interactive`): prompts before overwriting existing files.
  - Non-interactive (piped/CI): skips files that already exist (no overwrite).
- Prints a summary of created, skipped, and any errors.
- Exit code `0` on success, `1` on error.

## Presets
- `python` (default)
  - `launch.json`: Debug current file or module.
  - `tasks.json`: Create venv, upgrade pip, install `requirements.txt`, run `pytest`, `ruff` check/format.
  - `settings.json`: Enable `pytest`, set `.venv/bin/python`, format on save, organize imports.

## External templates
If you pass `--template-dir <DIR>`, the tool looks for files at:
```
<DIR>/python/launch.json
<DIR>/python/tasks.json
<DIR>/python/settings.json
```
If a file exists there, it is used; otherwise, the embedded default is used.

## Examples
- Generate all defaults into the current project:
  - `vscode-kit generate --project-root .`
- Generate only launch and settings:
  - `vscode-kit generate --project-root . --selected launch,settings`
- Force interactive overwrite prompts:
  - `vscode-kit generate --project-root . --interactive`
- Use external templates:
  - `vscode-kit generate --project-root . --template-dir /path/to/templates`

## Development
- Run tests: `cargo test`
- Run the CLI locally: `cargo run -- generate --project-root .`

Note: `validate` is currently experimental and exits non-zero.
