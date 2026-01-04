---
id: task-0065
title: Create Settings page for workspace config
status: Done
assignee: []
created_date: '2026-01-03 21:35'
updated_date: '2026-01-04 02:16'
labels:
  - ui
  - settings
  - frontend
dependencies:
  - task-0064
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create a new Settings page that loads workspace configuration from core layer and displays it using the WorkspaceConfig component. The page provides a dedicated view for viewing and managing workspace configuration.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Settings page component created in `gui/src/pages/settings.rs`
- [ ] #2 Page uses `use_resource` to load workspace config via core layer API
- [ ] #3 Page displays `WorkspaceConfig` component
- [ ] #4 Page has title "Settings" or "Workspace Configuration"
- [ ] #5 Page layout is consistent with Home page styling
- [ ] #6 Loading state is displayed while config is being fetched
- [ ] #7 Error state displays user-friendly error message
- [ ] #8 Page is exported in `gui/src/pages/mod.rs`
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
- Create `gui/src/pages/settings.rs` module
- Implement Settings page component using Dioxus
- Use `use_resource` hook to call core `load_workstation_config()` API
- Display WorkspaceConfig component when data is loaded
- Add loading and error state handling
- Style page with Tailwind CSS consistent with Home page
- Export page in pages module
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
(To be filled during implementation)
<!-- SECTION:NOTES:END -->
