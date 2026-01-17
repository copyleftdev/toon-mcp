# Task Queue

Execute tasks in order. Each task is self-contained.

## Status

| # | Task | Status | Depends On |
|---|------|--------|------------|
| 001 | [scaffold](001-scaffold.md) | pending | — |
| 002 | [bootstrap](002-bootstrap.md) | pending | 001 |
| 003 | [encode](003-encode.md) | pending | 002 |
| 004 | [decode](004-decode.md) | pending | 002 |
| 005 | [validate](005-validate.md) | pending | 004 |
| 006 | [stats](006-stats.md) | pending | 003 |
| 007 | [testing](007-testing.md) | pending | 003,004,005,006 |
| 008 | [docs](008-docs.md) | pending | 007 |

## Execution

Use Claude Code slash commands:

```
/project:implement-task 001-scaffold
/project:verify-task 001-scaffold
/project:run-all-tasks
```

Or manually:

```
Read .claude/tasks/001-scaffold.md and implement it.
```

## Dependency Graph

```
001 ─► 002 ─┬─► 003 ─► 006 ─┐
            │               │
            └─► 004 ─► 005 ─┼─► 007 ─► 008
```
