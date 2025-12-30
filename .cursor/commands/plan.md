# Start Task Work

Implement a backlog task with tests until all acceptance criteria pass.

## Step 1: Load Task

```bash
mise exec -- backlog task TASK_ID --plain
```

Read the description and all acceptance criteria.

## Step 2: Research & Plan

### Research Phase
- Review existing code patterns in relevant files
- Check dependencies and imports needed
- Identify similar implementations to reference
- Review external documentation if needed

### Create Implementation Plan
Output a structured plan:

```
## Implementation Plan

### Files to Modify
- path/to/file.rs - What changes and why
- path/to/test.rs - What tests to add

### Steps
1. Specific change with rationale
2. Specific change with rationale
3. Add test for X covering Y

### Tests Required
- Unit test: what it tests and why
- Integration test: what it tests and why
- Edge cases: specific scenarios to cover

### Verification
- AC 1: How to verify it passes
- AC 2: How to verify it passes
```

Wait for approval before proceeding.

## Step 3: Implement Loop

Repeat until all acceptance criteria pass:

1. Make one change
2. Write tests for that change
3. Run `mise run build`
   - If fails: fix errors, go to step 3
4. Run `mise run test`
   - If fails: fix failing test, go to step 3
5. Verify which acceptance criteria now pass

After each iteration, state:
- What you just did
- Which AC(s) now pass
- What's next (or "All AC pass, ready to complete")

## Step 4: Mark Complete

When all acceptance criteria pass:

```bash
mise exec -- backlog task edit TASK_ID -s "Done"
```

## Rules

- Do proper research before coding
- Every code change needs a test
- Never proceed with failing builds or tests
- State progress after each iteration
- Don't mark done until all AC pass