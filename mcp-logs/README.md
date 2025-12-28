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

### Socket Path

Default: `/tmp/log-agent.sock`

To change, modify `SOCKET_PATH` in `src/server/index.ts`.

### Log Storage Limit

Default: 10,000 logs (FIFO)

To change, modify the LogStore initialization in `index.ts`:
```typescript
const logStore = new LogStore(20000); // Change limit
```

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

### 0.0.1 (2025-12-24)

- Initial release
- Unix socket server for log reception
- 6 MCP tools for log querying
- Real-time log streaming
- Multi-project support
- In-memory storage with FIFO
- Bun runtime required
