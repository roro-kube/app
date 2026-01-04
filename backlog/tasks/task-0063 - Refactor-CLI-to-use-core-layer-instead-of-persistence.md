---
id: task-0063
title: Refactor CLI to use core layer instead of persistence
status: Done
assignee: []
created_date: '2026-01-03 21:35'
updated_date: '2026-01-03 22:17'
labels:
  - cli
  - refactor
  - backend
dependencies:
  - task-0062
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Refactor CLI application to use core layer APIs instead of directly accessing persistence layer. This ensures proper architecture where CLI and GUI both go through core layer, eliminating duplication and maintaining single source of truth for persistence access.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 `cli/src/main.rs` uses `roro_core` instead of `roro_persistence::load_workstation_config()`
- [ ] #2 `cli/src/commands/sync.rs` uses core APIs instead of persistence functions
- [ ] #3 Error handling uses `CoreError` instead of `PersistenceError`
- [ ] #4 `cli/Cargo.toml` removes `roro_persistence` dependency
- [ ] #5 CLI behavior remains unchanged after refactoring
- [ ] #6 All CLI commands still work correctly
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
- Update `cli/src/main.rs` to import and use core config API
- Update `cli/src/commands/sync.rs` to use core git and config APIs
- Replace `PersistenceError` with `CoreError` in error handling
- Remove `roro_persistence` from `cli/Cargo.toml` dependencies
- Verify CLI commands still function correctly
- Test sync command with actual repository
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
(To be filled during implementation)
<!-- SECTION:NOTES:END -->
