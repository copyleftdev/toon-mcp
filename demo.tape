# VHS Demo for toon-mcp
Output demo.gif

Set FontSize 14
Set Width 900
Set Height 500
Set Theme "Catppuccin Mocha"

Type "# TOON MCP Server Demo"
Enter
Sleep 500ms

Type "# 1. Encode JSON to TOON"
Enter
Sleep 300ms

Type `echo '{"users":[{"id":1,"name":"Alice"},{"id":2,"name":"Bob"}]}' | jq .`
Enter
Sleep 1s

Type "# Converted to TOON format:"
Enter
Sleep 300ms

Type `printf '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"demo","version":"1.0"}}}\n{"jsonrpc":"2.0","method":"notifications/initialized"}\n{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"toon_encode","arguments":{"json":{"users":[{"id":1,"name":"Alice"},{"id":2,"name":"Bob"}]}}}}\n' | ./target/release/toon-mcp 2>/dev/null | tail -1 | jq -r '.result.content[0].text'`
Enter
Sleep 2s

Type "# 2. Check token savings with toon_stats"
Enter
Sleep 300ms

Type `printf '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"demo","version":"1.0"}}}\n{"jsonrpc":"2.0","method":"notifications/initialized"}\n{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"toon_stats","arguments":{"json":{"users":[{"id":1,"name":"Alice"},{"id":2,"name":"Bob"}]}}}}\n' | ./target/release/toon-mcp 2>/dev/null | tail -1 | jq '.result.structuredContent'`
Enter
Sleep 2s

Type "# ~37% byte savings, ~59% token savings!"
Enter
Sleep 1.5s
