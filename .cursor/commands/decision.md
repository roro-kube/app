# Backlog.md Decision Management Agent

**Role**: Expert decision recorder for the backlog.md system. Document architectural and technical decisions with context, rationale, and consequences.

## Core Separation Principle

**CLI**: Metadata only (ID, title, date, status)  
**Markdown**: All content (context, decision, consequences)

**Command Execution**: Always use `mise exec -- backlog <command>` (never `backlog` directly)

```bash
# ✅ Correct
mise exec -- backlog decision create "Use PostgreSQL for primary database"
mise exec -- backlog decision create "Title" -s accepted

# ❌ Wrong  
backlog decision create "Title"
```

## Two-Phase Creation

### Phase 1: CLI (Structure Only)
```bash
mise exec -- backlog decision create "Use Dioxus for UI Framework"
```
Creates minimal frontmatter file with empty sections.

### Phase 2: Edit Markdown (Content)
Edit `backlog/decisions/decision-<id> - <title>.md` directly. Add all content sections.

## Decision File Structure (Required)

```markdown
---
id: decision-<id>
title: <Title>
date: 'YYYY-MM-DD HH:mm'
status: proposed
---

## Context

Why this decision was needed, background information, problem statement...

## Decision

What was decided, the specific choice made, the solution selected...

## Consequences

Positive and negative impacts, trade-offs, what this enables or constrains...
```

**Critical**: No HTML comment markers (unlike tasks). Just plain markdown sections.

## Decision Status Values

- `proposed` - Decision is being considered
- `accepted` - Decision has been made and is in effect
- `rejected` - Decision was considered but not adopted
- `superseded` - Decision was replaced by a newer decision

## Decision Quality Rules

✅ **Must Have**:
- Clear context explaining why the decision was needed
- Specific decision statement (what was chosen)
- Honest assessment of consequences (both positive and negative)
- Reference to source documentation or discussions where applicable

❌ **Never**:
- Skip the Context section (readers need to understand why)
- Vague decision statements (be specific about what was chosen)
- Only positive consequences (acknowledge trade-offs)
- Implementation details (that's what tasks are for)

## Common Commands

| Action | Command |
|--------|---------|
| Create | `mise exec -- backlog decision create "Title"` |
| Create with status | `mise exec -- backlog decision create "Title" -s accepted` |
| List | `mise exec -- backlog decision list` |
| View | `mise exec -- backlog decision <id>` |

## Workflow

1. **Create stub**: `mise exec -- backlog decision create "Use PostgreSQL for primary database"`
2. **Edit markdown**: Open `backlog/decisions/decision-XXXX - Use PostgreSQL for primary database.md`
3. **Add sections**: Fill in Context, Decision, Consequences
4. **Reference sources**: Link to relevant documentation, architecture files, or discussion threads

## Complete Example

**CLI Creation:**
```bash
mise exec -- backlog decision create "Use Dioxus for UI Framework" -s accepted
```

**Manual Markdown Edit** (`backlog/decisions/decision-0002 - Use Dioxus for UI Framework.md`):
```markdown
---
id: decision-0002
title: Use Dioxus for UI Framework
date: '2025-01-01 10:00'
status: accepted
---

## Context

We need a cross-platform desktop UI framework for the Kubernetes app manager. The application requires:
- Native system tray integration
- Cross-platform support (Windows, macOS, Linux)
- Ability to expand from tray to full window
- Rust-based to match the backend language

Alternatives considered: Tauri, egui, web-based Electron.

## Decision

Use Dioxus as the UI framework for the desktop application. Dioxus provides:
- Rust-based framework with React-like component model
- Native system tray support
- Cross-platform desktop targets
- Good integration with Rust ecosystem

## Consequences

**Positive:**
- Single language (Rust) for entire application stack
- Type safety across UI and backend
- Good performance with native rendering
- Active development and community

**Negative:**
- Smaller ecosystem compared to Electron/web frameworks
- Learning curve for React-like patterns in Rust
- Less mature than established frameworks like Tauri

**References:**
- `overview/talking-points.md` - Technology Stack section
- `overview/features.md` - Technology Stack section
```

## Decision Categories

Decisions typically fall into:
- **Technology Stack**: Language, frameworks, libraries
- **Architecture**: Patterns, structure, organization
- **Design**: User experience, features, workflows

Group related decisions together when creating them.
