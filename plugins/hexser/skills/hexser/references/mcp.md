# hexser MCP server

hexser ships an MCP (Model Context Protocol) server, `hex-mcp-server`, that exposes
your project's **architecture graph** plus its **AI context / agent pack** to an MCP
client (Claude Code, Claude Desktop, or any agent) over **stdio** using
line-delimited **JSON-RPC 2.0**. An agent can list and read those resources to learn
your hexagonal architecture (layers, components, relationships, constraints,
suggestions) without you pasting anything in.

Setting this up by hand is what the `/hexser:mcp-setup` command automates — this doc
is the manual reference behind it.

## Feature flags & the binary

The server is gated behind the `mcp` feature, which **implies `ai` + `serde` +
`serde_json`** (so enabling `mcp` alone is enough):

```toml
# workspace/path dependency
hexser = { version = "0.6", features = ["mcp"] }
```

- Binary: `hex-mcp-server` (declared `[[bin]] required-features = ["mcp"]`, source
  `src/bin/hex_mcp_server.rs`). It is essentially:

```rust
// requires feature = "mcp"
fn main() -> hexser::HexResult<()> {
    // McpStdioServer adapter — hexser::adapters::mcp_stdio (mcp feature)
    hexser::adapters::mcp_stdio::McpStdioServer::new().run()
}
```

You normally do **not** write this yourself — you run the shipped binary and point an
MCP client at it via `.mcp.json`.

## Running the server — two scenarios

### Scenario A — hexser is a workspace or path member

If your project depends on hexser as a **workspace member** or a **path** dependency,
run the binary through cargo from your project root:

```sh
cargo run -p hexser --features mcp --bin hex-mcp-server
```

This speaks stdio JSON-RPC 2.0. `-p hexser` selects the hexser package inside your
workspace; `--features mcp` turns on the server; `--bin hex-mcp-server` picks the
binary.

`.mcp.json` (project-local MCP config for Claude Code):

```json
{
  "mcpServers": {
    "hexser": {
      "command": "cargo",
      "args": ["run", "-p", "hexser", "--features", "mcp", "--bin", "hex-mcp-server"],
      "cwd": "/absolute/path/to/your/project"
    }
  }
}
```

`cwd` must be the project root (where the workspace `Cargo.toml` lives) so cargo can
resolve `-p hexser` and so `hexser/refresh` can rebuild the right package.

### Scenario B — hexser is a normal registry (crates.io) dependency

If hexser is just a regular `crates.io` dependency, `cargo run -p hexser …` does
**NOT** work: `-p hexser` only resolves for packages that are workspace/path members,
not for a registry dep pulled into `~/.cargo/registry`. Instead, install the binary
once, then invoke the installed `hex-mcp-server` directly:

```sh
cargo install hexser --features mcp
# installs `hex-mcp-server` into ~/.cargo/bin (put that dir on PATH)
```

`.mcp.json`:

```json
{
  "mcpServers": {
    "hexser": {
      "command": "hex-mcp-server",
      "args": [],
      "cwd": "/absolute/path/to/your/project"
    }
  }
}
```

Keep `cwd` at the project root so `hexser/refresh` runs its `cargo build` against your
project. If `~/.cargo/bin` is not on PATH, use the absolute path
(`/Users/you/.cargo/bin/hex-mcp-server`) as `command`.

> Claude Desktop uses the same three keys (`command`, `args`, `cwd`) under its own
> `mcpServers` config; the Scenario A entry above matches the documented Claude Desktop
> setup.

## Resources

The server exposes two resources per project (namespaced by project name):

| URI | Content |
| --- | --- |
| `hexser://{project}/context` | `AIContext` JSON (architecture, components, relationships, constraints, suggestions) |
| `hexser://{project}/pack` | `AgentPack` JSON (`ai_context` + guidelines + docs, versioned) |

Legacy **flat** forms are still accepted and resolve to project `"hexser"`:

- `hexser://context` → context for project `hexser`
- `hexser://pack` → pack for project `hexser`

The default `McpStdioServer::new()` serves project `hexser`; use
`McpStdioServer::with_registry(ProjectRegistry)` to serve multiple named projects
(`ProjectConfig` / `ProjectRegistry` domain types).

See `graph-and-ai.md` for `AIContext` / `AgentPack` structure and the standalone
`hex-ai-export` / `hex-ai-pack` binaries.

## Methods (JSON-RPC 2.0, line-delimited over stdio)

| Method | Purpose |
| --- | --- |
| `initialize` | Handshake; returns server capabilities |
| `resources/list` | List available resources (context, pack) |
| `resources/read` | Read one resource by `uri` |
| `hexser/refresh` | Rebuild the project (see caveat below) |

Example exchange (one JSON object per line):

```json
{"jsonrpc":"2.0","id":1,"method":"initialize"}
{"jsonrpc":"2.0","id":2,"method":"resources/list"}
{"jsonrpc":"2.0","id":3,"method":"resources/read","params":{"uri":"hexser://hexser/context"}}
{"jsonrpc":"2.0","id":4,"method":"hexser/refresh"}
```

Notifications (requests with no `id`) intentionally get **no response**.

The MCP wire types live under `domain/mcp/`: `JsonRpcRequest`, `JsonRpcResponse`,
`JsonRpcError`, `ServerCapabilities`, `ResourceCapability`, `Resource`,
`ResourceContent`, `ResourceList`, `InitializeRequest`, `InitializeResult`,
`ProjectConfig`, `ProjectRegistry`, `RefreshRequest`, `RefreshResult`. The port
contract is `McpServer` (`ports/mcp_server.rs`): `initialize`, `list_resources`,
`read_resource(&str)`, `refresh_project`, `handle_request`.

## JSON-RPC error codes

| Code | Meaning |
| --- | --- |
| `-32700` | Parse error — malformed JSON |
| `-32600` | Invalid request — bad request shape |
| `-32601` | Method not found — unknown method |

(Notifications produce no response at all, including on error.)

## Refresh and the link-time inventory caveat

`hexser/refresh` runs `cargo build -p {project} --features macros` (300s timeout,
stderr captured and capped at 64 KiB) and returns a `RefreshResult`. It **rebuilds**
the project so the on-disk artifacts are current.

**Important:** hexser's component inventory is collected at **link time** (via
`inventory::submit!` in the derive macros). The running server binary already linked
its inventory when it started, so:

- `hexser/refresh` rebuilds but **cannot hot-add** newly written components to the
  live graph.
- After you add or remove `#[derive(HexDomain/HexPort/HexAdapter/…)]` components, you
  must **RESTART** `hex-mcp-server` for them to appear in `resources/read`.

In Claude Code, restarting the MCP server (or the session) re-links a fresh inventory
and picks up your new components.

## Automation

`/hexser:mcp-setup` detects which scenario applies (workspace/path vs. registry dep),
writes the correct `.mcp.json` entry, and installs the binary when needed — prefer it
over doing the above by hand.
