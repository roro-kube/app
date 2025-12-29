---
id: task-0014
title: Integrate Tailwind CSS build process and asset loading
status: To Do
assignee: []
created_date: '2025-12-29 22:59'
labels:
  - frontend
  - gui
  - css
dependencies:
  - task-0013
priority: high
---

## Description (the why)

Integrate the Tailwind CSS build process to compile CSS and load it in the GUI application. This includes configuring Vite to output compiled CSS to the GUI assets directory and integrating the stylesheet into the Dioxus application. This enables the GUI to use Tailwind utility classes for styling.

## Acceptance Criteria (the what)

- [ ] Vite build configuration outputs compiled CSS to `gui/assets/tailwind.css`
- [ ] Mise task exists for building Tailwind CSS (e.g., `mise run tailwind-build` executes `npm run build` in tailwind directory)
- [ ] Running mise task successfully compiles CSS
- [ ] Compiled `tailwind.css` file exists in `gui/assets/` directory
- [ ] GUI application can load Tailwind CSS using `document::Stylesheet` with `asset!()` macro
- [ ] Tailwind CSS is included in the main app layout component
- [ ] Tailwind utility classes work in GUI components (e.g., `class: "min-h-screen bg-white"`)
- [ ] Dark mode classes work when dark mode is enabled (e.g., `dark:bg-zinc-900`)
- [ ] CSS is properly purged of unused styles during build
- [ ] Build process can be run repeatedly without errors

