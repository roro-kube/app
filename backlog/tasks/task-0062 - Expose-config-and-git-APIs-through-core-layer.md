---
id: task-0062
title: Expose config and git APIs through core layer
status: To Do
assignee: []
created_date: '2026-01-03 21:35'
labels:
  - config
  - core
  - backend
dependencies: []
priority: high
---

## Description
<!-- SECTION:DESCRIPTION:BEGIN -->
Expose workspace configuration and git operations through the core layer API to provide a unified interface for accessing persistence functionality. This enables CLI and GUI layers to use core instead of directly accessing persistence, avoiding duplication and following proper architecture.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] Core layer exposes `load_workstation_config()` function that wraps persistence layer
- [ ] Core layer exposes `get_config_path_string()` function that wraps persistence layer
- [ ] Core layer exposes `sync_repository()` function that wraps persistence git operations
- [ ] All functions convert `PersistenceError` to `CoreError` appropriately
- [ ] API is exported in `core/src/api/mod.rs` and `core/src/lib.rs`
- [ ] Unit tests verify error conversion and API functionality
<!-- AC:END -->

## Implementation Plan
<!-- SECTION:PLAN:BEGIN -->
- Create `core/src/api/config.rs` module
- Implement `load_workstation_config()` wrapper with error conversion
- Implement `get_config_path_string()` wrapper with error conversion
- Implement `sync_repository()` wrapper with error conversion
- Export config module in `core/src/api/mod.rs`
- Re-export config API in `core/src/lib.rs`
- Add unit tests for error conversion paths
<!-- SECTION:PLAN:END -->

## Implementation Notes
<!-- SECTION:NOTES:BEGIN -->
(To be filled during implementation)
<!-- SECTION:NOTES:END -->

