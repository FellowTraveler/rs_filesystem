rs_filesystem:  MCP Rust Filesystem tools
=============================

rs_filesystem is a simple set of filesystem tools that can be used in Claude desktop or any other MCP client.


# CLI options

* `--mcp`: Enable MCP server
* `--resources`: display resources
* `--prompts`: display prompts
* `--tools`: display tools

# How to use MCP CLI server in Claude Desktop?

1. Edit `claude_desktop_config.json`: Claude Desktop -> `Settings` -> `Developer` -> `Edit Config` 
2. Add the following configuration to the `servers` section:

```json
{
   "mcpServers": {
      "rs_filesystem": {
         "command": "/path/to/rs_filesystem",
         "args": [
            "--mcp"
         ],
         "env": {
            "MCP_RS_FILESYSTEM_ALLOWED_DIRECTORIES": "/path/number/one:/path/number/two"
         }
      }
   }
}
```

Make sure you use the actual path to the rs_filesystem binary.
Make sure the `MCP_RS_FILESYSTEM_ALLOWED_DIRECTORIES` env variable is set to a colon-separated list of allowed directories.
The tools will only work inside those directories.

If you want to check MCP log, please use `tail -n 20 -f ~/Library/Logs/Claude/rs_filesystem.logs.jsonl`.


# References

* MCP Specification: https://spec.modelcontextprotocol.io/
* Model Context Protocol (MCP): https://modelcontextprotocol.io/introduction
* rpc-router: json-rpc routing library - https://github.com/jeremychone/rust-rpc-router/
* Zed context_server: https://github.com/zed-industries/zed/tree/main/crates/context_server
