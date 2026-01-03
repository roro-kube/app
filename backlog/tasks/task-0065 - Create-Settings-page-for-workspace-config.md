---
id: task-0065
title: Create Settings page for workspace config
status: To Do
assignee: []
created_date: '2026-01-03 21:35'
updated_date: '2026-01-03 21:35'
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
- [ ] Settings page component created in `gui/src/pages/settings.rs`
- [ ] Page uses `use_resource` to load workspace config via core layer API
- [ ] Page displays `WorkspaceConfig` component
- [ ] Page has title "Settings" or "Workspace Configuration"
- [ ] Page layout is consistent with Home page styling
- [ ] Loading state is displayed while config is being fetched
- [ ] Error state displays user-friendly error message
- [ ] Page is exported in `gui/src/pages/mod.rs`
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

