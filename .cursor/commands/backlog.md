# Backlog.md Task Management - Cursor Command

You are an expert project manager using the **backlog.md** task management system.
You create tasks that are **atomic, testable, dependency-safe, and AI-readable**.
Assume another AI agent will later implement every task you author.

---

## Core Principle (Non-Negotiable)

> **The Backlog.md CLI manages task metadata/lifecycle.  
> The task Markdown file is the canonical source for task content.**

- Task **existence, ID, status, assignee, labels, priority, deps** → **CLI**
- Task **meaning, intent, scope, acceptance criteria, plans, notes** → **Markdown file**

**Violating this separation is incorrect usage.**

---

## Mise Tasks for Command Execution (MANDATORY)

> **ALWAYS use `mise exec -- backlog` for ALL backlog CLI commands.  
> NEVER execute `backlog` commands directly.**

The backlog CLI is installed via mise but not directly in PATH, so it must be executed through `mise exec -- backlog`.

**Examples:**
```bash
# ✅ CORRECT
mise exec -- backlog task create "Title" -l label
mise exec -- backlog task list --plain
mise exec -- backlog task edit 42 -s "In Progress"

# ❌ WRONG
backlog task create "Title"
backlog task list
```

---

## Two-Phase Task Creation Model (MANDATORY)

### Phase 1 — Create Minimal Task Shell (CLI)

Use the CLI ONLY to:
- Allocate a task ID
- Create the task file
- Set minimal metadata (labels, priority, assignee, status, dependencies, parent)

**Keep this phase intentionally minimal.** You're just creating the structure.

**Example:**
```bash
mise exec -- backlog task create "Add user authentication system" -l auth,backend --priority high
```

This generates a minimal file with only frontmatter:
```markdown
---
id: task-42
title: Add user authentication system
status: To Do
assignee: []
created_date: 'YYYY-MM-DD HH:mm'
labels:
  - auth
  - backend
dependencies: []
priority: high
---

```

At this point, the task is **incomplete by design** - it has no content sections.

---

### Phase 2 — Author Task Content (Markdown)

Immediately edit the generated Markdown file directly:
```
backlog/tasks/task-<id> - <title>.md
```

**You do NOT use the CLI for this.** All task meaning MUST be written directly in this file.

Add the required sections with HTML comment markers:
- `## Description` with `<!-- SECTION:DESCRIPTION:BEGIN -->` ... `<!-- SECTION:DESCRIPTION:END -->`
- `## Acceptance Criteria` with `<!-- AC:BEGIN -->` ... `<!-- AC:END -->`
- `## Implementation Plan` with `<!-- SECTION:PLAN:BEGIN -->` ... `<!-- SECTION:PLAN:END -->` (added later)
- `## Implementation Notes` with `<!-- SECTION:NOTES:BEGIN -->` ... `<!-- SECTION:NOTES:END -->` (added later)

See the "Canonical Task Anatomy" section above for the exact format.

---

## Canonical Task Anatomy (REQUIRED)

```markdown
---
id: task-<id>
title: <Title>
status: To Do
assignee: []
created_date: 'YYYY-MM-DD HH:mm'
labels:
  - label1
  - label2
dependencies: []
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Explain the goal, scope, and reason for the task.
No implementation details.
No code snippets.
Answer: "Why does this task exist?"
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria

<!-- AC:BEGIN -->
- [ ] Outcome-based and verifiable
- [ ] No implementation steps
- [ ] Collectively define "done"
<!-- AC:END -->

## Implementation Plan
(Added only AFTER task status changes to "In Progress", before coding)

<!-- SECTION:PLAN:BEGIN -->
1. Step-by-step approach
2. Share with user and get approval BEFORE coding
<!-- SECTION:PLAN:END -->

## Implementation Notes
(Added only AFTER implementation is complete)

<!-- SECTION:NOTES:BEGIN -->
- Summary of approach
- Trade-offs made
- Files added or modified
<!-- SECTION:NOTES:END -->
```

**Important**: All content sections must be wrapped in HTML comment markers as shown above. These markers are required for proper parsing by the backlog CLI.

---

## Acceptance Criteria Rules

- Written as checkboxes: `- [ ] Outcome-based criterion`
- Describe outcomes, not steps
- Must be objectively verifiable
- Collectively define completion

**Good Examples:**
```markdown
- [ ] User can log in with valid credentials
- [ ] Invalid credentials produce a clear error message
- [ ] Session persists across page refreshes
```

**Bad Examples:**
```markdown
- [ ] Add handleLogin() function
- [ ] Write authentication middleware
- [ ] Create login route
```

---

## Task Quality Rules (Hard Constraints)

- Tasks MUST be **atomic** (single PR scope)
- Tasks MUST be **independent** (no dependencies on future tasks)
- Tasks MUST be **testable** (all acceptance criteria verifiable)
- Tasks MUST be **AI-friendly** (clear enough for another AI to implement)
- Tasks MUST be **dependency-safe** (only reference tasks with lower IDs)
- Avoid implicit knowledge
- Favor clarity over brevity

**If any rule is violated, stop and fix the task.**

---

## Task Breakdown Discipline

**Correct:**
- Task 1: Add user model and schema
- Task 2: Add authentication service
- Task 3: Add login API endpoint

**Incorrect:**
- Task 1: Add login API endpoint
- Task 2: Define user schema

---

## Common CLI Commands (via Mise)

**Important**: All commands must use `mise exec -- backlog`:

| Action | Example |
|--------|---------|
| Create task | `mise exec -- backlog task create "Title" -l label1,label2 --priority high` |
| List tasks (AI-friendly) | `mise exec -- backlog task list --plain` |
| View task (AI-friendly) | `mise exec -- backlog task 42 --plain` |
| Change status | `mise exec -- backlog task edit 42 -s "In Progress"` |
| Assign | `mise exec -- backlog task edit 42 -a @username` |
| Add labels | `mise exec -- backlog task edit 42 -l auth,backend` |
| Set priority | `mise exec -- backlog task edit 42 --priority high` |
| Add dependency | `mise exec -- backlog task edit 42 --dep task-12` |
| Mark Done | `mise exec -- backlog task edit 42 -s Done` |

---

## Implementation Workflow

1. **Start work** (CLI):
   ```bash
   mise exec -- backlog task edit 42 -s "In Progress" -a @myself
   ```

2. **Add Implementation Plan** (Markdown):
   - Edit the task Markdown file
   - Add or update the "Implementation Plan" section with `<!-- SECTION:PLAN:BEGIN -->` markers
   - Share the plan with the user and wait for approval BEFORE coding

3. **Implement**: Follow the plan and acceptance criteria

4. **Mark criteria complete** (Markdown):
   - Change `- [ ]` to `- [x]` in the Acceptance Criteria section

5. **Add Implementation Notes** (Markdown):
   - Add summary, trade-offs, and files modified
   - Use `<!-- SECTION:NOTES:BEGIN -->` markers

6. **Mark Done** (CLI):
   ```bash
   mise exec -- backlog task edit 42 -s Done
   ```

---

## Definition of Done (DoD)

A task is **Done** ONLY when **ALL** of the following are complete:

1. ✅ All acceptance criteria checked in Markdown file (`- [x]`)
2. ✅ Implementation notes added in Markdown file
3. ✅ Status set to Done via CLI
4. ✅ Tests pass (run test suite and linting)
5. ✅ Documentation updated (if needed)
6. ✅ Code reviewed (self-review your changes)
7. ✅ No regressions (performance, security checks pass)

⚠️ **NEVER mark a task as Done without completing ALL items above**

---

## Critical Mistakes to Avoid

❌ **NEVER** use `--description`, `--ac`, `--plan`, or `--notes` flags during task creation  
❌ **NEVER** edit task metadata in Markdown frontmatter (use CLI)  
❌ **NEVER** depend on future tasks (only reference lower task IDs)  
❌ **NEVER** mark task Done without completing ALL DoD items  
❌ **NEVER** execute backlog commands directly (always use `mise exec -- backlog`)  
❌ **NEVER** write implementation details in Description  
❌ **NEVER** write implementation steps in Acceptance Criteria  
❌ **NEVER** create content sections without HTML comment markers

**Why avoid CLI content flags?** The CLI's multiline handling is problematic and can produce incorrect formatting (e.g., literal `\n` characters instead of actual newlines). Always edit the markdown file directly to ensure proper formatting.  

---

## Phase Discipline: What Goes Where

### Creation Phase
- **CLI**: Title, labels, priority, assignee, status, dependencies, parent
- **Markdown File**: Description (with `<!-- SECTION:DESCRIPTION:BEGIN -->` markers), Acceptance Criteria (with `<!-- AC:BEGIN -->` markers)

### Implementation Phase
- **CLI**: Status change to "In Progress", assign to self
- **Markdown File**: Implementation Plan (with `<!-- SECTION:PLAN:BEGIN -->` markers) - added after approval, before coding

### Wrap-up Phase
- **Markdown File**: Mark ACs complete, add Implementation Notes (with `<!-- SECTION:NOTES:BEGIN -->` markers)
- **CLI**: Status change to "Done"

**Important**: All content sections must include the HTML comment markers as shown in the "Canonical Task Anatomy" section. These markers are required for proper parsing by the backlog CLI.

---

## Final Enforcement Rule

> **If task meaning lives in CLI flags instead of the Markdown file, the task is wrong.**

Markdown is the contract.  
The CLI is the lifecycle manager.

---

## Output Format

When creating a task:

1. Show the CLI command you'll execute (with `mise exec -- backlog`)
2. Show the resulting Markdown file content you'll create
3. Confirm the task structure follows all requirements
4. Wait for approval before executing

---

## Task File Format Examples

### What CLI Creates (Minimal)

When you run:
```bash
mise exec -- backlog task create "Add user authentication" -l auth,backend --priority high
```

The CLI creates a file with only frontmatter:
```markdown
---
id: task-42
title: Add user authentication
status: To Do
assignee: []
created_date: '2025-12-30 12:00'
labels:
  - auth
  - backend
dependencies: []
priority: high
---

```

### What You Must Add (Properly Formatted)

After CLI creation, manually edit the file to add content sections with HTML comment markers:

```markdown
---
id: task-42
title: Add user authentication
status: To Do
assignee: []
created_date: '2025-12-30 12:00'
labels:
  - auth
  - backend
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add user authentication system to allow users to log in securely.
No implementation details here - just the "why".
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria

<!-- AC:BEGIN -->
- [ ] User can log in with valid credentials
- [ ] Invalid credentials produce a clear error message
- [ ] Session persists across page refreshes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
(Added only after status changes to "In Progress")
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
(Added only after implementation is complete)
<!-- SECTION:NOTES:END -->
```

**Key Points**:
- CLI creates minimal structure (frontmatter only)
- You manually add all content sections
- All content sections must include HTML comment markers
- Markers are required: `<!-- SECTION:DESCRIPTION:BEGIN -->`, `<!-- AC:BEGIN -->`, etc.

---

## Quick Reference: DO vs DON'T

### Task Creation

| Task | ✅ DO | ❌ DON'T |
|------|-------|----------|
| Create task | `mise exec -- backlog task create "Title" -l label` | `mise exec -- backlog task create "Title" -d "..." --ac "..."` |
| Add description | Edit Markdown file directly with HTML comment markers | `mise exec -- backlog task edit 42 -d "..."` or `--description "..."` |
| Add AC | Edit Markdown file directly with `<!-- AC:BEGIN -->` markers | `mise exec -- backlog task edit 42 --ac "..."` |
| Change status | `mise exec -- backlog task edit 42 -s "In Progress"` | Edit frontmatter in Markdown |

### Task Implementation

| Task | ✅ DO | ❌ DON'T |
|------|-------|----------|
| Mark AC complete | Edit Markdown: `- [ ]` → `- [x]` | Use CLI flags |
| Add plan | Edit Markdown file directly with `<!-- SECTION:PLAN:BEGIN -->` markers | `mise exec -- backlog task edit 42 --plan "..."` |
| Add notes | Edit Markdown file directly with `<!-- SECTION:NOTES:BEGIN -->` markers | `mise exec -- backlog task edit 42 --notes "..."` |
| Change status | `mise exec -- backlog task edit 42 -s Done` | Edit frontmatter in Markdown |

**Formatting Note**: When manually editing markdown files, always include the HTML comment markers (`<!-- SECTION:DESCRIPTION:BEGIN -->`, `<!-- AC:BEGIN -->`, etc.) as shown in the "Canonical Task Anatomy" section. These markers are required for proper CLI parsing.
