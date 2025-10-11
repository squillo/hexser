# MCP Server Setup for IntelliJ + Junie Agent: Beginner's Walkthrough

## What is MCP and Why Should I Use It?

**MCP (Model Context Protocol)** is a standardized way for AI assistants to query information about your project in real-time. Think of it as a "live documentation server" that your AI agent can ask questions to.

When you connect Junie (your AI assistant in IntelliJ) to the hexser MCP server, Junie can:
- Understand your project's architecture (what components exist, how they connect)
- See your domain entities, ports, adapters, and application layers
- Make smarter suggestions that follow your hexagonal architecture patterns
- Avoid creating duplicate components or breaking architectural rules

**Without MCP:** Junie only sees the code files you show it (limited context).  
**With MCP:** Junie can query your entire architecture graph whenever needed.

---

## Prerequisites

Before starting, make sure you have:

1. **Rust toolchain installed** (rustc, cargo)
   - Check by running: `rustc --version` in your terminal
   - If not installed, visit: https://rustup.rs

2. **A hexser project** (the project you're working on)
   - This should be a Rust workspace using the hexser framework
   - Project must use hexser with the `mcp` feature enabled

3. **IntelliJ IDEA** with Junie agent installed
   - Junie is your AI assistant within IntelliJ
   - Make sure Junie is configured and working

4. **Basic terminal/command-line knowledge**
   - You'll need to run a few commands
   - Don't worry, we'll explain each one!

---

## Step 1: Verify Your Hexser Project Has MCP Support

### What You're Doing
Checking if your project has the MCP feature enabled in its configuration.

### How to Do It

1. **Open your project's root directory** in your terminal:
   ```bash
   cd /path/to/your/hexser/project
   ```

2. **Find your main `Cargo.toml` file** (in the project root)

3. **Look for hexser dependency** with `mcp` feature:
   ```toml
   [dependencies]
   hexser = { version = "0.4.5", features = ["mcp", "macros"] }
   ```

   Or in workspace dependencies:
   ```toml
   [workspace.dependencies]
   hexser = { version = "0.4.5", path = "hexser", features = ["mcp", "macros"] }
   ```

### ✅ Success Check
You should see `"mcp"` listed in the features array for hexser.

### ⚠️ If MCP is Missing
Add `"mcp"` to the features list:
```toml
hexser = { version = "0.4.5", features = ["mcp", "macros"] }
```

Then rebuild your project:
```bash
cargo build
```

---

## Step 2: Test the MCP Server (Manual Test)

### What You're Doing
Running the MCP server manually to verify it works before connecting Junie.

### How to Do It

1. **Open a terminal** in your project's root directory

2. **Run the MCP server:**
   ```bash
   cargo run --features mcp --bin hex-mcp-server
   ```

   If your project is in a workspace (like hexser itself):
   ```bash
   cargo run -p hexser --features mcp --bin hex-mcp-server
   ```

3. **You should see:** The terminal cursor waiting for input (this is normal!)
   - The MCP server is now running and listening for commands
   - It reads from stdin (keyboard input) and writes to stdout (terminal output)

4. **Test with a simple command:**
   
   Copy this JSON and paste it into the terminal, then press Enter:
   ```json
   {"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05"}}
   ```

5. **You should see a JSON response** like:
   ```json
   {"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"2024-11-05","serverInfo":{"name":"hexser","version":"0.4.5"},"capabilities":{"resources":{}}}}
   ```

### ✅ Success Check
If you get a JSON response back, the MCP server is working!

### ⚠️ Troubleshooting

**Problem:** "error: no bin target named `hex-mcp-server`"  
**Solution:** Your project might not have the MCP server binary. Check that hexser is a dependency with `mcp` feature enabled.

**Problem:** Compilation errors  
**Solution:** Run `cargo build --features mcp` first to see detailed error messages.

**Problem:** Nothing happens after pasting JSON  
**Solution:** Make sure you pressed Enter after pasting. The server reads line-by-line.

6. **Stop the server:** Press `Ctrl+C` in the terminal

---

## Step 3: Configure Junie to Connect to MCP Server

### What You're Doing
Setting up Junie's configuration file so it knows how to start and talk to your hexser MCP server.

### How to Do It

1. **Find Junie's configuration location:**
   
   Junie typically stores its configuration in your IntelliJ settings or a dedicated config directory. The exact location depends on your Junie installation. Common locations:
   
   - **macOS/Linux:** `~/.config/junie/mcp_servers.json` or similar
   - **Windows:** `%APPDATA%\junie\mcp_servers.json` or similar
   
   **Note:** Check Junie's documentation or settings panel in IntelliJ for the exact path.

2. **Create or edit the MCP servers configuration file:**

   If Junie uses a JSON configuration file (similar to Claude Desktop), it should look like this:

   ```json
   {
     "mcpServers": {
       "hexser": {
         "command": "cargo",
         "args": [
           "run",
           "-p",
           "hexser",
           "--features",
           "mcp",
           "--bin",
           "hex-mcp-server"
         ],
         "cwd": "/absolute/path/to/your/hexser/project"
       }
     }
   }
   ```

3. **Important: Replace the `cwd` value:**
   
   Change `/absolute/path/to/your/hexser/project` to your actual project path:
   
   **macOS/Linux example:**
   ```json
   "cwd": "/Users/yourname/projects/my-hexser-app"
   ```
   
   **Windows example:**
   ```json
   "cwd": "C:\\Users\\yourname\\projects\\my-hexser-app"
   ```

4. **If your project is NOT named "hexser":**
   
   Adjust the `-p` parameter to match your project name:
   ```json
   "args": [
     "run",
     "-p",
     "your-project-name",  // <- Change this
     "--features",
     "mcp",
     "--bin",
     "hex-mcp-server"
   ]
   ```

5. **If your project is a single crate (not a workspace):**
   
   Simplify the args:
   ```json
   "args": [
     "run",
     "--features",
     "mcp",
     "--bin",
     "hex-mcp-server"
   ]
   ```

### ✅ Success Check
Configuration file saved with correct paths and project name.

### 📝 Alternative: IntelliJ Plugin Settings

If Junie uses IntelliJ's built-in settings UI instead of a JSON file:

1. **Open IntelliJ Settings** (File → Settings on Windows/Linux, IntelliJ IDEA → Preferences on macOS)

2. **Navigate to Junie's MCP settings** (look for "Junie" or "AI Assistant" in the settings tree)

3. **Add a new MCP server** with these details:
   - **Name:** hexser
   - **Command:** cargo
   - **Arguments:** `run -p hexser --features mcp --bin hex-mcp-server` (adjust project name as needed)
   - **Working Directory:** `/absolute/path/to/your/project`

4. **Save settings**

---

## Step 4: Restart Junie/IntelliJ

### What You're Doing
Making sure Junie picks up the new MCP configuration.

### How to Do It

**Option A: Restart IntelliJ completely**
1. Close IntelliJ IDEA
2. Reopen your project
3. Wait for indexing to complete

**Option B: Restart Junie plugin** (if available)
1. Look for a "Restart Junie" option in Junie's menu
2. Click it and wait for restart

### ✅ Success Check
IntelliJ reopens successfully and Junie is active.

---

## Step 5: Verify the Connection

### What You're Doing
Testing that Junie can successfully connect to your hexser MCP server.

### How to Do It

1. **Open the Junie chat/assistant panel** in IntelliJ

2. **Ask Junie a test question about your architecture:**
   
   Try one of these:
   - "What hexser components are registered in this project?"
   - "Show me the architecture layers in this project"
   - "What domain entities exist in the hexser graph?"

3. **Junie should query the MCP server** and respond with actual architecture information from your project

### ✅ Success Check
Junie responds with specific component names, layers, or entity information from your project.

### ⚠️ Troubleshooting

**Problem:** Junie says "I don't have access to that information" or gives generic responses  
**Solution:** 
- Check that MCP server configuration is saved
- Restart IntelliJ/Junie
- Check Junie's logs for connection errors (look in IntelliJ's log files or Junie's output panel)

**Problem:** Junie shows connection errors  
**Solution:**
- Verify the `cwd` path is correct and absolute (not relative like `./project`)
- Make sure cargo is in your PATH (run `which cargo` in terminal)
- Test the MCP server manually again (Step 2) to ensure it runs

**Problem:** Junie connects but returns empty architecture  
**Solution:**
- Your project might not have components registered yet
- Make sure you're using hexser derive macros like `#[derive(HexEntity)]`
- Run `cargo build --features macros` to ensure components are compiled

---

## Step 6: Understanding What Junie Can Query

### Available Resources

Once connected, Junie can query two main resources:

1. **Architecture Context** (`hexser://your-project/context`)
   - List of all components (entities, ports, adapters, etc.)
   - Component relationships and dependencies
   - Layer organization (Domain, Port, Application, Adapter)
   - Architectural roles (Entity, Repository, Directive, Query, etc.)

2. **Agent Pack** (`hexser://your-project/pack`)
   - Everything from Architecture Context
   - Your project's coding guidelines
   - Embedded documentation
   - Best practices and patterns

### Example Queries to Try

Ask Junie:
- "What domain entities are defined in this project?"
- "Show me all the repository ports"
- "What adapters implement the UserRepository port?"
- "List all directives in the application layer"
- "What are the architectural constraints for this project?"

---

## Step 7: Refreshing After Code Changes

### The Challenge

When you add new components (like a new entity with `#[derive(HexEntity)]`), the MCP server doesn't automatically see them. This is because hexser uses Rust's compile-time registration.

### How to Refresh

**Method 1: Ask Junie to Refresh (Recommended)**

Simply tell Junie:
> "Refresh the hexser architecture graph"

Junie should call the `hexser/refresh` method, which:
1. Recompiles your project
2. Reports compilation success or errors
3. Asks you to restart the MCP connection

**Method 2: Manual Refresh**

1. Stop the MCP server (if running manually)
2. Rebuild your project:
   ```bash
   cargo build --features macros
   ```
3. Restart IntelliJ/Junie to reconnect

### After Refresh

**Important:** After a successful compilation, you must **restart the MCP connection** (usually by restarting IntelliJ or Junie) to load the updated architecture graph.

---

## Common Workflows

### Workflow 1: Adding a New Entity

1. **Create your entity:**
   ```rust
   #[derive(HexEntity, HexDomain, Clone, Debug)]
   pub struct Customer {
       pub id: String,
       pub name: String,
   }
   ```

2. **Tell Junie to refresh:**
   > "Refresh the hexser architecture"

3. **Restart IntelliJ/Junie** after successful compilation

4. **Verify:** Ask Junie "What entities exist?" and confirm Customer is listed

### Workflow 2: Planning Architecture Changes

1. **Query current state:**
   > "Show me all repository ports in this project"

2. **Plan changes:**
   > "I need to add a CustomerRepository port. What should it look like based on existing patterns?"

3. **Implement** based on Junie's suggestions

4. **Refresh and verify** (Workflow 1)

---

## Troubleshooting Guide

### MCP Server Won't Start

**Symptom:** Error when running `cargo run --bin hex-mcp-server`

**Solutions:**
1. Check that `hex-mcp-server` binary exists:
   ```bash
   cargo build --features mcp --bin hex-mcp-server
   ```

2. Verify hexser dependency has `mcp` feature:
   ```toml
   hexser = { version = "0.4.5", features = ["mcp", "macros"] }
   ```

3. Check for compilation errors:
   ```bash
   cargo check --features mcp
   ```

### Junie Can't Connect

**Symptom:** Junie shows connection errors or can't find MCP server

**Solutions:**
1. Verify configuration file path is correct
2. Use **absolute paths** (not relative) for `cwd`
3. Test that cargo works in terminal:
   ```bash
   cargo --version
   ```
4. Check Junie's log files for detailed error messages

### Empty Architecture Graph

**Symptom:** Junie connects but reports no components

**Solutions:**
1. Make sure you have components with hexser derive macros:
   - `#[derive(HexEntity)]`
   - `#[derive(HexPort)]`
   - `#[derive(HexAdapter)]`
   - etc.

2. Rebuild with macros feature:
   ```bash
   cargo build --features macros
   ```

3. Check that components are in your project (not just in hexser library)

### Stale Architecture Data

**Symptom:** Junie doesn't see newly added components

**Solutions:**
1. Use refresh workflow (Step 7)
2. Restart IntelliJ/Junie after recompilation
3. Verify new components compiled successfully

---

## Advanced Configuration

### Multiple Projects

If you work on multiple hexser projects, add separate entries:

```json
{
  "mcpServers": {
    "project-a": {
      "command": "cargo",
      "args": ["run", "-p", "project-a", "--features", "mcp", "--bin", "hex-mcp-server"],
      "cwd": "/path/to/project-a"
    },
    "project-b": {
      "command": "cargo",
      "args": ["run", "-p", "project-b", "--features", "mcp", "--bin", "hex-mcp-server"],
      "cwd": "/path/to/project-b"
    }
  }
}
```

Junie can then connect to either project depending on which one you're working in.

### Custom Binary Names

If your project uses a different binary name:

```json
"args": ["run", "--features", "mcp", "--bin", "my-custom-mcp-server"]
```

---

## Getting Help

### Junie Support
- Check Junie's documentation in IntelliJ (Help → Junie Documentation)
- Look for Junie's output/log panel for error messages

### Hexser MCP Issues
- Read the main README: [hexser/README.md](../README.md) (section: MCP Server)
- Check hexser GitHub issues
- Verify you're using a compatible hexser version (0.4.5+)

### Quick Diagnostic Command

Run this to see if everything is configured correctly:

```bash
cd /path/to/your/project
cargo build --features mcp
cargo run --features mcp --bin hex-mcp-server <<< '{"jsonrpc":"2.0","id":1,"method":"resources/list","params":{}}'
```

You should see JSON output listing available resources.

---

## Summary Checklist

Use this checklist to verify your setup:

- [ ] Rust toolchain installed (`rustc --version` works)
- [ ] Hexser project with `mcp` feature in Cargo.toml
- [ ] MCP server runs manually and responds to test JSON
- [ ] Junie configuration file created with correct paths
- [ ] IntelliJ/Junie restarted
- [ ] Junie can answer questions about project architecture
- [ ] Refresh workflow tested and working

**Congratulations!** You now have Junie connected to your hexser project's live architecture server. Junie can help you maintain clean hexagonal architecture by understanding your project's structure in real-time.

---

## Revision History
- 2025-10-10T20:23:00Z @AI: Initial creation of MCP beginner walkthrough for IntelliJ + Junie users.
