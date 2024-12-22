MCP Rust CLI server template
=============================

Model Context Protocol (MCP) is an open protocol that enables seamless integration between LLM applications
and external data sources and tools. Whether you’re building an AI-powered IDE, enhancing a chat interface,
or creating custom AI workflows, MCP provides a standardized way to connect LLMs with the context they need.

mcp-rs-template is a simple application template that demonstrates how to implement MCP CLI server in Rust.

# How to use template?

1. Clone the repository
2. Modify project information in `Cargo.toml` and `src/mcp/mod.rs`
3. Modify server handlers:
    - `src/mcp/prompts.rs`: prompts handlers
    - `src/mcp/resources.rs`: resources handlers
    - `src/mcp/tools.rs`: tools handlers
4. Modify `src/mcp/templates/*.json` if you prefer to use json files for prompts, resources, and tools

mcp-rs-template is based on [rust-rpc-router](https://github.com/jeremychone/rust-rpc-router), a JSON-RPC routing
library for Rust.

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
      "current-time": {
         "command": "rs_filesystem",
         "args": [
            "--mcp"
         ],
         "env": {
            "API_KEY": "xxxx"
         }
      }
   }
}
```

If you want to check MCP log, please use `tail -n 20 -f ~/Library/Logs/Claude/mcp*.log`.


# References

* MCP Specification: https://spec.modelcontextprotocol.io/
* Model Context Protocol (MCP): https://modelcontextprotocol.io/introduction
* rpc-router: json-rpc routing library - https://github.com/jeremychone/rust-rpc-router/
* Zed context_server: https://github.com/zed-industries/zed/tree/main/crates/context_server
