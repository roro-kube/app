---
id: decision-0009
title: Layered Modular Architecture Pattern
date: '2025-12-30 20:54'
status: accepted
---
## Context

The application needs a clear structure that:
- Separates concerns (UI, business logic, data access)
- Enables independent testing of each layer
- Allows swapping implementations (e.g., different databases)
- Supports multiple interfaces (GUI, CLI, potentially web)
- Prevents circular dependencies
- Makes the codebase maintainable as it grows

We need an architecture pattern that provides clear boundaries and dependency direction.

## Decision

Adopt a layered modular architecture pattern with the following layers:
- **Application Layer**: GUI, CLI, Web (user interfaces)
- **Core Layer**: Business logic, API, validation, orchestration
- **Domain Layer**: Business entities, types, value objects
- **Persistence Layer**: Data storage, models, store operations

Dependencies flow downward: Application → Core → Domain/Persistence. Never the reverse.

Each layer is a separate Cargo crate in a workspace for modularity and independent development.

## Consequences

**Positive:**
- Clear separation of concerns
- Each layer can be tested independently
- Easy to swap implementations (e.g., different storage backends)
- Prevents circular dependencies through dependency direction rules
- Scalable structure as codebase grows
- Multiple interfaces can share the same Core layer

**Negative:**
- More initial structure and boilerplate
- Need to be disciplined about dependency direction
- Some overhead in data transformation between layers
- Learning curve for team members new to layered architecture

**References:**
- doc-0001 (Architecture) - Complete architecture documentation