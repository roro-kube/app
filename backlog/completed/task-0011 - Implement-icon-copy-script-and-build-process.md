---
id: task-0011
title: Implement icon copy script and build process
status: Done
assignee: []
created_date: '2025-12-29 22:59'
updated_date: '2025-12-30 14:17'
labels:
  - frontend
  - assets
  - icons
dependencies:
  - task-0010
priority: high
ordinal: 7000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement the icon copy script and build process that copies icons from the icon library (e.g., Heroicons) to the GUI assets directory. This script maps internal icon names to library icon names and automates the process of copying SVG icons to the correct location. This enables the GUI application to use icons that are managed separately from the Rust code, making it easy to update icons without touching Rust code.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 `icons/copy-icons.js` script exists and copies icons from library to GUI assets
- [ ] #2 Icon mapping file or structure exists that maps internal names to library names (e.g., `home` → `home`)
- [ ] #3 Script copies icons to `gui/assets/icons/` directory
- [ ] #4 Copied icons follow naming convention (e.g., `{name}-outline.svg`)
- [ ] #5 Script handles errors gracefully (missing icons, missing directories, etc.)
- [ ] #6 `package.json` includes a `build` script that runs the copy script
- [ ] #7 Mise task exists for building icons (e.g., `mise run icons-build` executes `npm run build` in icons directory)
- [ ] #8 Running mise task successfully copies icons to GUI assets
- [ ] #9 At least one test icon is successfully copied and accessible in GUI assets directory
<!-- AC:END -->
