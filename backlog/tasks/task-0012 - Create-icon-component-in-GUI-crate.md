---
id: task-0012
title: Create icon component in GUI crate
status: To Do
assignee: []
created_date: '2025-12-29 22:59'
labels:
  - frontend
  - gui
  - icons
dependencies:
  - task-0011
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create a reusable icon component in the GUI crate that allows Rust code to easily display SVG icons. The component should load icons from the assets directory using `include_str!` and render them in Dioxus components. This provides a clean, type-safe way to use icons throughout the GUI application without hardcoding SVG content in components.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria

<!-- AC:BEGIN -->
- [ ] `gui/src/icons.rs` module exists with Icon component
- [ ] Icon component accepts `name`, optional `class`, and optional `size` parameters
- [ ] Component loads SVG icons from `gui/assets/icons/` using `include_str!`
- [ ] Component renders SVG icons using `dangerous_inner_html` in Dioxus
- [ ] Component applies size classes (defaults to `w-4 h-4` if not specified)
- [ ] Component applies custom classes when provided
- [ ] `get_icon_svg()` function maps icon names to SVG content
- [ ] Component handles missing icons gracefully (returns default icon or error)
- [ ] Icon component can be imported and used in other GUI components
- [ ] At least one icon can be successfully rendered in a test component
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
(Added only after status changes to "In Progress")
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
(Added only after implementation is complete)
<!-- SECTION:NOTES:END -->

