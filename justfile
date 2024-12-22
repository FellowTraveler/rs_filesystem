# Remove or modify the shell line since Macs use bash/zsh by default
# set shell := ["pwsh","-NoProfile","-NoLogo","-Command"]

ping:
  echo '{ "jsonrpc": "2.0", "id": 1, "method": "ping" }' | ./target/debug/rs_filesystem --mcp

prompts-list:
  echo '{ "jsonrpc": "2.0", "id": 1, "method": "prompts/list" }' | ./target/debug/rs_filesystem --mcp

prompt-get:
  echo '{ "jsonrpc": "2.0", "id": 1, "method": "prompts/get", "params": {"name":"current_time","arguments": {"city": "hangzhou"} } }' | ./target/debug/rs_filesystem --mcp

tools-list:
  echo '{ "jsonrpc": "2.0", "id": 1, "method": "tools/list" }' | ./target/debug/rs_filesystem --mcp

resources-list:
  echo '{ "jsonrpc": "2.0", "id": 1, "method": "resources/list" }' | ./target/debug/rs_filesystem --mcp

current-time:
  echo '{ "jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": { "name": "get_current_time_in_city", "arguments": {"city":"Hangzhou" } } }' | ./target/debug/rs_filesystem --mcp
