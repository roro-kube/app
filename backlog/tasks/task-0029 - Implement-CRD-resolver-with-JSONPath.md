---
id: task-0029
title: Implement CRD resolver with JSONPath
status: To Do
assignee: []
created_date: '2026-01-01 16:37'
updated_date: '2026-01-01 19:26'
labels:
  - kubernetes
  - crd
  - jsonpath
  - backend
milestone: m-10
dependencies:
  - task-0028
priority: low
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement CRD resolver that fetches Custom Resource Definitions from Kubernetes API and extracts values using JSONPath expressions, with namespace-aware resolution.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 JSONPath-based CRD value extraction implemented
- [ ] #2 CRD resource fetching from Kubernetes API implemented
- [ ] #3 Namespace-aware CRD resolution implemented
- [ ] #4 Support for user-defined CRD structures
- [ ] #5 Error handling for missing/invalid CRDs
- [ ] #6 Unit tests verify CRD resolution
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
- Create CRD resolver module in core layer
- Implement JSONPath value extraction
- Add CRD resource fetching from Kubernetes API
- Implement namespace-aware resolution
- Add error handling and unit tests
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
(To be filled during implementation)
<!-- SECTION:NOTES:END -->
