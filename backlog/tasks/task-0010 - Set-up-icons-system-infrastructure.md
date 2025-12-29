---
id: task-0010
title: Set up icons system infrastructure
status: To Do
assignee: []
created_date: '2025-12-29 22:59'
labels:
  - frontend
  - assets
  - icons
dependencies:
  - task-0009
priority: high
---

## Description (the why)

Set up the infrastructure for managing SVG icons used throughout the GUI application. This includes creating the icons directory structure, installing icon library dependencies (e.g., Heroicons), and establishing the foundation for icon management. Icons should be easy to add/remove, consistent in style, optimized for size, and accessible from Rust code. This task establishes the directory structure and dependencies needed for the icon system.

## Acceptance Criteria (the what)

- [ ] `icons/` directory exists at project root
- [ ] `icons/package.json` exists with icon library dependency (e.g., heroicons)
- [ ] `icons/branding/` directory exists for custom branding icons
- [ ] `icons/dist/` directory structure exists with subdirectories for `macos/`, `windows/`, and `png/`
- [ ] `icons/scripts/` directory exists for build and verification scripts
- [ ] `package.json` includes a build script that can be run via npm
- [ ] Mise task exists for installing icon dependencies (e.g., `mise run icons-install` executes `npm install` in icons directory)
- [ ] Icon library (e.g., heroicons) is installable via mise task
- [ ] Directory structure matches the architecture specification

