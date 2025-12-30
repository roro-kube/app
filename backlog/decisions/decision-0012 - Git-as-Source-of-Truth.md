---
id: decision-0012
title: Git as Source of Truth
date: '2025-12-30 20:54'
status: accepted
---
## Context

Configuration (app definitions, manifests, environment values) needs to be:
- Version controlled for change history
- Collaborative (multiple team members can edit)
- Auditable (who changed what and when)
- Recoverable (can revert to previous states)
- Portable (works across different machines)

We need a single source of truth for all configuration that supports these requirements.

## Decision

Use Git as the single source of truth for all configuration. Specifically:
- All app configurations, manifests, and environment values are stored in Git
- Local JSON cache is derived from Git (not the other way around)
- Git repository is the authoritative source
- Changes are made by editing files in Git and syncing
- Supports both local Git repos and remote (GitHub, GitLab, etc.)

The local runtime cache is ephemeral and can be rebuilt from Git at any time.

## Consequences

**Positive:**
- Built-in version control and history
- Easy collaboration via Git workflows (branches, PRs)
- Full audit trail of changes
- Can revert to any previous state
- Works with existing Git infrastructure
- No additional versioning system needed
- Industry-standard approach

**Negative:**
- Requires Git knowledge from users
- Need to handle merge conflicts
- Git operations add latency compared to direct file access
- Need to manage Git repository state (clean/dirty, branches)
- Authentication complexity for private repositories

**References:**
- doc-0004 (Talking Points) - Git-Backed Configuration section
- doc-0005 (Description) - Git as source of truth
- doc-0002 (Features) - Key Differentiators section