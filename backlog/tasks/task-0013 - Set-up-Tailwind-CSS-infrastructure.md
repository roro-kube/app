---
id: task-0013
title: Set up Tailwind CSS infrastructure
status: In Progress
assignee: []
created_date: '2025-12-29 22:59'
updated_date: '2025-12-30 14:49'
labels:
  - frontend
  - assets
  - css
dependencies:
  - task-0009
priority: high
ordinal: 1000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Set up the infrastructure for Tailwind CSS integration, including directory structure, configuration files, and dependencies. Tailwind CSS provides utility-first styling that keeps the CSS bundle small through purging unused styles. This task establishes the foundation for styling the GUI application with Tailwind CSS, separate from the Rust code.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 `tailwind/` directory exists at project root
- [ ] #2 `tailwind/package.json` exists with Tailwind CSS, PostCSS, and Vite dependencies
- [ ] #3 `tailwind/tailwind.config.js` exists with configuration
- [ ] #4 Tailwind config includes content paths for scanning Rust files (`../gui/src/**/*.rs`)
- [ ] #5 Tailwind config includes theme extension for custom colors (if needed)
- [ ] #6 Tailwind config enables dark mode with `"class"` strategy
- [ ] #7 `tailwind/postcss.config.js` exists with PostCSS configuration
- [ ] #8 `tailwind/vite.config.ts` exists with Vite build configuration
- [ ] #9 `tailwind/src/tailwind.css` exists with Tailwind directives (`@tailwind base`, `@tailwind components`, `@tailwind utilities`)
- [ ] #10 `package.json` includes a build script
- [ ] #11 Mise task exists for installing Tailwind dependencies (e.g., `mise run tailwind-install` executes `npm install` in tailwind directory)
- [ ] #12 All dependencies are installable via mise task
<!-- AC:END -->
