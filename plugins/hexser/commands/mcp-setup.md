---
description: Install and wire the hexser MCP server for this project
argument-hint: ""
allowed-tools: Read, Write, Edit, Bash(cargo *), Bash(which *)
---

Set up the hexser MCP server for the current project so Claude can introspect its
hexagonal architecture (the `AIContext` and `AgentPack` resources it exposes over
stdio). Follow `skills/hexser/references/mcp.md` for the details behind each step.

The hexser MCP server is the `hex-mcp-server` binary. It ships with the `hexser`
crate and requires the `mcp` feature (which implies `ai` + `serde` + `serde_json`).
It speaks line-delimited JSON-RPC 2.0 over stdio.

Work through these steps. Read before you write, and never run git.

## 1. Locate the hexser dependency and pick a launch mode

Read the project's `Cargo.toml` (and, if present, the workspace-root `Cargo.toml`)
to find how `hexser` is depended on. There are two modes:

- **Workspace / path member** — `hexser` is a workspace member, or a dependency
  declared with `path = "..."` (e.g. a local checkout, or this repo itself). In
  this mode you can build and run its binary directly with `cargo`.
- **Registry dependency** — `hexser` comes from crates.io as a plain version
  requirement (e.g. `hexser = "0.6"`). In this mode `cargo run -p hexser` will not
  resolve, so install the binary once and launch it by name.

If you are unsure, run `cargo metadata --format-version 1` and inspect the source
of the `hexser` package (a `path+file://...` or workspace member => path mode; a
`registry+https://...` source => registry mode).

## 2. Ensure the `mcp` feature is available

- **Workspace / path member:** confirm the server compiles with the feature
  (build, don't run — the server would block reading stdin):
  ```
  cargo build -p hexser --features mcp --bin hex-mcp-server
  ```
  If it builds, the binary and feature are wired. You do not need to add `mcp` to
  the project's own dependency on hexser — the `--features mcp` flag on the launch
  command enables it for the server build.

- **Registry dependency:** install the server binary with the feature enabled:
  ```
  cargo install hexser --features mcp
  ```
  Then verify it is on `PATH`:
  ```
  which hex-mcp-server
  ```
  `cargo install` places it in `~/.cargo/bin`. If `which` finds nothing, make sure
  `~/.cargo/bin` is on the user's `PATH`.

## 3. Write or patch the project `.mcp.json`

Create `.mcp.json` at the project root if it does not exist, otherwise Read it and
add a `hexser` entry under `mcpServers` without disturbing existing servers. Keep it
minimal and valid JSON. Use the absolute path to the project root for `cwd`.

**Workspace / path member** — launch via cargo:
```json
{
  "mcpServers": {
    "hexser": {
      "command": "cargo",
      "args": ["run", "-p", "hexser", "--features", "mcp", "--bin", "hex-mcp-server"],
      "cwd": "/absolute/path/to/project"
    }
  }
}
```

**Registry dependency** — launch the installed binary by name:
```json
{
  "mcpServers": {
    "hexser": {
      "command": "hex-mcp-server",
      "args": [],
      "cwd": "/absolute/path/to/project"
    }
  }
}
```

Only use the keys shown here (`command`, `args`, `cwd`). Do not invent config keys.
When merging into an existing file, preserve the other `mcpServers` entries and
re-emit strictly valid JSON (no comments, no trailing commas).

## 4. Tell the user to reload, and how refresh works

Report back clearly:

- **Reload to connect.** Claude does not pick up a new or changed `.mcp.json`
  mid-session. The user must reload / restart Claude Code (and approve the `hexser`
  MCP server when prompted) before it will connect and the server's resources
  become available.
- **Restart to see new components.** The server exposes the project's architecture
  from hexser's link-time inventory registry, which is fixed when the binary is
  compiled. The `hexser/refresh` method rebuilds the project (`cargo build` with
  `--features macros`) but does NOT re-register components into the running process.
  So after adding new domain/port/adapter/etc. components, the user must fully
  **stop and relaunch the MCP server** — calling `hexser/refresh` alone is not
  enough to surface newly added components.

Do not claim hexser starts any MCP server automatically; this command is the opt-in
setup. Once connected, the server serves `hexser://{project}/context` (AIContext
JSON) and `hexser://{project}/pack` (AgentPack JSON).
