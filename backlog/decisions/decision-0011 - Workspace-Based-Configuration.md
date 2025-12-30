---
id: decision-0011
title: Workspace-Based Configuration
date: '2025-12-30 20:54'
status: accepted
---
## Context

Configuration needs to support:
- Multiple apps in a single workspace
- Per-app configuration files
- Environment-specific values (dev, staging, prod)
- Manifest templates stored relative to each app
- Workspace-level settings (Git remote, cluster context)

A monolithic JSON file becomes unwieldy with many apps. We need a structure similar to how Cargo workspaces organize Rust projects.

## Decision

Use a workspace-based configuration structure similar to Cargo workspaces:

```
.kube-apps/
├── workspace.json          # Workspace metadata
├── apps/
│   ├── my-app/
│   │   ├── app.json       # App definition
│   │   ├── manifests/     # K8s YAML templates
│   │   └── values/        # Environment-specific values
│   │       ├── dev.json
│   │       └── prod.json
│   └── another-app/
└── environments/
    ├── dev.json
    └── prod.json
```

Each app is self-contained with its own directory, making it easy to add/remove apps and manage configurations independently.

## Consequences

**Positive:**
- Scalable structure - easy to add/remove apps
- Clear organization - each app is self-contained
- Familiar pattern (similar to Cargo workspaces)
- Easy to version control (each app can be in separate repo if needed)
- Environment-specific configs are clearly separated
- Manifests stored relative to app (portable)

**Negative:**
- More directory structure to manage
- Need to handle workspace-level vs app-level config precedence
- Slightly more complex than single JSON file
- Need to discover and load all apps in workspace

**References:**
- doc-0005 (Description) - Configuration Structure section
- doc-0004 (Talking Points) - Workspace-Based Configuration section