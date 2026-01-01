---
id: task-0032
title: Create template engine abstraction
status: To Do
assignee: []
created_date: '2026-01-01 16:37'
updated_date: '2026-01-01 16:42'
labels:
  - templates
  - backend
milestone: m-3
dependencies:
  - task-0019
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create a template engine trait/interface and implement Handlebars and Tera template engines, with support for raw (non-templated) manifests and template engine selection logic.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Template engine trait/interface created
- [ ] #2 Handlebars template engine implementation
- [ ] #3 Tera template engine implementation
- [ ] #4 Support for raw (non-templated) manifests
- [ ] #5 Template engine selection logic implemented
- [ ] #6 Unit tests verify template engine abstraction
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
- Create template engine trait in domain layer
- Implement Handlebars template engine
- Implement Tera template engine
- Add raw manifest support
- Implement engine selection logic
- Add unit tests
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
(To be filled during implementation)
<!-- SECTION:NOTES:END -->
