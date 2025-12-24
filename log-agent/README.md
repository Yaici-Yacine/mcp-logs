# mcp-log-agent

[![Crates.io](https://img.shields.io/crates/v/mcp-log-agent.svg)](https://crates.io/crates/mcp-log-agent)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Real-time log capture CLI for development projects with MCP (Model Context Protocol) integration.

## Features

- Capture stdout/stderr from any process in real-time
- Stream logs to MCP server via Unix socket
- JSON-based structured logging
- Automatic log level inference (info, warn, error, debug)
- Support for multiple simultaneous projects
- Zero file I/O - all logs in memory

## Installation

```bash
cargo install mcp-log-agent
```

Or build from source:

```bash
git clone https://github.com/Yaici-Yacine/mcp-logs.git
cd mcp-logs/log-agent
cargo build --release
```

## Quick Start

### 1. Start the MCP server

The MCP server must be running to receive logs. See [mcp-logs documentation](https://github.com/Yaici-Yacine/mcp-logs) for setup instructions.

### 2. Capture logs from your project

```bash
# Capture from any command
mcp-log-agent run --project my-app bun dev

# Node.js project
mcp-log-agent run --project api npm start

# Rust project
mcp-log-agent run --project backend cargo run

# Python project
mcp-log-agent run --project ml-script python train.py
```

### 3. Test the connection

```bash
mcp-log-agent test --message "Hello from CLI"
```

## Usage

### Run Command

Spawn a process and capture its logs:

```bash
mcp-log-agent run --project <PROJECT_NAME> <COMMAND> [ARGS...]
```

**Options:**
- `--project, -p`: Project name for identification (default: "default")
- Command and arguments: The command to run with its arguments

**Examples:**

```bash
# Web server
mcp-log-agent run --project frontend bun dev

# Build process
mcp-log-agent run --project build npm run build

# Tests
mcp-log-agent run --project tests cargo test

# Shell script
mcp-log-agent run --project demo bash ./script.sh
```

### Test Command

Test the connection to the MCP server:

```bash
mcp-log-agent test [--message <TEXT>]
```

**Options:**
- `--message, -m`: Custom test message (optional)

## Log Format

Logs are sent as newline-delimited JSON to the Unix socket:

```json
{
  "version": "1.0",
  "type": "log_entry",
  "data": {
    "timestamp": "2025-12-24T10:30:45.123Z",
    "level": "info",
    "source": "stdout",
    "project": "my-app",
    "message": "Server started on port 3000",
    "pid": 12345
  }
}
```

**Log Levels:**
- `info` - Informational messages
- `warn` - Warning messages
- `error` - Error messages
- `debug` - Debug messages

Levels are automatically inferred from message content.

**Sources:**
- `stdout` - Standard output
- `stderr` - Standard error

## Configuration

### Socket Path

Default: `/tmp/log-agent.sock`

To change the socket path, modify `SOCKET_PATH` in `src/socket.rs` and recompile.

## Integration with MCP

This CLI works in tandem with the MCP server to provide real-time log analysis capabilities:

1. **Start the MCP server** - Handles incoming logs and provides query tools
2. **Run mcp-log-agent** - Captures and streams logs from your projects
3. **Query via MCP tools** - Search, filter, and analyze logs in real-time

See the [complete documentation](https://github.com/Yaici-Yacine/mcp-logs) for MCP server setup and usage.


## Use Cases

- **Development Monitoring**: Capture logs from dev servers in real-time
- **Debugging**: Stream logs to AI assistants for analysis
- **Multi-Project Management**: Monitor multiple projects simultaneously
- **Testing**: Capture test output for analysis
- **CI/CD Integration**: Stream build logs for real-time monitoring

## Requirements

- Rust 1.70+ (for building from source)
- Unix-like system (Linux, macOS) - Windows not yet supported
- MCP server running (for log reception)

## Limitations

- Unix sockets only (no Windows support yet)
- Logs stored in memory on MCP server (no persistence by default)
- Cannot attach to existing processes (only spawn new ones)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

Yacine Yaici - yaiciy01@gmail.com

## Related Projects

- [mcp-logs](https://github.com/Yaici-Yacine/mcp-logs) - The complete MCP logging system
- [MCP Protocol](https://modelcontextprotocol.io/) - Model Context Protocol specification

## Changelog

### 0.1.0 (2025-12-24)

- Initial release
- Real-time log capture from any process
- Unix socket streaming to MCP server
- Automatic log level inference
- Multi-project support
- Test command for connection verification
