# Task 008: Documentation

**Status**: pending  
**Depends On**: 007-testing  

---

## Objective

Create README and ensure code is documented.

---

## Acceptance Criteria

- [ ] README.md with installation and usage
- [ ] Tool descriptions are clear
- [ ] Example commands provided

---

## Files to Create

### README.md

```markdown
# TOON MCP Server

MCP server exposing TOON format for LLM cost optimization. **18-40% token savings** over JSON.

## Installation

```bash
cargo build --release
```

## Usage

### Claude Desktop

Add to `claude_desktop_config.json`:
```json
{
  "mcpServers": {
    "toon": {
      "command": "/path/to/toon-mcp"
    }
  }
}
```

## Tools

### toon_encode

Convert JSON to TOON.

```json
{"json": {"users": [{"id": 1, "name": "Alice"}]}}
```

Options: `delimiter`, `indent`, `fold_keys`, `flatten_depth`

### toon_decode

Convert TOON to JSON.

```json
{"toon": "name: Alice\nage: 30"}
```

Options: `strict`, `coerce_types`, `expand_paths`, `output_format`

### toon_validate

Check TOON syntax.

```json
{"toon": "x: 1"}
```

Returns: `{"valid": true}` or `{"valid": false, "error": {...}}`

### toon_stats

Compare token/byte counts.

```json
{"json": {"data": [1, 2, 3]}}
```

Returns savings percentages.

## License

MIT
```

---

## Verification

- README is readable and accurate
- `cargo doc --open` generates docs

---

## Completion

All tasks complete. Server is ready for use.
