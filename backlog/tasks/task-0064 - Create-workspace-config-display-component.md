---
id: task-0064
title: Create workspace config display component
status: To Do
assignee: []
created_date: '2026-01-03 21:35'
updated_date: '2026-01-03 21:35'
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
- [ ] `WorkspaceConfig` component created in `gui/src/components/workspace_config/mod.rs`
- [ ] Component displays each app as a detailed card with all fields
- [ ] App name shown as card header
- [ ] Git URL displayed with label
- [ ] Local path shown (or default path if not specified)
- [ ] Sync interval displayed (or default value if not specified)
- [ ] Kubernetes context shown if specified, otherwise "Not specified"
- [ ] Uses Tailwind CSS styling consistent with existing components
- [ ] Handles empty config state gracefully
- [ ] Component is exported in `gui/src/components/mod.rs`
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

