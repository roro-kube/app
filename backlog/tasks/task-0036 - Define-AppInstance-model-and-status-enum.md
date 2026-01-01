---
id: task-0036
title: Define AppInstance model and status enum
status: To Do
assignee: []
created_date: '2026-01-01 16:38'
updated_date: '2026-01-01 16:42'
labels:
  - domain
  - instances
  - backend
milestone: m-4
dependencies:
  - task-0022
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Define AppInstance struct with all required fields and implement InstanceStatus enum (NotDeployed, Deploying, Running, Failed, Updating) with instance ID generation support.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 AppInstance struct defined with all required fields
- [ ] #2 InstanceStatus enum implemented (NotDeployed, Deploying, Running, Failed, Updating)
- [ ] #3 Instance ID generation implemented (UUID or user-defined)
- [ ] #4 Unit tests verify AppInstance model
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
- Create AppInstance struct in domain layer
- Implement InstanceStatus enum
- Add instance ID generation logic
- Add unit tests
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
(To be filled during implementation)
<!-- SECTION:NOTES:END -->
