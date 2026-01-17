Implement task $ARGUMENTS from the `.claude/tasks/` directory.

## Instructions

1. **Read the task file** at `.claude/tasks/$ARGUMENTS.md`
2. **Read CLAUDE.md** for project context and conventions
3. **Think hard** about the implementation approach before coding
4. **Implement** following the technical specification exactly
5. **Run verification** commands from the task
6. **Confirm** all acceptance criteria are met

## Rules

- Follow the code patterns in CLAUDE.md
- Run `cargo check` after creating/modifying Rust files
- Run `cargo test` to verify nothing is broken
- If a task depends on incomplete prior tasks, stop and report

## On Completion

When finished, output:
- Summary of files created/modified
- Test results
- Any issues encountered
