---
id: decision-0001
title: Zero Controllers
date: '2025-12-30 20:54'
status: accepted
---
## Context

Many Kubernetes tools (Argo CD, Flux, Acorn) require installing cluster-side controllers or operators. This creates:
- Installation complexity (need cluster permissions)
- Ongoing maintenance (controllers need updates)
- Cluster resource usage
- Security concerns (controllers run with elevated permissions)
- Barrier to entry (can't use tool without cluster access)

For a local-first development tool, cluster-side components are unnecessary overhead.

## Decision

Require zero cluster-side controllers or operators. The application:
- Uses only standard Kubernetes APIs (no custom resources for tool operation)
- Runs entirely client-side (on user's machine)
- No cluster-side installation required
- No custom operators or controllers
- Pure client-side application using kubectl-equivalent APIs

Users can use the tool immediately with any Kubernetes cluster without installation.

## Consequences

**Positive:**
- Zero installation - works with any cluster immediately
- No cluster-side resource usage
- No security concerns from cluster-side components
- Lower barrier to entry
- Works with clusters where you can't install controllers
- Simpler architecture

**Negative:**
- Can't use controller patterns (watch resources, automatic reconciliation)
- All logic must run client-side
- Requires local machine to be running for operations
- No cluster-side state or coordination
- Limited to what standard Kubernetes APIs provide

**References:**
- doc-0004 (Talking Points) - Architecture Principles section
- doc-0002 (Features) - Key Differentiators section
- doc-0005 (Description) - Zero Controllers principle