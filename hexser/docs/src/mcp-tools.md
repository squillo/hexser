# MCP Tools

Hexser provides Model Context Protocol (MCP) server capabilities for exposing architecture data to AI agents. This enables AI assistants to query your project's hexagonal architecture, agent packs, and other metadata through a standardized JSON-RPC 2.0 interface.

## What is MCP?

Model Context Protocol (MCP) is a protocol that allows AI agents to connect to servers and access contextual information. In hexser's case, the MCP server exposes:

- Architecture context (HexGraph data)
- Agent pack information
- Project metadata
- Component relationships and dependencies

This allows AI coding assistants to understand your hexagonal architecture and provide better, architecture-aware suggestions.

## Requirements

MCP tools require both the `ai` and `mcp` features to be enabled in your `Cargo.toml`:

```toml
[dependencies]
hexser = { version = "0.x", features = ["ai", "mcp"] }
```

## The McpServer Trait

The `McpServer` trait defines the core operations that MCP server adapters must implement:

```rust
pub trait McpServer {
    fn initialize(
        &self,
        request: crate::domain::mcp::InitializeRequest,
    ) -> crate::HexResult<crate::domain::mcp::InitializeResult>;

    fn list_resources(&self) -> crate::HexResult<crate::domain::mcp::ResourceList>;

    fn read_resource(&self, uri: &str) -> crate::HexResult<crate::domain::mcp::ResourceContent>;

    fn handle_request(
        &self,
        request: crate::domain::mcp::JsonRpcRequest,
    ) -> crate::domain::mcp::JsonRpcResponse;
}
```

### Methods

#### initialize

Handles the MCP initialization handshake. This is the first method called by clients and performs capability negotiation.

**Arguments:**
- `request`: Client's initialization request containing protocol version and capabilities

**Returns:**
- `InitializeResult` containing server information and supported capabilities

#### list_resources

Returns all resources that clients can query. For hexser, this typically includes:
- `hexser://context` - Architecture context (AIContext JSON)
- `hexser://pack` - Agent pack information (AgentPack JSON)

**Returns:**
- `ResourceList` containing available resource URIs and descriptions

#### read_resource

Retrieves the content of a specific resource by URI.

**Arguments:**
- `uri`: Resource URI to read (e.g., `hexser://context`)

**Returns:**
- `ResourceContent` containing the requested data

#### handle_request

Main entry point for processing JSON-RPC requests. Routes requests to appropriate handlers based on method name.

**Arguments:**
- `request`: JSON-RPC 2.0 request

**Returns:**
- JSON-RPC 2.0 response (success or error)

## The hex_mcp_server Binary

Hexser includes a pre-built MCP server binary that you can run to expose your architecture data:

### Usage

```bash
# Run the MCP server
cargo run --bin hex_mcp_server --features ai,mcp

# Or if installed:
hex_mcp_server
```

The server communicates over stdio using line-delimited JSON-RPC 2.0 messages. AI agents connect to this server and send requests via stdin, receiving responses on stdout.

### Protocol Flow

1. **Client connects** to the server process
2. **Initialize**: Client sends `initialize` method with protocol version
3. **Server responds** with capabilities and server info
4. **Client requests** resources via `list_resources` or `read_resource`
5. **Server responds** with architecture data
6. **Connection closes** when client disconnects

### Example JSON-RPC Requests

#### Initialize Request

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {
    "protocolVersion": "2024-11-05",
    "capabilities": {},
    "clientInfo": {
      "name": "example-client",
      "version": "1.0.0"
    }
  }
}
```

#### List Resources Request

```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "resources/list",
  "params": {}
}
```

#### Read Resource Request

```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "resources/read",
  "params": {
    "uri": "hexser://context"
  }
}
```

## Implementing Custom MCP Adapters

You can create custom MCP server adapters for different transports (HTTP, WebSocket, etc.) by implementing the `McpServer` trait:

```rust
struct CustomMcpServer {
    graph: std::sync::Arc<hexser::domain::HexGraph>,
}

impl hexser::ports::McpServer for CustomMcpServer {
    fn initialize(
        &self,
        request: hexser::domain::mcp::InitializeRequest,
    ) -> hexser::HexResult<hexser::domain::mcp::InitializeResult> {
        std::result::Result::Ok(hexser::domain::mcp::InitializeResult {
            protocol_version: std::string::String::from("2024-11-05"),
            capabilities: hexser::domain::mcp::ServerCapabilities {
                resources: std::option::Option::Some(hexser::domain::mcp::ResourcesCapability {
                    subscribe: false,
                }),
            },
            server_info: hexser::domain::mcp::ServerInfo {
                name: std::string::String::from("custom-hexser-server"),
                version: std::string::String::from("1.0.0"),
            },
        })
    }

    fn list_resources(&self) -> hexser::HexResult<hexser::domain::mcp::ResourceList> {
        let resources = std::vec![
            hexser::domain::mcp::Resource {
                uri: std::string::String::from("hexser://context"),
                name: std::string::String::from("Architecture Context"),
                description: std::option::Option::Some(std::string::String::from(
                    "Complete architecture context for AI agents"
                )),
                mime_type: std::option::Option::Some(std::string::String::from("application/json")),
            },
        ];
        std::result::Result::Ok(hexser::domain::mcp::ResourceList { resources })
    }

    fn read_resource(&self, uri: &str) -> hexser::HexResult<hexser::domain::mcp::ResourceContent> {
        match uri {
            "hexser://context" => {
                let context = self.graph.to_ai_context()?;
                let json = serde_json::to_string_pretty(&context)?;
                std::result::Result::Ok(hexser::domain::mcp::ResourceContent {
                    uri: std::string::String::from(uri),
                    mime_type: std::option::Option::Some(std::string::String::from("application/json")),
                    text: std::option::Option::Some(json),
                })
            }
            _ => std::result::Result::Err(hexser::HexError::not_found(
                std::string::String::from("Resource not found"),
            )),
        }
    }

    fn handle_request(
        &self,
        request: hexser::domain::mcp::JsonRpcRequest,
    ) -> hexser::domain::mcp::JsonRpcResponse {
        // Route to appropriate handler based on request.method
        todo!("Implement request routing")
    }
}
```

## Integration with AI Agents

AI coding assistants can connect to the MCP server to query architecture information:

### Claude Desktop Configuration

Add to your Claude Desktop configuration file (`claude_desktop_config.json`):

```json
{
  "mcpServers": {
    "hexser": {
      "command": "hex_mcp_server",
      "args": [],
      "env": {}
    }
  }
}
```

### Cursor IDE Configuration

Configure Cursor to use the MCP server for architecture-aware suggestions by adding the server to your workspace settings.

### Custom Integration

Any tool that supports MCP can connect to the hexser MCP server. The protocol uses standard JSON-RPC 2.0 over stdio, making it compatible with various AI tools and frameworks.

## Available Resources

### hexser://context

Returns the complete architecture context as JSON, including:
- All registered components (nodes)
- Component layers (Domain, Ports, Adapters, Application, Infrastructure)
- Dependencies between components
- Component metadata and intent

This data allows AI agents to understand your architecture and provide context-aware suggestions.

### hexser://pack

Returns agent pack information including:
- Project conventions
- Architecture patterns
- Coding standards
- Custom rules and guidelines

## Best Practices

1. **Enable features**: Always enable both `ai` and `mcp` features when using MCP tools.

2. **Secure your server**: The MCP server exposes architecture information. In production, ensure proper access controls are in place.

3. **Keep graph updated**: Register components with the HexGraph to ensure AI agents have accurate architecture data.

4. **Document resources**: Add clear descriptions to custom resources so AI agents understand their purpose.

5. **Handle errors gracefully**: Implement robust error handling in custom MCP adapters to provide helpful error messages.

6. **Version your protocol**: Use semantic versioning for custom MCP adapters to maintain compatibility.

## Transports

### Stdio Transport (Default)

The default `hex_mcp_server` binary uses stdio transport:
- Reads JSON-RPC requests from stdin (one per line)
- Writes JSON-RPC responses to stdout (one per line)
- Logs and errors go to stderr

This is the standard transport for MCP and works with most AI tools.

### Custom Transports

You can implement custom transports (HTTP, WebSocket, etc.) by:
1. Implementing the `McpServer` trait
2. Creating an adapter that handles your transport protocol
3. Routing requests to the MCP server implementation

## Troubleshooting

### Server won't start
- Ensure `ai` and `mcp` features are enabled
- Check that all dependencies are properly installed
- Verify the binary is built with the correct features

### AI agent can't connect
- Verify the server is running and accepting connections
- Check stdio configuration in your AI tool
- Ensure JSON-RPC messages are properly formatted

### Resources not found
- Verify your HexGraph is properly populated
- Check resource URIs match expected format
- Ensure `read_resource` implementation handles all listed resources

## See Also

- [Core Concepts](core-concepts.md) - Understanding hexagonal architecture in hexser
- [Architecture](architecture.md) - Detailed architecture documentation
- [Getting Started](getting-started.md) - Setting up your hexser project
