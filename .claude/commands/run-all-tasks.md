Execute all tasks in dependency order to build the complete toon-mcp server.

## Task Order

Execute these in sequence, stopping if any fails:

1. `001-scaffold` — Project structure and Cargo.toml
2. `002-bootstrap` — MCP server with stdio transport
3. `003-encode` — toon_encode tool
4. `004-decode` — toon_decode tool  
5. `005-validate` — toon_validate tool
6. `006-stats` — toon_stats tool
7. `007-testing` — Test suite and fixtures
8. `008-docs` — README and documentation

## For Each Task

1. Read `.claude/tasks/XXX-*.md`
2. Implement following the spec
3. Run `cargo check` and `cargo test`
4. Verify acceptance criteria
5. If PASS, continue to next task
6. If FAIL, stop and report

## On Completion

Report final status:
- All tasks completed successfully
- Or: stopped at task N with error X
