---
id: task-0014
title: Integrate Tailwind CSS build process and asset loading
status: Done
assignee: []
created_date: '2025-12-29 22:59'
updated_date: '2025-12-30 17:25'
labels:
  - frontend
  - gui
  - css
dependencies:
  - task-0013
priority: high
ordinal: 11000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Integrate the Tailwind CSS build process to compile CSS and load it in the GUI application. This includes configuring Vite to output compiled CSS to the GUI assets directory and integrating the stylesheet into the Dioxus application. This enables the GUI to use Tailwind utility classes for styling.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Vite build configuration outputs compiled CSS to `gui/assets/tailwind.css`
- [ ] #2 Mise task exists for building Tailwind CSS (e.g., `mise run tailwind-build` executes `npm run build` in tailwind directory)
- [ ] #3 Running mise task successfully compiles CSS
- [ ] #4 Compiled `tailwind.css` file exists in `gui/assets/` directory
- [ ] #5 GUI application can load Tailwind CSS using `document::Stylesheet` with `asset!()` macro
- [ ] #6 Tailwind CSS is included in the main app layout component
- [ ] #7 Tailwind utility classes work in GUI components (e.g., `class: "min-h-screen bg-white"`)
- [ ] #8 Dark mode classes work when dark mode is enabled (e.g., `dark:bg-zinc-900`)
- [ ] #9 CSS is properly purged of unused styles during build
- [ ] #10 Build process can be run repeatedly without errors
<!-- AC:END -->
