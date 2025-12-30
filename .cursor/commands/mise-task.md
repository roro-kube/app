# Mise Task Authoring

Create schema-valid, deterministic mise tasks. Tasks declare intent in logically grouped files under `.mise/` (e.g., `build.toml`, `run.toml`, `assets.toml`, `quality.toml`), shell scripts contain behavior in `.mise/shell/`.

## Core Rules

- File tasks (default): Use for any logic
- Inline tasks: Only single commands, no conditionals/loops/pipes
- Schema header: `#:schema ./schema/mise-task.json`
- Shell scripts: Must start with `#!/usr/bin/env bash` and `set -euo pipefail`

## Task File Organization

Tasks are organized into logically grouped files under `.mise/`:

- **`.mise/build.toml`** - Build-related tasks (build-workspace, build-persistence, build-domain, build-core, build-assets, build-gui)
- **`.mise/run.toml`** - Runtime/execution tasks (run-gui, backlog-server)
- **`.mise/assets.toml`** - Asset-related tasks (icons, tailwind, assets-watch)
- **`.mise/quality.toml`** - Code quality tasks (fmt, clippy, test, quality, npm-audit, file-size-check)

All files are included via `mise.toml`'s `task_config.includes`. When creating a task, add it to the appropriate file based on its purpose. Mise handles cross-file dependencies automatically.

## File Task (Default)

Add tasks to the appropriate file based on their purpose. For example, a build task would go in `.mise/build.toml`:

**.mise/build.toml** (or other appropriate file):
```toml
#:schema ./schema/mise-task.json

[task-name]
description = "Clear purpose statement"
run = "bash .mise/shell/task-name.sh"
```

**.mise/shell/task-name.sh**:
```bash
#!/usr/bin/env bash
set -euo pipefail

# Implementation
```

## Inline Task (Restricted)

Only for single, simple commands:
```toml
[version]
description = "Display mise version"
run = "mise --version"
```

## Task Properties

```toml
[task-name]
description = "Purpose"              # Required
run = "command"                      # Required
depends = ["other-task"]             # Optional
env = { KEY = "value" }              # Optional
```

## Naming

- Use `kebab-case`
- One task = one responsibility
- Description states purpose, not implementation

## Anti-Patterns

❌ Multi-line inline tasks
❌ Control flow in inline tasks
❌ Scripts outside `.mise/shell/`
❌ Missing schema header
❌ Missing descriptions

## Output Format

When creating a task:

1. Show the appropriate task file entry (e.g., `.mise/build.toml`, `.mise/assets.toml`, etc.) based on task purpose
2. Show `.mise/shell/script.sh` (if file task)
3. Confirm compliance
4. Provide test command: `mise run task-name`

Wait for approval before executing.