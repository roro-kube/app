# Start Task Work

Begin implementation of a backlog task by creating a comprehensive implementation plan. This command handles task planning, breakdown, and status updates.

## Workflow

### Step 1: Retrieve Task Details

```bash
mise exec -- backlog task TASK_ID --plain
```

Read and understand:
- Task description (the why)
- All acceptance criteria (the what)
- Current status and dependencies
- Existing labels and priority

### Step 2: Analyze Complexity

Determine if the task should be:

**A. Implemented Directly**
- Task is atomic (single PR)
- Scope is clear and bounded
- No major sub-components
- Can be completed in one go

**B. Broken Down**
- Task is too large for one PR
- Contains multiple independent features
- Has distinct phases or components
- Would benefit from incremental delivery

### Step 3A: Create Implementation Plan (Direct)

If implementing directly, create a **comprehensive** plan between the section markers:

```markdown
## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->

### Research
- [ ] Review existing authentication patterns in `auth/`
- [ ] Check database schema for user table structure
- [ ] Review JWT library documentation

### Implementation Steps
1. **Create user model** (`models/user.rs`)
   - Add fields: id, email, password_hash, created_at
   - Implement validation methods
   
2. **Add authentication service** (`services/auth.rs`)
   - Implement password hashing (bcrypt)
   - Create JWT token generation
   - Add login verification logic

3. **Create login endpoint** (`routes/auth.rs`)
   - POST /api/login handler
   - Request validation
   - Error handling

### Testing Strategy
- [ ] Unit tests for password hashing
- [ ] Unit tests for JWT generation
- [ ] Integration test for login flow
- [ ] Test invalid credentials handling
- [ ] Test edge cases (empty fields, special chars)

### Verification Steps
- [ ] AC 1: User can log in with valid credentials
  - Manual test: POST to /api/login with valid user
  - Expected: 200 response with JWT token
- [ ] AC 2: Invalid credentials produce clear error
  - Manual test: POST with wrong password
  - Expected: 401 with "Invalid credentials" message
- [ ] All tests pass with >80% coverage
- [ ] Error messages are user-friendly

<!-- SECTION:PLAN:END -->
```

**Plan Requirements**:
1. **Research Phase**
   - Specific files/components to examine
   - External docs or APIs to review
   - Checkboxes for each item

2. **Implementation Steps**
   - Numbered, ordered sequence
   - File/function names in bold with paths
   - Specific changes for each step
   - Note dependencies between steps

3. **Testing Strategy**
   - Unit tests needed
   - Integration tests needed
   - Edge cases to cover
   - Checkboxes for tracking

4. **Verification Steps**
   - Map each AC to verification method
   - Include test commands or manual steps
   - Expected outcomes
   - Checkboxes for tracking

### Step 3B: Break Down Task (Subtasks)

If breaking down, create subtasks that are:
- **Atomic**: Each is one PR
- **Independent**: Can be implemented separately
- **Ordered**: Earlier tasks don't depend on later ones
- **Complete**: Together they fulfill all parent ACs

**Present breakdown analysis**:
```markdown
## Task Breakdown Recommendation

This task should be broken into X subtasks:

### Subtask 1: [Title]
**Labels**: label1, label2
**Priority**: high
**Dependencies**: None
**Scope**: 
Clear 2-3 sentence description of what this accomplishes

**Acceptance Criteria**:
- [ ] Specific outcome 1
- [ ] Specific outcome 2

**Why separate**: Reasoning for splitting this out

---

### Subtask 2: [Title]
**Labels**: label1, label2
**Priority**: high
**Dependencies**: task-SUBTASK1_ID
**Scope**: 
Clear 2-3 sentence description

**Acceptance Criteria**:
- [ ] Specific outcome 1
- [ ] Specific outcome 2

**Why separate**: Reasoning for splitting this out

---

### Subtask 3: [Title]
**Labels**: label1, label2
**Priority**: medium
**Dependencies**: task-SUBTASK1_ID, task-SUBTASK2_ID
**Scope**: 
Clear 2-3 sentence description

**Acceptance Criteria**:
- [ ] Specific outcome 1
- [ ] Specific outcome 2

**Why separate**: Reasoning for splitting this out
```

Then for each subtask:
```bash
mise exec -- backlog task create "Subtask title" -l label1,label2 --dep task-XX --priority high
```

And populate each subtask's Markdown file with:
- Full description in `<!-- SECTION:DESCRIPTION:BEGIN -->` section
- All ACs in `<!-- AC:BEGIN -->` section

### Step 4: Get User Approval

Present the plan or breakdown and **wait for explicit approval**.

**Say**: "I've created the implementation plan above. Please review and approve before I update the task."

### Step 5: Update Task

**If Direct Implementation**:

1. **Edit task Markdown file**:
   - Replace content between `<!-- SECTION:PLAN:BEGIN -->` and `<!-- SECTION:PLAN:END -->`
   - Keep section markers intact
   - Add complete implementation plan

2. **Update status via CLI**:
   ```bash
   mise exec -- backlog task edit TASK_ID -s "In Progress" -a @yourself
   ```

**If Breakdown**:

1. **Create each subtask**:
   ```bash
   mise exec -- backlog task create "Subtask title" -l labels --dep task-XX --priority high
   ```

2. **Edit each subtask Markdown file**:
   - Add description in `<!-- SECTION:DESCRIPTION:BEGIN -->` section
   - Add ACs in `<!-- AC:BEGIN -->` section

3. **Update parent task Markdown**:
   - Add breakdown explanation in `<!-- SECTION:PLAN:BEGIN -->` section
   - Reference all created subtask IDs

4. **Keep parent in current status** (or set to "Blocked" if appropriate)

## Section Markers (CRITICAL)

All content must be placed between section markers:

```markdown
## Description
<!-- SECTION:DESCRIPTION:BEGIN -->
Content here
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] Criterion 1
- [ ] Criterion 2
<!-- AC:END -->

## Implementation Plan
<!-- SECTION:PLAN:BEGIN -->
(Plan content goes here)
<!-- SECTION:PLAN:END -->

## Implementation Notes
<!-- SECTION:NOTES:BEGIN -->
(Notes added after implementation)
<!-- SECTION:NOTES:END -->
```

**Never remove or modify section markers.**

## Requirements Checklist

### Implementation Plan Must Have:
- [ ] Content between `<!-- SECTION:PLAN:BEGIN -->` and `<!-- SECTION:PLAN:END -->`
- [ ] Research phase with checkboxes
- [ ] Implementation steps with file paths
- [ ] Testing strategy with checkboxes
- [ ] Verification steps mapped to ACs
- [ ] All items specific and actionable

### Subtask Breakdown Must Have:
- [ ] Each subtask is atomic (one PR)
- [ ] Clear scope and reasoning for each
- [ ] Specific ACs for each subtask
- [ ] Proper dependency ordering (only reference lower IDs)
- [ ] All parent ACs covered by subtasks
- [ ] Each subtask has description in DESCRIPTION section
- [ ] Each subtask has ACs in AC section

## Output Format

### For Direct Implementation:
1. Show task ID and title
2. Present comprehensive implementation plan with section markers
3. Wait for approval
4. Show CLI command: `mise exec -- backlog task edit TASK_ID -s "In Progress" -a @yourself`
5. Confirm you'll update the task Markdown file

### For Breakdown:
1. Show task ID and title
2. Present breakdown with all subtask details
3. Wait for approval
4. Show CLI commands to create each subtask
5. Confirm you'll populate each subtask Markdown file
6. Show plan to update parent task

## Anti-Patterns

❌ Content outside section markers
❌ Removing or modifying section markers
❌ Vague plan steps ("Implement auth")
❌ Missing testing strategy
❌ No research phase
❌ Skipping user approval
❌ Not mapping ACs to verification steps
❌ Subtasks depending on future tasks
❌ Missing file paths in implementation steps