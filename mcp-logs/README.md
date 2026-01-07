# mcp-logs

[![npm version](https://img.shields.io/npm/v/mcp-logs.svg)](https://www.npmjs.com/package/mcp-logs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

MCP (Model Context Protocol) server for real-time log capture and analysis from development projects.

## ⚠️ Requirements

**This package requires [Bun](https://bun.sh) runtime to be installed.**

Install Bun:
```bash
curl -fsSL https://bun.sh/install | bash
```

## Features

- Receive logs from CLI agent via Unix socket
- In-memory log storage (10,000 logs max, FIFO)
- 6 MCP tools for querying and analyzing logs
- Real-time log streaming
- Multi-project support
- Automatic log level inference
- JSON-based protocol

## Installation

### From NPM Registry (recommended)

```bash
# Using npm
npm install -g mcp-logs

# Using bun
bun install -g mcp-logs

# Using pnpm
pnpm install -g mcp-logs
```

> **Important:** This package requires [Bun](https://bun.sh) runtime. Install it first:
> ```bash
> curl -fsSL https://bun.sh/install | bash
> ```

### From Source

```bash
# Clone repository
git clone https://github.com/Yaici-Yacine/mcp-logs.git
cd mcp-logs/mcp-logs

# Install dependencies
bun install

# Install globally
bun install -g .
```

## Usage

### Configuring MCP Clients

The MCP server must be configured in your MCP client before use. Below are configurations for popular clients:

#### OpenCode

Edit `~/.config/opencode/mcp.json`:

**If installed globally (recommended):**
```json
{
  "mcpServers": {
    "mcp-logs": {
      "command": "mcp-logs"
    }
  }
}
```

**If using from source:**
```json
{
  "mcpServers": {
    "mcp-logs": {
      "command": "bun",
      "args": ["run", "/absolute/path/to/mcp-log/mcp-logs/index.ts"],
      "env": {
        "VERBOSE": "false"
      }
    }
  }
}
```

#### Claude Desktop

Edit `~/Library/Application Support/Claude/claude_desktop_config.json` (macOS) or `%APPDATA%\Claude\claude_desktop_config.json` (Windows):

**If installed globally (recommended):**
```json
{
  "mcpServers": {
    "mcp-logs": {
      "command": "mcp-logs"
    }
  }
}
```

**If using from source:**
```json
{
  "mcpServers": {
    "mcp-logs": {
      "command": "bun",
      "args": ["run", "/absolute/path/to/mcp-log/mcp-logs/index.ts"],
      "env": {
        "VERBOSE": "false"
      }
    }
  }
}
```

#### Cline (VSCode Extension)

Edit VSCode settings (`settings.json`):

**If installed globally (recommended):**
```json
{
  "cline.mcpServers": {
    "mcp-logs": {
      "command": "mcp-logs"
    }
  }
}
```

**If using from source:**
```json
{
  "cline.mcpServers": {
    "mcp-logs": {
      "command": "bun",
      "args": ["run", "/absolute/path/to/mcp-log/mcp-logs/index.ts"],
      "env": {
        "VERBOSE": "false"
      }
    }
  }
}
```

> **Note:** Ensure `bun` is installed and in your PATH. Install Bun: https://bun.sh

### Standalone Server (for testing)

```bash
mcp-logs
```

The server will:
1. Start listening on Unix socket `/tmp/log-agent.sock`
2. Start the MCP server on stdio
3. Wait for log-agent CLI to send logs

### With OpenCode

Add to your OpenCode configuration (`~/.config/opencode/opencode.json`):

```json
{
  "mcp": {
    "mcp-logs": {
      "type": "local",
      "enabled": true,
      "command": ["bun", "x", "mcp-logs@latest"]
    }
  }
}
```

## MCP Tools

### 1. get_recent_logs

Get the most recent N logs.

**Parameters:**
- `count` (optional): Number of logs (default: 50, max: 500)

**Example:**
```
Show me the last 100 logs
```

### 2. get_logs

Advanced filtering with multiple criteria.

**Parameters:**
- `project` (optional): Filter by project name
- `level` (optional): `info`, `warn`, `error`, `debug`
- `source` (optional): `stdout`, `stderr`
- `search` (optional): Text search in messages
- `limit` (optional): Max results (default: 100)

**Example:**
```
Show me error logs from project "my-app"
```

### 3. search_logs

Search logs by text content.

**Parameters:**
- `query` (required): Search text
- `project` (optional): Filter by project
- `limit` (optional): Max results (default: 50)

**Example:**
```
Search for "database connection" in the logs
```

### 4. get_errors

Get only error-level logs.

**Parameters:**
- `project` (optional): Filter by project
- `limit` (optional): Max results (default: 50)

**Example:**
```
Show me all errors
```

### 5. get_stats

Get global statistics about captured logs.

**Example:**
```
Show log statistics
```

### 6. clear_logs

Clear all logs from memory.

**Example:**
```
Clear all logs
```

## Complete System

This MCP server works with the `mcp-log-agent` CLI to provide real-time log capture:

1. **Install the CLI:**
   ```bash
   cargo install mcp-log-agent
   ```

2. **Start MCP server** (via OpenCode or standalone)

3. **Capture logs:**
   ```bash
   mcp-log-agent run --project my-app bun dev
   ```

4. **Query via MCP tools** in OpenCode/Claude

## Configuration

### Configuration Files

`mcp-logs` supports flexible configuration via JSON files and environment variables.

**Configuration priority (highest to lowest):**
1. Environment variables (`MCP_LOGS_*`)
2. Local config file (`.mcp-logs.json`)
3. Global config file (`~/.config/mcp-logs/config.json`)
4. Default values

### Quick Start: Create Configuration

```bash
# Create local config (project directory) with detailed comments
bun run index.ts config init

# Create global config (user-wide)
bun run index.ts config init --global

# Create minimal config without comments
bun run index.ts config init --minimal
```

This generates a JSON configuration file with detailed inline comments for each parameter.

### Configuration Commands

```bash
# Initialize config with detailed comments
bun run index.ts config init [--global] [--minimal]

# Show current merged configuration
bun run index.ts config show

# Get specific configuration value
bun run index.ts config get <section.field>

# Set configuration value
bun run index.ts config set <section.field> <value> [--global]

# List all available configuration keys
bun run index.ts config list

# Display help
bun run index.ts config help
```

**Examples:**
```bash
# Get/set configuration values
bun run index.ts config get server.verbose
bun run index.ts config set server.verbose true
bun run index.ts config set storage.max_logs 20000
bun run index.ts config set logging.log_level debug --global
```

### Configuration File Structure

Example `.mcp-logs.json` with comments:

```json
{
  "server": {
    "socket_path": "/tmp/log-agent.sock",
    "_socket_path_comment": "Path to Unix domain socket for receiving logs from agents",
    "_socket_path_env": "MCP_LOGS_SOCKET_PATH",
    
    "name": "mcp-logs",
    "version": "0.1.1",
    "verbose": false,
    "_verbose_comment": "Enable verbose logging from the server itself",
    "_verbose_env": "MCP_LOGS_VERBOSE or VERBOSE"
  },
  
  "storage": {
    "max_logs": 10000,
    "_max_logs_comment": "Maximum number of logs to keep in memory (FIFO)",
    "_max_logs_note": "Older logs are discarded when limit is reached",
    "_max_logs_env": "MCP_LOGS_MAX_LOGS",
    
    "storage_type": "memory",
    "_storage_type_values": "memory | sqlite | postgres"
  },
  
  "logging": {
    "log_level": "info",
    "_log_level_values": "debug | info | warn | error",
    
    "log_file": null,
    "_log_file_comment": "Path to log file (null = console only)",
    
    "log_format": "text",
    "_log_format_values": "text | json"
  },
  
  "performance": {
    "buffer_size": 1000,
    "connection_timeout": 30000,
    "max_connections": 100
  },
  
  "features": {
    "auto_cleanup": true,
    "max_log_age_hours": 24,
    "enable_stats": true
  }
}
```

> **Note:** Lines starting with `_` are comments and are ignored by the loader. They provide inline documentation.

### Environment Variables

Override any config value using environment variables:

```bash
# Server settings
export MCP_LOGS_SOCKET_PATH="/custom/path.sock"
export MCP_LOGS_VERBOSE=true

# Storage settings
export MCP_LOGS_MAX_LOGS=20000

# Logging settings
export MCP_LOGS_LOG_LEVEL=debug

# Start server with env vars
mcp-logs
```

### Socket Path

Default: `/tmp/log-agent.sock`

Change via:
- Config file: `server.socket_path = "/custom/path.sock"`
- Environment: `MCP_LOGS_SOCKET_PATH=/custom/path.sock`
- Both server and agent must use the same socket path

### Log Storage Limit

Default: 10,000 logs (FIFO)

Change via:
- Config file: `storage.max_logs = 20000`
- Environment: `MCP_LOGS_MAX_LOGS=20000`

When the limit is reached, older logs are automatically discarded (First In, First Out).

## Log Protocol

Logs are received as newline-delimited JSON:

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

## Requirements

- **Bun >= 1.0.0** (required runtime)
- Unix-like system (Linux, macOS)
- mcp-log-agent CLI for capturing logs

To install Bun, visit: https://bun.sh

## Development

```bash
# Clone repository
git clone https://github.com/Yaici-Yacine/mcp-logs.git
cd mcp-logs/mcp-logs

# Install dependencies
bun install

# Run in development
bun run dev

# Build
bun build index.ts --target=bun --outdir=./dist
```

## Limitations

- In-memory storage only (no persistence)
- Unix sockets only (no Windows support)
- Maximum 10,000 logs by default (FIFO)
- One server instance per socket path

## Related Projects

- [mcp-log-agent](https://crates.io/crates/mcp-log-agent) - Rust CLI for log capture
- [MCP Protocol](https://modelcontextprotocol.io/) - Model Context Protocol spec

## Contributing

Contributions are welcome! Please submit a Pull Request.

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Author

Yacine Yaici - yaiciy01@gmail.com

## Links

- [GitHub Repository](https://github.com/Yaici-Yacine/mcp-logs)
- [Report Issues](https://github.com/Yaici-Yacine/mcp-logs/issues)
- [npm Package](https://www.npmjs.com/package/mcp-logs)

## Changelog

### 0.1.2 (2026-01-07)

- **Enhanced Config CLI**: Integrated config management into main binary
  - Merged standalone `config.ts` into `index.ts` for unified CLI
  - `bun run index.ts config <command>` - Single entry point
  - **NEW**: `config get <key>` - Get specific configuration values
  - **NEW**: `config set <key> <value>` - Modify configuration values directly
  - **NEW**: `config list` - List all available configuration keys
  - Support for `--global` flag on `set` command
  - Type validation for enums (storage_type, log_level, log_format)
- **Improved User Experience**:
  - No more separate config.ts file - everything in index.ts
  - Better help messages with examples
  - Cleaner command structure
- **Bug Fixes**: Removed duplicate config.ts file

### 0.1.1 (2026-01-06)

- **Configuration System**: Complete configuration management with JSON files
  - Global config: `~/.config/mcp-logs/config.json`
  - Local config: `.mcp-logs.json`
  - Environment variable support (`MCP_LOGS_*`)
  - Configuration priority: env vars > local > global > defaults
- **Configuration CLI**: Integrated config commands into main CLI (`index.ts config`)
  - `init` command with `--global` and `--minimal` options
  - `show` command to display merged configuration
  - **NEW**: `get <key>` command to retrieve specific values
  - **NEW**: `set <key> <value>` command to modify configuration
  - **NEW**: `list` command to show all available keys
  - Inline comments with `_comment` fields for each parameter
- **Configurable Settings**:
  - Server: socket_path, name, version, verbose
  - Storage: max_logs, storage_type (memory/sqlite/postgres prep)
  - Logging: log_level, log_file, log_format
  - Performance: buffer_size, connection_timeout, max_connections
  - Features: auto_cleanup, max_log_age_hours, enable_stats
- **CLI Modes**: Single binary supports both server mode and config management
  - `bun run index.ts` - Start MCP server (default)
  - `bun run index.ts config <command>` - Config management
- **Detailed Documentation**: Every config parameter has inline explanation
- **Version Bump**: Updated to 0.1.1 in package.json

### 0.0.1 (2025-12-24)

- Initial release
- Unix socket server for log reception
- 6 MCP tools for log querying
- Real-time log streaming
- Multi-project support
- In-memory storage with FIFO
- Bun runtime required
