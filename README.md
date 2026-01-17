# TOON MCP Server

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org/)

MCP server exposing TOON format for LLM cost optimization. **18-40% token savings** over JSON.

**Repository:** [github.com/copyleftdev/toon-mcp](https://github.com/copyleftdev/toon-mcp)

## Demo

![TOON MCP Server Demo](demo.gif)

## Installation

```bash
cargo build --release
```

## Usage

### Claude Desktop

Add to `~/Library/Application Support/Claude/claude_desktop_config.json` (macOS) or `%APPDATA%\Claude\claude_desktop_config.json` (Windows):

```json
{
  "mcpServers": {
    "toon": {
      "command": "/path/to/toon-mcp"
    }
  }
}
```

### Claude Code CLI

Add to `~/.claude/settings.json`:

```json
{
  "mcpServers": {
    "toon": {
      "command": "/path/to/toon-mcp"
    }
  }
}
```

Or for project-specific configuration, create `.mcp.json` in your project root:

```json
{
  "mcpServers": {
    "toon": {
      "command": "/path/to/toon-mcp"
    }
  }
}
```

### Cursor IDE

Add to `.cursor/mcp.json` in your project:

```json
{
  "mcpServers": {
    "toon": {
      "command": "/path/to/toon-mcp"
    }
  }
}
```

### Generic MCP Client

The server uses stdio transport. Connect by spawning the process and communicating via stdin/stdout:

```bash
# Start server and send initialize request
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"my-client","version":"1.0"}}}' | ./toon-mcp
```

Example in Node.js:

```javascript
const { spawn } = require('child_process');
const server = spawn('./toon-mcp');

server.stdout.on('data', (data) => {
  console.log('Response:', JSON.parse(data));
});

server.stdin.write(JSON.stringify({
  jsonrpc: "2.0",
  id: 1,
  method: "initialize",
  params: {
    protocolVersion: "2024-11-05",
    capabilities: {},
    clientInfo: { name: "my-client", version: "1.0" }
  }
}) + '\n');
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

## Contributing

Contributions are welcome! Please open an issue or submit a PR at [github.com/copyleftdev/toon-mcp](https://github.com/copyleftdev/toon-mcp).

## License

MIT - See [LICENSE](LICENSE) for details.
