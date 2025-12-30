---
id: decision-0013
title: Explicit Lifecycle Management
date: '2025-12-30 20:54'
status: accepted
---
## Context

Many Kubernetes tools (Argo CD, Flux) use continuous reconciliation - they constantly watch Git and automatically apply changes. This creates:
- "Magic" behavior where changes happen automatically
- Loss of user control over when things happen
- Background processes that are hard to reason about
- Conflicts with local development workflows

We need a model where users have explicit control over when apps are deployed, updated, or stopped.

## Decision

Adopt explicit lifecycle management where:
- User explicitly controls when apps start/stop via UI actions
- No background reconciliation or automatic updates
- No "magic" automation - all actions are user-initiated
- User owns the lifecycle - they decide when to sync, deploy, update
- Changes in Git don't automatically apply - user must trigger sync/deploy

This is similar to Docker Compose's explicit `docker-compose up` model.

## Consequences

**Positive:**
- User has full control and visibility
- Predictable behavior - no surprises from background processes
- Easier to debug - user knows exactly what triggered an action
- Works well with local development (no conflicts with external changes)
- Simple mental model - user actions cause changes

**Negative:**
- Requires manual action for updates (less "hands-off")
- User must remember to sync/deploy changes
- No automatic drift detection or correction
- Less suitable for production GitOps workflows (though that's not the target)

**References:**
- doc-0004 (Talking Points) - Explicit Lifecycle section
- doc-0002 (Features) - Architecture Principles section
- doc-0005 (Description) - No magic reconciliation principle