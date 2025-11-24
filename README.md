# rustmcp-template
template for building mcp server in rust


- First compile the mcp server using the following command
```bash
cargo build --release --bin mcp
```

- Run the compiled mcp server using the following
```bash
./target/release/mcp
```

- add the following to your mcp client (Claude Desktop)
```json
{
    "mcpServers": {
        "example-mcp": {
            "command": "path/to/mcp/target/release/mcp",
            "args": []
        }
    }
}
```

