Verify that task $ARGUMENTS is complete and all acceptance criteria are met.

## Instructions

1. **Read the task file** at `.claude/tasks/$ARGUMENTS.md`
2. **Check each acceptance criterion** individually
3. **Run verification commands** from the task
4. **Report status** for each criterion

## Output Format

```
Task: $ARGUMENTS
Status: PASS | FAIL | PARTIAL

Acceptance Criteria:
- [x] Criterion 1 - verified by ...
- [ ] Criterion 2 - FAILED: reason
- [x] Criterion 3 - verified by ...

Verification Commands:
$ cargo check
  → PASS
$ cargo test
  → PASS (N tests)

Issues:
- None | List of issues
```
