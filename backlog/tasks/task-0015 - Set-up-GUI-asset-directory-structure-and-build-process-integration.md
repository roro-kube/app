---
id: task-0015
title: Set up GUI asset directory structure and build process integration
status: To Do
assignee: []
created_date: '2025-12-29 22:59'
labels:
  - frontend
  - gui
  - assets
dependencies:
  - task-0011
  - task-0014
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Set up the complete GUI asset directory structure and integrate the build process so that icons and Tailwind CSS are built before the GUI application runs. This includes creating the asset directory structure, setting up build process coordination (via Taskfile.yml or similar), and ensuring assets are available when the GUI application needs them. This establishes the complete asset management workflow.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria

<!-- AC:BEGIN -->
- [ ] `gui/assets/` directory exists with proper structure
- [ ] `gui/assets/icons/` directory exists for SVG icons
- [ ] `gui/assets/branding/` directory exists for logos and app icons
- [ ] `gui/assets/tailwind.css` location is established (from task-0014)
- [ ] Build process coordination exists (Taskfile.yml, Makefile, or similar)
- [ ] Build process includes mise task to build icons (executes `mise run icons-build`)
- [ ] Build process includes mise task to build Tailwind CSS (executes `mise run tailwind-build`)
- [ ] Build process can run both icon and Tailwind builds in sequence
- [ ] GUI application can access assets using `asset!()` macro and `include_str!()` macro
- [ ] Complete build workflow successfully prepares all assets before GUI compilation
- [ ] Development workflow supports watch mode for assets (optional but recommended)
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
(Added only after status changes to "In Progress")
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
(Added only after implementation is complete)
<!-- SECTION:NOTES:END -->

