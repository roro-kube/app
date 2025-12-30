---
id: task-0015
title: Set up GUI asset directory structure and build process integration
status: Done
assignee: []
created_date: '2025-12-29 22:59'
updated_date: '2025-12-30 17:47'
labels:
  - frontend
  - gui
  - assets
dependencies:
  - task-0011
  - task-0014
priority: high
ordinal: 12000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Set up the complete GUI asset directory structure and integrate the build process so that icons and Tailwind CSS are built before the GUI application runs. This includes creating the asset directory structure, setting up build process coordination (via Taskfile.yml or similar), and ensuring assets are available when the GUI application needs them. This establishes the complete asset management workflow.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 `gui/assets/` directory exists with proper structure
- [ ] #2 `gui/assets/icons/` directory exists for SVG icons
- [ ] #3 `gui/assets/branding/` directory exists for logos and app icons
- [ ] #4 `gui/assets/tailwind.css` location is established (from task-0014)
- [ ] #5 Build process coordination exists (Taskfile.yml, Makefile, or similar)
- [ ] #6 Build process includes mise task to build icons (executes `mise run icons-build`)
- [ ] #7 Build process includes mise task to build Tailwind CSS (executes `mise run tailwind-build`)
- [ ] #8 Build process can run both icon and Tailwind builds in sequence
- [ ] #9 GUI application can access assets using `asset!()` macro and `include_str!()` macro
- [ ] #10 Complete build workflow successfully prepares all assets before GUI compilation
- [ ] #11 Development workflow supports watch mode for assets (optional but recommended)
<!-- AC:END -->
