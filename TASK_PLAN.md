---
task_id: mcp-concurrent-access-support
status: completed
---

# Task: MCP Server Concurrent Access During Active Development

## Problem Statement

**User Scenario**: "Sometimes, an AI agent will be working on the project and also need to call the MCP server on the project."

This describes a concurrent access pattern where:
1. **AI Agent Activity**: An AI assistant is actively modifying project code (adding components, changing architecture)
2. **MCP Server Queries**: The same AI assistant queries the MCP server to understand the current architecture
3. **Synchronization Gap**: Changes made to code aren't reflected in MCP responses until recompilation

### Current Behavior

The MCP server serves architecture data from `HexGraph::current()`, which is built via the `inventory` crate at **compile time**:

```rust
// hexser/src/graph/hex_graph.rs
impl HexGraph {
    pub fn current() -> std::sync::Arc<Self> {
        let builder = GraphBuilder::new();
        for entry in inventory::iter::<ComponentEntry> {
            builder.add_component_entry(entry);
        }
        builder.build()
    }
}
```

**Key Limitation**: The `inventory` system collects components via `inventory::submit!` macros during compilation. Runtime code changes (adding new `#[derive(HexEntity)]` structs, etc.) are **not visible** until:
1. Code is saved
2. Project is recompiled
3. MCP server is restarted with new binary

### Impact

**Scenario Example**:
1. AI agent adds new `OrderEntity` struct with `#[derive(HexEntity)]`
2. AI agent immediately queries `hexser://myproject/context` to verify addition
3. **Problem**: MCP server returns old graph without `OrderEntity` because compilation hasn't occurred

**Result**: AI agent has stale architecture view, leading to:
- Incorrect assumptions about component relationships
- Duplicate component creation attempts
- Confusion about what exists in the codebase

## Architecture Analysis

### Graph Construction Flow

```
┌─────────────────┐
│  Source Code    │
│  #[derive(...)] │
└────────┬────────┘
         │ Compilation
         ▼
┌─────────────────┐
│ inventory::     │
│ submit!()       │
│ (proc macro)    │
└────────┬────────┘
         │ Link Time
         ▼
┌─────────────────┐
│ Static Registry │
│ (embedded)      │
└────────┬────────┘
         │ Runtime
         ▼
┌─────────────────┐
│ HexGraph::      │
│ current()       │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  MCP Server     │
│  (serves graph) │
└─────────────────┘
```

**Critical Path**: Code → Compile → Inventory → Graph → MCP

### Synchronization Challenges

1. **Compilation Barrier**: Rust requires full compilation to execute new code
2. **No Runtime Reflection**: Rust has no runtime type introspection for custom derives
3. **Static Registry**: `inventory` crate is compile-time only, no dynamic registration
4. **Process Isolation**: MCP server runs as separate process from compiler

## Solution Options

### Option A: Automatic Recompilation with File Watching

**Concept**: Monitor source files, trigger recompilation, restart MCP server automatically.

**Implementation**:
```rust
pub struct McpWatchServer {
    registry: ProjectRegistry,
    watcher: notify::RecommendedWatcher,
    compile_queue: std::sync::mpsc::Receiver<ProjectName>,
}

impl McpWatchServer {
    fn watch_project(&mut self, project: &ProjectConfig) {
        // Watch src/ directory for changes
        self.watcher.watch(&project.src_path, RecursiveMode::Recursive)?;
    }
    
    fn handle_file_change(&mut self, event: notify::Event) {
        if event.paths.iter().any(|p| p.extension() == Some("rs")) {
            // Trigger recompilation
            self.schedule_rebuild(project_name);
        }
    }
    
    fn rebuild_project(&mut self, project_name: &str) -> HexResult<()> {
        // Run: cargo build -p {project_name} --features macros
        let output = std::process::Command::new("cargo")
            .args(&["build", "-p", project_name, "--features", "macros"])
            .output()?;
        
        if output.status.success() {
            // Reload graph from newly compiled binary
            let new_graph = Self::load_graph_from_binary(project_name)?;
            self.registry.update_graph(project_name, new_graph);
        }
    }
}
```

**Pros**:
- Fully automatic synchronization
- No manual intervention required
- Real-time architecture updates

**Cons**:
- Complex: requires file watching, process management, graph reloading
- Performance: frequent recompilations during active development
- Reliability: compilation errors break synchronization
- Platform-specific: file watching behavior varies across OS

**Effort**: 8-12 hours implementation + testing

---

### Option B: Explicit Refresh Endpoint

**Concept**: Add MCP method for clients to explicitly request graph refresh.

**Implementation**:
```rust
// Add new MCP method
impl McpServer for McpStdioServer {
    fn refresh_project(&mut self, project: &str) -> HexResult<RefreshResult> {
        // Trigger compilation
        self.compile_project(project)?;
        
        // Reload graph
        let new_graph = HexGraph::load_from_binary(project)?;
        self.registry.update_graph(project, new_graph)?;
        
        Ok(RefreshResult {
            status: "success",
            components_added: 3,
            components_removed: 1,
        })
    }
}
```

**MCP JSON-RPC**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "hexser/refresh",
  "params": {
    "project": "myproject"
  }
}
```

**AI Agent Workflow**:
1. Agent makes code changes
2. Agent calls `hexser/refresh` with project name
3. MCP server recompiles and reloads
4. Agent queries `hexser://myproject/context` for updated graph

**Pros**:
- Simple implementation
- Client controls timing (avoid unnecessary rebuilds)
- Clear failure handling (compilation errors returned to client)
- No background threads or file watchers

**Cons**:
- Manual: AI must remember to call refresh
- Blocking: client waits for compilation
- No automatic sync

**Effort**: 2-3 hours implementation

---

### Option C: Document Limitations + Best Practices

**Concept**: Accept current behavior, document clearly, provide usage guidelines.

**Documentation**:
```markdown
## MCP Server Architecture Data

⚠️ **Important**: The MCP server serves architecture data from the most recent
compilation. Changes to source code are NOT reflected until:

1. Code is saved
2. Project is recompiled (cargo build)
3. MCP server is restarted (if using long-running process)

### Recommended Workflow for AI Agents

**Pattern 1: Query-First**
1. Query current architecture via MCP
2. Plan changes based on current state
3. Make code changes
4. Restart MCP server before next query session

**Pattern 2: Batch Changes**
1. Make all planned code changes
2. Trigger compilation (cargo build)
3. Restart MCP server
4. Query updated architecture

**Pattern 3: Development Mode**
Run MCP server in one-shot mode for each query:
```
# Query with fresh compilation
cargo build && cargo run --bin hex-mcp-server < query.json
```

### Integration with AI Assistants

Configure AI tools to trigger compilation before MCP queries:

```json
{
  "mcpServers": {
    "hexser": {
      "command": "sh",
      "args": ["-c", "cargo build -q && cargo run --bin hex-mcp-server"],
      "cwd": "/path/to/project"
    }
  }
}
```
```

**Pros**:
- Zero implementation effort
- No complexity added
- Works with current architecture
- Educates users on Rust compilation model

**Cons**:
- Manual workflow required
- Risk of stale data if users forget
- Not seamless for AI agents

**Effort**: 1 hour documentation

---

### Option D: Hybrid Approach (Recommended)

**Concept**: Combine explicit refresh (Option B) with clear documentation (Option C).

**Implementation**:

1. **Add `hexser/refresh` method** (2-3 hours)
   - Manual trigger for clients
   - Returns compilation status
   - Updates graph on success

2. **Document limitation clearly** (1 hour)
   - Explain compile-time nature
   - Provide best practices
   - Show refresh method usage

3. **Optional: Add health check** (1 hour)
   ```rust
   // New MCP method
   fn project_health(&self, project: &str) -> HexResult<HealthStatus> {
       let config = self.registry.get(project)?;
       let src_modified = Self::get_last_modified(&config.src_path)?;
       let graph_timestamp = config.graph_built_at;
       
       Ok(HealthStatus {
           in_sync: src_modified <= graph_timestamp,
           last_modified: src_modified,
           last_compiled: graph_timestamp,
       })
   }
   ```

4. **Optional: Add auto-compile flag** (future enhancement)
   ```toml
   [hexser.mcp]
   auto_compile = true  # Enable Option A behavior
   ```

**Pros**:
- Flexible: supports both manual and automatic workflows
- Progressive: start simple, add features later
- Clear: documentation sets expectations
- Extensible: can add watching later if needed

**Cons**:
- Still requires client awareness
- Initial version not fully automatic

**Effort**: 4-5 hours total

---

## Recommendation

**Implement Option D: Hybrid Approach**

### Phase 1: Explicit Refresh (Immediate)
- Add `hexser/refresh` MCP method
- Document usage in README.md
- Provide client examples

### Phase 2: Enhanced Documentation (Immediate)
- Document compile-time limitation
- Provide best practices for AI workflows
- Show Claude Desktop configuration

### Phase 3: Health Monitoring (Future)
- Add `hexser/health` method
- Track source vs graph staleness
- Warn clients when refresh needed

### Phase 4: Auto-Compilation (Future, Optional)
- Add file watching capability
- Make opt-in via config flag
- Implement only if users request

### Rationale

1. **Pragmatic**: Solves immediate problem without overengineering
2. **Rust-Aligned**: Respects compilation model, doesn't fight it
3. **Flexible**: Clients choose manual vs automatic workflow
4. **Extensible**: Can add automation later based on feedback

## Implementation Plan

### Phase 1: Explicit Refresh (3 hours)

#### 1.1: Add Refresh Domain Types
```rust
// hexser/src/domain/mcp/refresh.rs
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RefreshRequest {
    pub project: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RefreshResult {
    pub status: String,
    pub compiled: bool,
    pub components_added: usize,
    pub components_removed: usize,
    pub error: Option<String>,
}
```

#### 1.2: Update McpServer Port
```rust
// hexser/src/ports/mcp_server.rs
pub trait McpServer {
    // ... existing methods ...
    fn refresh_project(&mut self, request: RefreshRequest) -> HexResult<RefreshResult>;
}
```

#### 1.3: Implement in McpStdioServer
```rust
// hexser/src/adapters/mcp_stdio.rs
impl McpServer for McpStdioServer {
    fn refresh_project(&mut self, request: RefreshRequest) -> HexResult<RefreshResult> {
        let config = self.registry.get(&request.project)
            .ok_or_else(|| Hexserror::not_found("Project", &request.project))?;
        
        // Run cargo build
        let output = std::process::Command::new("cargo")
            .args(&["build", "-p", &request.project, "--features", "macros"])
            .current_dir(&config.root_path)
            .output()
            .map_err(|e| Hexserror::adapter("E_COMPILE", &format!("{}", e)))?;
        
        if !output.status.success() {
            return Ok(RefreshResult {
                status: "error".into(),
                compiled: false,
                components_added: 0,
                components_removed: 0,
                error: Some(String::from_utf8_lossy(&output.stderr).into()),
            });
        }
        
        // Reload graph (note: requires dynamic loading mechanism)
        // For now, return success but require server restart
        Ok(RefreshResult {
            status: "compiled".into(),
            compiled: true,
            components_added: 0,
            components_removed: 0,
            error: Some("Server restart required to load new graph".into()),
        })
    }
}
```

#### 1.4: Add Tests
- Test refresh with valid project
- Test refresh with invalid project
- Test compilation error handling

### Phase 2: Documentation (1 hour)

Update README.md:
- Add "Concurrent Development" section
- Document `hexser/refresh` method
- Provide AI assistant configuration examples
- Explain compilation requirement

### Phase 3: Future Enhancements (TBD)

- Dynamic graph reloading (requires loading compiled .rlib or .so)
- File watching with auto-compile
- Health check endpoint
- Compile-on-query option

## Technical Challenges

### Challenge 1: Dynamic Graph Reloading

**Problem**: After compilation, need to load new `HexGraph` from updated binary.

**Options**:
1. **Restart MCP server** (simplest, but disruptive)
2. **Dynamic library loading** (complex, requires .so/.dylib build)
3. **Parse Cargo metadata** (incomplete, misses custom code)
4. **File-based serialization** (requires graph export during build)

**Recommendation**: Start with server restart requirement, add dynamic loading later if critical.

### Challenge 2: Compilation Performance

**Problem**: Recompiling on every change is slow for large projects.

**Solutions**:
- Use incremental compilation (cargo default)
- Only rebuild specific workspace member
- Cache compilation results
- Provide manual refresh (don't auto-compile)

### Challenge 3: Error Handling

**Problem**: Compilation errors should not crash MCP server.

**Solution**: Return compilation errors as structured data in `RefreshResult`.

## Success Criteria

- [ ] `hexser/refresh` method implemented and tested
- [ ] Documentation explains compile-time limitation
- [ ] Client examples provided for Claude Desktop
- [ ] AI workflow best practices documented
- [ ] Compilation errors handled gracefully
- [ ] All existing tests pass
- [ ] New tests cover refresh scenarios

## Current Step

Phase 1: Analyzing problem and documenting solution options

## Blockers

None currently.

## Next Actions

1. Get user feedback on recommended approach (Option D)
2. Confirm `hexser/refresh` method is acceptable solution
3. Decide on dynamic graph reloading vs server restart requirement
4. Begin Phase 1 implementation if approved

## Questions for User

1. **Preferred Solution**: Option D (hybrid) acceptable, or prefer full automation (Option A)?
2. **Refresh Behavior**: Is server restart after compile acceptable, or need dynamic reloading?
3. **AI Integration**: Which AI tools are you using (Claude, Cline, other)?
4. **Development Workflow**: How frequently are you querying MCP during active coding?
5. **Performance Tolerance**: How long is acceptable for refresh operation (seconds? minutes?)?

## Estimated Effort

### Option D (Recommended):
- Phase 1 (Refresh method): 3 hours
- Phase 2 (Documentation): 1 hour
- Testing: 1 hour
- **Total**: 5 hours

### Option A (Full Auto-Compile):
- File watching: 3 hours
- Compilation management: 3 hours
- Graph reloading: 4 hours
- Testing: 2 hours
- **Total**: 12 hours

### Option C (Documentation Only):
- **Total**: 1 hour

## Notes

- The fundamental challenge is Rust's compilation model: no runtime reflection for macros
- Any solution involving updated architecture data requires recompilation
- The question is: automatic vs manual, and how to handle graph reloading
- Similar challenge exists for hot-reloading in other Rust tools (e.g., cargo-watch)
- MCP specification doesn't define resource invalidation/refresh patterns
