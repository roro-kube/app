---
id: task-0033
title: Implement template context builder
status: To Do
assignee: []
created_date: '2026-01-01 16:37'
updated_date: '2026-01-01 16:42'
labels:
  - templates
  - backend
milestone: m-3
dependencies:
  - task-0032
  - task-0029
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement template context builder that resolves variables from static config, environment variables, CRD values via JSONPath, and computed values (namespace, instance_id).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Variable resolution pipeline implemented
- [ ] #2 Static variables from config resolved
- [ ] #3 Environment variables from system resolved
- [ ] #4 CRD values via JSONPath resolved
- [ ] #5 Computed values (namespace, instance_id) resolved
- [ ] #6 Variable substitution validation implemented
- [ ] #7 Unit tests verify context building
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
- Create template context builder module
- Implement variable resolution pipeline
- Add static variable resolution
- Add environment variable resolution
- Add CRD value resolution
- Add computed value resolution
- Add validation and unit tests
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
(To be filled during implementation)
<!-- SECTION:NOTES:END -->
