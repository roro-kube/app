# Mise Task Authoring

Create schema-valid, deterministic mise tasks. Tasks declare intent in `.mise/tasks.toml`, shell scripts contain behavior in `.mise/shell/`.

## Core Rules

- File tasks (default): Use for any logic
- Inline tasks: Only single commands, no conditionals/loops/pipes
- Schema header: `#:schema ./schema/mise-task.json`
- Shell scripts: Must start with `#!/usr/bin/env bash` and `set -euo pipefail`

## File Task (Default)

**.mise/tasks.toml**:
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

1. Show `.mise/tasks.toml` entry
2. Show `.mise/shell/script.sh` (if file task)
3. Confirm compliance
4. Provide test command: `mise run task-name`

Wait for approval before executing.