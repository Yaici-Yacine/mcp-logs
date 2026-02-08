# 🔄 Restart Feature - MCP Remote Process Control

## Overview

The `restart_process` feature allows AI assistants (via MCP) to remotely restart processes being monitored by the mcp-log-agent. This enables intelligent process management based on log analysis, error detection, or manual intervention.

## ✨ Key Features

- **🤖 AI-Driven Restarts**: Let your AI assistant restart processes based on log analysis
- **🔄 Bidirectional Communication**: Socket-based command protocol between MCP server and agents
- **📡 Works in All Modes**: TUI mode (--watch) and one-shot mode
- **✅ Graceful Shutdown**: SIGTERM first, then SIGKILL if needed
- **📊 Real-time Feedback**: Status updates sent back to MCP server
- **🎯 Project-Specific**: Target specific projects when multiple agents are running

## Architecture

### Communication Flow

```
┌─────────────────┐         ┌──────────────────┐         ┌─────────────────┐
│   AI Assistant  │         │   MCP Server     │         │  mcp-log-agent  │
│   (Claude, etc) │         │   (TypeScript)   │         │     (Rust)      │
└────────┬────────┘         └────────┬─────────┘         └────────┬────────┘
         │                           │                            │
         │  restart_process call     │                            │
         ├──────────────────────────>│                            │
         │                           │                            │
         │                           │  Command JSON              │
         │                           │  (via Unix socket)         │
         │                           ├───────────────────────────>│
         │                           │                            │
         │                           │                     ┌──────▼──────┐
         │                           │                     │   Restart   │
         │                           │                     │   Process   │
         │                           │                     └──────┬──────┘
         │                           │                            │
         │                           │  Response JSON             │
         │                           │<───────────────────────────┤
         │                           │                            │
         │  Success response         │                            │
         │<──────────────────────────┤                            │
         │                           │                            │
```

### Protocol Messages

#### 1. Command Message (MCP → Agent)

```json
{
  "version": "1.0",
  "type": "command",
  "data": {
    "command": "restart",
    "project": "my-app",
    "requestId": "uuid-123"
  }
}
```

#### 2. Command Response (Agent → MCP)

**Success:**
```json
{
  "version": "1.0",
  "type": "command_response",
  "data": {
    "requestId": "uuid-123",
    "success": true,
    "message": "Process restarted successfully",
    "pid": 54321,
    "project": "my-app"
  }
}
```

**Error:**
```json
{
  "version": "1.0",
  "type": "command_response",
  "data": {
    "requestId": "uuid-123",
    "success": false,
    "message": "Failed to start process: command not found",
    "pid": null,
    "project": "my-app"
  }
}
```

## Usage

### Via MCP Tool

In your MCP client (Claude Desktop, OpenCode, Cline):

```
Can you restart the 'frontend' project?
```

Or directly:

```json
{
  "tool": "restart_process",
  "arguments": {
    "project": "frontend"
  }
}
```

### Response

```json
{
  "success": true,
  "message": "Restart command sent to project 'frontend'. The agent will stop the current process and start a new one.",
  "project": "frontend",
  "note": "Check the agent's logs for confirmation and the new process PID."
}
```

## Example Use Cases

### 1. Error Detection & Auto-Restart

**Scenario**: AI detects repeated connection errors

```
AI: I see connection errors in the logs. Let me restart the service.
    → Calls restart_process { "project": "backend" }
    
Agent: ✓ Process restarted via MCP (PID: 12345)
```

### 2. Config Change Application

**Scenario**: User modifies configuration file

```
User: I just updated the database config in .env
AI: I'll restart the application to apply the changes.
    → Calls restart_process { "project": "my-app" }
    
Agent: 🔄 Restart requested via MCP
        ✓ Process restarted via MCP (PID: 23456)
```

### 3. Memory Leak Mitigation

**Scenario**: AI detects increasing memory usage

```
AI: Memory usage has grown to 2GB. Restarting to free resources.
    → Calls restart_process { "project": "api-server" }
    
Agent: ✓ Process restarted via MCP (PID: 34567)
```

### 4. Multi-Project Orchestration

**Scenario**: Restarting services in order

```
AI: I'll restart the services in the correct order:
    1. → restart_process { "project": "database" }
    2. Wait 5 seconds
    3. → restart_process { "project": "backend" }
    4. Wait 3 seconds  
    5. → restart_process { "project": "frontend" }
```

## Testing

### Manual Test (Simulating MCP)

#### 1. Start the MCP Server

```bash
cd mcp-logs
bun run index.ts
```

#### 2. Start an Agent

```bash
cd log-agent
./target/release/mcp-log-agent run --project test-app --watch -- npm start
```

#### 3. Send Restart Command

```bash
# Using send-restart-command.sh
./send-restart-command.sh test-app

# Or manually with netcat
echo '{"version":"1.0","type":"command","data":{"command":"restart","project":"test-app","requestId":"test-123"}}' | nc -U /tmp/log-agent.sock
```

#### 4. Observe

In the TUI, you should see:
1. `🔄 Restart requested via MCP`
2. Process stopping
3. New process starting
4. `✓ Process restarted via MCP (PID: XXXXX)`

### Automated Test Script

```bash
./test-restart-mcp.sh
```

This script:
- Creates a test process
- Provides instructions for testing
- Creates helper script to send restart commands

## Implementation Details

### Rust Agent (mcp-log-agent)

**Files Modified:**

1. **`src/types/mod.rs`**
   - Added `CommandMessage`, `CommandResponse` types
   - Added `RestartCommand` for internal communication

2. **`src/socket.rs`**
   - Transformed from unidirectional to bidirectional
   - Added `start_command_listener()` for receiving commands
   - Added `send_command_response()` for sending responses
   - Uses separate connection for reading commands

3. **`src/tui/mod.rs`**
   - Added `restart_rx` channel to receive restart commands
   - New branch in `tokio::select!` to handle MCP restarts
   - Sends responses back to MCP server

### TypeScript MCP Server (mcp-logs)

**Files Modified:**

1. **`src/types/index.ts`**
   - Added `CommandMessage`, `CommandResponse` interfaces

2. **`src/server/index.ts`**
   - Added `clients: Map<string, socket>` to track connections
   - Added `sendCommand()` method to send commands to agents
   - Modified `handleData()` to process command responses

3. **`src/mcp/tools.ts`**
   - Added `restart_process` tool definition

4. **`src/mcp/handlers.ts`**
   - Added `restartProcess()` handler
   - Validates project exists before sending command

## Error Handling

### Common Errors

| Error | Cause | Solution |
|-------|-------|----------|
| `Project 'X' is not connected` | Agent not running or different project name | Start agent or check project name |
| `Failed to send restart command` | Socket communication issue | Check socket permissions, restart MCP server |
| `Process restart failed: command not found` | Invalid command in agent config | Fix command in mcp-log-agent config |
| `Connection timeout` | Agent not responding | Check agent is running, check socket path |

### Debugging

**Enable verbose mode:**

```bash
# MCP Server
VERBOSE=true bun run index.ts

# Agent
mcp-log-agent run --verbose --project test --watch -- command
```

**Check connections:**

```bash
# List connected projects
# In MCP client:
list_projects
```

**Monitor socket:**

```bash
# Watch socket activity
sudo lsof /tmp/log-agent.sock
```

## Configuration

### Socket Path

Both server and agent must use the same socket path:

**Agent (.mcp-log-agent.toml):**
```toml
[agent]
socket_path = "/tmp/log-agent.sock"
```

**Server (.mcp-logs.json):**
```json
{
  "server": {
    "socket_path": "/tmp/log-agent.sock"
  }
}
```

### Auto-Restart on Exit

Control what happens when a process exits:

```toml
[agent]
auto_quit = false  # Don't quit when process exits
watch = true       # Keep watching and allow restarts
```

## Limitations

1. **Unix Sockets Only**: Works on Linux/macOS only (no Windows support yet)
2. **Local Communication**: Server and agents must be on the same machine
3. **No Restart History**: Previous restart attempts are not logged (yet)
4. **Single Command Type**: Currently only "restart" command is supported

## Future Enhancements

- [ ] Add `stop_process` command
- [ ] Add `get_process_status` command
- [ ] Add restart history/audit log
- [ ] Support for Windows Named Pipes
- [ ] Scheduled restarts (cron-like)
- [ ] Rolling restarts for multiple instances
- [ ] Restart with different command/args
- [ ] Health check before considering restart successful

## Security Considerations

- **Unix Socket Permissions**: Socket file has default permissions (check with `ls -l /tmp/log-agent.sock`)
- **Local Only**: No network exposure
- **No Authentication**: Any process with socket access can send commands
- **Command Validation**: Only whitelisted commands are accepted

## Changelog

### Version 1.2.0 (2025-01-20)

- ✨ **NEW**: `restart_process` MCP tool
- 🔄 **NEW**: Bidirectional socket communication
- 📡 **NEW**: Command protocol for agent control
- 🎯 **NEW**: Project-specific command targeting
- ✅ **NEW**: Command response feedback to MCP
- 📝 **NEW**: TUI messages for MCP-initiated restarts
- 🧪 **NEW**: Test scripts for manual testing

## Support

For issues or questions:
- GitHub Issues: https://github.com/Yaici-Yacine/mcp-logs/issues
- Email: yaiciy01@gmail.com

## License

MIT License - see LICENSE file for details.
