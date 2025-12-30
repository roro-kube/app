---
id: decision-0014
title: Separation of Concerns
date: '2025-12-30 20:54'
status: accepted
---
## Context

The application manages multiple concerns:
- Configuration storage and versioning (Git)
- Runtime state caching and reconciliation
- Kubernetes cluster interactions
- User interface (tray and window)

These concerns have different lifecycles, update frequencies, and failure modes. Mixing them creates tight coupling and makes the system harder to reason about and maintain.

## Decision

Separate the application into distinct layers with clear responsibilities:

- **State Layer**: Git-stored configuration (source of truth, versioned, collaborative)
- **Runtime Layer**: Local state cache with reconciliation (ephemeral, fast access)
- **Kubernetes Layer**: Actual cluster state (external, eventually consistent)
- **UI Layer**: Dioxus tray + full window interface (user interaction, display)

Each layer has a single, well-defined responsibility and communicates with others through clear interfaces.

## Consequences

**Positive:**
- Clear boundaries make system easier to understand
- Each layer can be developed and tested independently
- Can swap implementations (e.g., different storage backends)
- Easier to debug - issues are isolated to specific layers
- Better separation allows for different update frequencies per layer

**Negative:**
- Need to maintain consistency across layers
- Data transformation overhead between layers
- More complex than monolithic approach
- Need to handle layer synchronization and reconciliation

**References:**
- doc-0005 (Description) - Core Architecture Principles section
- doc-0004 (Talking Points) - Architecture Principles section