---
id: task-0064
title: Create workspace config display component
status: Done
assignee: []
created_date: '2026-01-03 21:35'
updated_date: '2026-01-04 02:15'
labels:
  - ui
  - components
  - frontend
dependencies:
  - task-0062
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create a reusable Dioxus component that displays workspace configuration (`WorkstationConfig`) as detailed cards. Each card shows all fields from an `AppReference` entry including name, git URL, local path, sync interval, and Kubernetes context.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 `WorkspaceConfig` component created in `gui/src/components/workspace_config/mod.rs`
- [ ] #2 Component displays each app as a detailed card with all fields
- [ ] #3 App name shown as card header
- [ ] #4 Git URL displayed with label
- [ ] #5 Local path shown (or default path if not specified)
- [ ] #6 Sync interval displayed (or default value if not specified)
- [ ] #7 Kubernetes context shown if specified, otherwise "Not specified"
- [ ] #8 Uses Tailwind CSS styling consistent with existing components
- [ ] #9 Handles empty config state gracefully
- [ ] #10 Component is exported in `gui/src/components/mod.rs`
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
- Create `gui/src/components/workspace_config/mod.rs` module
- Define component props to receive `WorkstationConfig` (Vec<AppReference>)
- Implement card layout using Tailwind CSS (similar to PodList/PortForwardItem)
- Display all AppReference fields with proper labels
- Show default values for optional fields (local_path, sync_interval)
- Handle empty config with appropriate message
- Export component in components module
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
(To be filled during implementation)
<!-- SECTION:NOTES:END -->
