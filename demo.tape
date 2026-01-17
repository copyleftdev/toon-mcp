# VHS Demo for toon-mcp
Output demo.gif

Set FontSize 16
Set Width 800
Set Height 600
Set Theme "Catppuccin Mocha"
Set Padding 20
Set Shell "bash"

Type "cd /home/ops/Project/toonz/toon-mcp"
Enter
Sleep 500ms

Type "# TOON MCP Server - Token Optimization Demo"
Enter
Enter
Sleep 1s

Type "cat sample.json"
Enter
Sleep 1.5s

Type "# Convert to TOON format..."
Enter
Sleep 500ms

Type "/bin/bash ./toon-demo encode sample.json"
Enter
Sleep 2.5s

Type "# Compare the savings:"
Enter
Sleep 500ms

Type "/bin/bash ./toon-demo stats sample.json"
Enter
Sleep 3s

Type "# 46% smaller, 66% fewer tokens!"
Enter
Sleep 2s
