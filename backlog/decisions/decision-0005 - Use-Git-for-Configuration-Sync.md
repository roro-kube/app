---
id: decision-0005
title: Use Git for Configuration Sync
date: '2025-12-30 20:53'
status: accepted
---
## Context

The application needs to store and sync configuration (app definitions, manifests, environment values) in a versioned, collaborative way. Requirements:
- Version control for configuration changes
- Collaborative editing by teams
- Support for local and remote repositories
- Authentication for private repositories (GitHub, GitLab)
- Automatic sync and refresh capabilities

Alternatives considered: Local file system only (no collaboration), cloud storage APIs (adds complexity, vendor lock-in), database (overkill, not versioned).

## Decision

Use Git as the storage and sync mechanism for all configuration. Specifically:
- Use `git2` or `gix` crate for Git operations
- Store all app configurations, manifests, and environment values in Git repositories
- Support both local and remote Git repositories
- Enable automatic sync with configurable intervals
- Support authenticated access for private repositories

## Consequences

**Positive:**
- Built-in version control and history
- Familiar tooling for developers
- Easy collaboration via pull requests and reviews
- No additional infrastructure required
- Works offline with local repositories
- Industry-standard approach

**Negative:**
- Requires Git knowledge from users
- Need to handle merge conflicts
- Authentication complexity for private repos
- Git operations can be slower than direct file access
- Need to handle Git repository state (clean/dirty)

**References:**
- doc-0004 (Talking Points) - Git-Backed Configuration section
- doc-0005 (Description) - Configuration Structure section
- doc-0002 (Features) - Technology Stack section