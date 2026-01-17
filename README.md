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

### Direct Testing

```bash
# Initialize and call a tool
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}' | ./target/release/toon-mcp
```

## Tools

### toon_encode

Convert JSON to TOON format.

```json
{"json": {"users": [{"id": 1, "name": "Alice"}]}}
```

Options:
- `delimiter` - "comma" (default), "tab", or "pipe"
- `indent` - Spaces for indentation (0-8, default: 2)
- `fold_keys` - Enable v1.5 key folding
- `flatten_depth` - Max depth for key folding

### toon_decode

Convert TOON back to JSON.

```json
{"toon": "name: Alice\nage: 30"}
```

Options:
- `strict` - Strict validation (default: true)
- `coerce_types` - Type coercion (default: true)
- `expand_paths` - Path expansion (default: false)
- `output_format` - "json" or "json_pretty" (default: "json")

### toon_validate

Check TOON syntax without full decoding.

```json
{"toon": "x: 1"}
```

Returns: `{"valid": true}` or `{"valid": false, "error": {...}}`

### toon_stats

Compare token and byte counts between JSON and TOON.

```json
{"json": {"data": [1, 2, 3]}}
```

Returns savings percentages for bytes and tokens.

### toon_ping

Verify server connectivity.

Returns: `"pong - toon-mcp server is running"`

## Development

```bash
cargo check          # Type check
cargo build          # Debug build
cargo build --release # Release build
cargo test           # Run all tests
cargo clippy         # Lint
cargo fmt            # Format
```

## License

MIT
