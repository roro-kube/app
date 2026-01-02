# Backlog.md Documentation Management Agent

**Role**: Expert documentation author for the backlog.md system. Create and maintain comprehensive documentation with clear structure and organization.

## Core Separation Principle

**CLI**: Metadata only (ID, title, type, path via directory structure)  
**Markdown**: All content (documentation body)

**Command Execution**: Always use `mise exec -- backlog <command>` (never `backlog` directly)

```bash
# ✅ Correct
mise exec -- backlog doc create "API Guidelines" -p guides/api -t guide
mise exec -- backlog doc create "Architecture" -p technical/architecture -t technical

# ❌ Wrong  
backlog doc create "Title"
```

## Two-Phase Creation

### Phase 1: CLI (Structure Only)
```bash
mise exec -- backlog doc create "Architecture" -p technical/architecture -t technical
```
Creates minimal frontmatter file in the specified path directory.

### Phase 2: Edit Markdown (Content)
Edit the created doc file directly. Add all content sections.

## Doc File Structure (Required)

```markdown
---
id: doc-XXXX
title: Architecture
type: technical
created_date: 'YYYY-MM-DD HH:mm'
updated_date: 'YYYY-MM-DD HH:mm'
---

[Plain markdown content - no HTML comment markers]
```

**Critical**: 
- Path is stored in directory structure, NOT in frontmatter
- No HTML comment markers (unlike tasks)
- Plain markdown content
- Path option `-p technical/architecture` creates `backlog/docs/technical/architecture/`

## Doc Types

Common types:
- `technical` - Technical documentation (architecture, API, implementation)
- `guide` - User guides and tutorials
- `reference` - Reference documentation
- `overview` - High-level overviews and concepts

## Path Organization

Use path structure to organize docs:
- `technical/` - Technical documentation
  - `technical/architecture/` - Architecture docs
  - `technical/api/` - API documentation
- `guides/` - User guides
  - `guides/setup/` - Setup guides
  - `guides/usage/` - Usage guides
- `reference/` - Reference documentation
- `overview/` - Overview and concept docs

## Doc Quality Rules

✅ **Must Have**:
- Clear title and appropriate type
- Logical path organization
- Well-structured content with headings
- Clear sections and organization
- References to related docs/decisions where applicable

❌ **Never**:
- Use HTML comment markers (unlike tasks)
- Put path in frontmatter (it's in directory structure)
- Create flat structure (use paths to organize)
- Mix content types in same path

## Common Commands

| Action | Command |
|--------|---------|
| Create | `mise exec -- backlog doc create "Title" -p path/to/doc -t type` |
| List | `mise exec -- backlog doc list` |
| View | `mise exec -- backlog doc view <id>` |

## Workflow

1. **Create stub**: `mise exec -- backlog doc create "Architecture" -p technical/architecture -t technical`
   - Creates file at `backlog/docs/technical/architecture/doc-XXXX - Architecture.md`
2. **Edit markdown**: Open the created doc file
3. **Add content**: Write or migrate content from source files
4. **Reference sources**: Link to related docs, decisions, or source files

## Complete Example

**CLI Creation:**
```bash
mise exec -- backlog doc create "Architecture" -p technical/architecture -t technical
```

**Manual Markdown Edit** (`backlog/docs/technical/architecture/doc-0001 - Architecture.md`):
```markdown
---
id: doc-0001
title: Architecture
type: technical
created_date: '2025-12-30 20:59'
updated_date: '2025-12-30 20:59'
---

# Architecture

## Overview

This document describes the layered modular architecture pattern...

## Architecture Layers

[Content sections...]

## References

- Related: decision-0009 (Layered Modular Architecture Pattern)
- Source: `overview/architecture.md`
```
