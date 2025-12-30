---
id: task-0012
title: Create icon component in GUI crate
status: Done
assignee: []
created_date: '2025-12-29 22:59'
updated_date: '2025-12-30 14:49'
labels:
  - frontend
  - gui
  - icons
dependencies:
  - task-0011
priority: high
ordinal: 8000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create a reusable icon component in the GUI crate that allows Rust code to easily display SVG icons. The component should load icons from the assets directory using `include_str!` and render them in Dioxus components. This provides a clean, type-safe way to use icons throughout the GUI application without hardcoding SVG content in components.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 `gui/src/icons.rs` module exists with Icon component
- [ ] #2 Icon component accepts `name`, optional `class`, and optional `size` parameters
- [ ] #3 Component loads SVG icons from `gui/assets/icons/` using `include_str!`
- [ ] #4 Component renders SVG icons using `dangerous_inner_html` in Dioxus
- [ ] #5 Component applies size classes (defaults to `w-4 h-4` if not specified)
- [ ] #6 Component applies custom classes when provided
- [ ] #7 `get_icon_svg()` function maps icon names to SVG content
- [ ] #8 Component handles missing icons gracefully (returns default icon or error)
- [ ] #9 Icon component can be imported and used in other GUI components
- [ ] #10 At least one icon can be successfully rendered in a test component
<!-- AC:END -->
