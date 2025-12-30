---
id: decision-0010
title: Local-First Philosophy
date: '2025-12-30 20:54'
status: accepted
---
## Context

Existing Kubernetes tools (Argo CD, Flux) require cluster-side controllers and continuous reconciliation, which is:
- Heavy and complex for local development
- Requires cluster permissions and installation
- Creates "magic" background processes users don't control
- Hostile to local development workflows
- Overkill for simple app management

We need an approach that:
- Works entirely from the local machine
- Requires no cluster-side components
- Gives users explicit control over lifecycle
- Is simple and easy to reason about

## Decision

Adopt a local-first philosophy where:
- All orchestration happens client-side (on the user's machine)
- No cluster-side controllers or operators required
- State is stored locally (Git repository + JSON cache)
- User explicitly controls when apps start/stop
- No background reconciliation or "magic" automation
- Works entirely from local machine using standard Kubernetes APIs

## Consequences

**Positive:**
- Simple mental model - everything happens locally
- No cluster-side installation or permissions required
- User has full control over lifecycle
- Works offline (with local Git repos)
- Easier to debug and reason about
- No "magic" background processes
- Lower barrier to entry

**Negative:**
- Requires local machine to be running for operations
- No automatic reconciliation if cluster state changes externally
- User must manually trigger sync/refresh operations
- Less "hands-off" than GitOps tools
- State can drift if not managed carefully

**References:**
- doc-0004 (Talking Points) - Architecture Principles section
- doc-0002 (Features) - Architecture Principles section
- doc-0005 (Description) - Local-first design rationale