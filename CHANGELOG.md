# Changelog

All notable changes to the mcp-log project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### mcp-log-agent v1.2.0 & mcp-logs v1.1.0 - 2025-01-20

#### 🔄 Major Feature: Remote Process Restart

**Added**
- **`restart_process` MCP Tool**: AI assistants can now restart monitored processes remotely
  - Works in both TUI (watch) mode and one-shot mode
  - Graceful shutdown with SIGTERM, fallback to SIGKILL
  - Real-time feedback via command response protocol
  - Project-specific targeting for multi-agent setups

**Protocol Enhancements**
- Bidirectional Unix socket communication (Agent ↔ MCP Server)
- New message types: `command` and `command_response`
- Request/response tracking with unique request IDs
- Command validation and error handling

**Agent (Rust) Changes**
- `src/types/mod.rs`: Added `CommandMessage`, `CommandResponse`, `RestartCommand` types
- `src/socket.rs`: Transformed to bidirectional communication
  - Added `start_command_listener()` for receiving commands from MCP server
  - Added `send_command_response()` for sending responses back
  - Separate connection for command listening to avoid conflicts
- `src/tui/mod.rs`: Integrated restart command handling
  - New channel for restart commands (`restart_rx`)
  - Restart branch in `tokio::select!` event loop
  - TUI messages: "🔄 Restart requested via MCP", "✓ Process restarted via MCP (PID: X)"
  - Response sent back to MCP server on success/failure

**MCP Server (TypeScript) Changes**
- `src/types/index.ts`: Added `CommandMessage`, `CommandResponse` interfaces
- `src/server/index.ts`: Enhanced SocketServer for bidirectional communication
  - Added `clients: Map<string, socket>` to track agent connections
  - Added `sendCommand()` method to send commands to specific agents
  - Modified `handleData()` to process command responses
  - Automatic cleanup of disconnected clients
- `src/mcp/tools.ts`: Added `restart_process` tool definition
- `src/mcp/handlers.ts`: Added `restartProcess()` handler with validation

**Documentation**
- Added `RESTART-FEATURE.md` - Complete restart feature documentation
  - Architecture diagrams
  - Protocol specifications
  - Usage examples and use cases
  - Testing instructions
  - Error handling guide
- Updated `README.md` with restart feature overview
- Created test scripts:
  - `test-restart-mcp.sh` - Manual testing guide
  - `send-restart-command.sh` - Command sending helper

**Use Cases Enabled**
- 🤖 AI-driven automatic restart on error detection
- ⚙️ Apply configuration changes by restarting services
- 💾 Mitigate memory leaks with scheduled restarts
- 🔄 Multi-project orchestration (restart in sequence)

**Security & Reliability**
- Unix socket remains local-only (no network exposure)
- Command validation (only whitelisted commands accepted)
- Graceful shutdown prevents data loss
- Response feedback ensures restart confirmation

---

## [1.0.0] - 2026-01-18

### mcp-logs (MCP Server)

**Major Release - Advanced Features**

#### Added
- **NEW: get_analytics Tool** - Comprehensive log analysis and insights
  - Summary statistics (total logs, time range, active projects)
  - Distribution by log level (error, warn, info, debug counts)
  - Distribution by project (logs per project)
  - Timeline analysis (grouped by minute/hour/day)
  - Top 10 most frequent messages
  - Error rate calculation (percentage of errors vs total logs)
  - Flexible time ranges: "1h", "6h", "24h", "7d", "30d"
  - Custom time ranges with startTime/endTime
  - Group by options: minute, hour, day, project, level

#### Enhanced
- **Temporal Filtering** - Filter logs by time range in get_logs tool
  - Multiple time format support:
    - ISO 8601: `"2026-01-18T10:00:00Z"`
    - Unix timestamps: `1737201600000`
    - Relative times: `"last 1h"`, `"last 30m"`, `"last 2d"`, `"last 1w"`
  - `startTime` and `endTime` parameters for custom ranges
  - Utility functions for time parsing and validation

- **Regex Search** - Advanced pattern matching in search_logs tool
  - New `regex` boolean parameter (default: false)
  - Full regex syntax support for complex queries
  - Backwards compatible (simple text search by default)

#### Infrastructure
- **Time Utilities Module** - New src/utils/time.ts
  - `parseTimeInput()` - Parse ISO 8601, timestamps, relative times
  - `isInTimeRange()` - Check if timestamp in range
  - `formatDuration()` - Human-readable duration formatting
  - `groupByTimeInterval()` - Group logs by time buckets

- **Enhanced Types** - Extended TypeScript interfaces
  - `AnalyticsOptions` - Options for analytics queries
  - `Analytics` - Complete analytics data structure
  - `LogFilter` - Added startTime/endTime/regex support

#### Documentation
- Complete get_analytics documentation with use cases
- Time format examples for temporal filtering
- Regex search patterns and examples
- Tool count updated: 6 → 8 tools

#### Testing
- Added test-features.ts for validating new functionality
- All TypeScript type checks pass, production-ready

---

## [0.1.2] - 2026-01-07

### mcp-logs (MCP Server)

#### Enhanced
- **Enhanced Config CLI**: Integrated config management into main binary
  - Merged standalone `config.ts` into `index.ts` for unified CLI
  - `bun run index.ts config <command>` - Single entry point
  - **NEW**: `config get <key>` - Get specific configuration values
  - **NEW**: `config set <key> <value>` - Modify configuration values directly
  - **NEW**: `config list` - List all available configuration keys
  - Support for `--global` flag on `set` command
  - Type validation for enums (storage_type, log_level, log_format)

#### Improved
- Better help messages with examples
- Cleaner command structure
- No more separate config.ts file - everything in index.ts

#### Fixed
- Removed duplicate config.ts file

---

## [0.1.1] - 2026-01-06

### mcp-log-agent (Rust CLI)

#### Added
- **Configuration System**: Complete configuration management with TOML files
  - Global config: `~/.config/mcp-log-agent/config.toml`
  - Local config: `.mcp-log-agent.toml`
  - Environment variable support (`MCP_LOG_*`)
  - Configuration priority: CLI args > env vars > local > global > defaults

- **Configuration CLI Commands**
  - `config init` - Initialize config with detailed inline comments
  - `config show` - Display current merged configuration
  - `config get <key>` - Retrieve specific values
  - `config set <key> <value>` - Modify configuration values
  - `config list` - Show all available configuration keys
  - `config validate` - Validate configuration files
  - `config detect` - Detect config sources

- **Color Themes System**
  - `config theme list` - List available color themes
  - `config theme set <name>` - Apply a color theme
  - `config theme create <name>` - Create custom theme
  - `config theme export <name>` - Export current colors as theme
  - `config theme preview <name>` - Preview theme colors
  - Built-in themes: default, solarized-dark, high-contrast, minimal, monochrome

#### Configurable Settings
- **Agent**: socket_path, default_project, default_command, verbose, connection_timeout
- **Output**: colors, format (colored/plain/json), show_timestamps, show_pid
- **Colors**: Customizable colors for all log levels and UI elements
- **Filters**: min_level, ignored_patterns, highlight_patterns
- **Performance**: buffer_size, flush_interval_ms

#### Enhanced
- Detailed inline comments in generated config files
- Environment variable documentation for each setting
- `.gitignore` integration for local configs in git repos

### mcp-logs (MCP Server)

#### Added
- **Configuration System**: JSON-based configuration with inline comments
  - Global config: `~/.config/mcp-logs/config.json`
  - Local config: `.mcp-logs.json`
  - Environment variable support (`MCP_LOGS_*`)
  - Configuration priority: env vars > local > global > defaults

- **Configuration CLI Commands**
  - `config init [--global] [--minimal]` - Initialize configuration
  - `config show` - Display merged configuration
  - `config get <key>` - Get specific value
  - `config set <key> <value> [--global]` - Set configuration value
  - `config list` - List all available keys
  - `config help` - Show help

#### Configurable Settings
- **Server**: socket_path, name, version, verbose
- **Storage**: max_logs, storage_type (memory/sqlite/postgres prep)
- **Logging**: log_level, log_file, log_format
- **Performance**: buffer_size, connection_timeout, max_connections
- **Features**: auto_cleanup, max_log_age_hours, enable_stats

---

## [0.0.1] - 2025-12-24

### Initial Release

#### Features
- Unix socket server for log reception
- 6 MCP tools for log querying:
  - `get_recent_logs` - Get most recent logs
  - `get_logs` - Advanced filtering
  - `search_logs` - Text search
  - `get_errors` - Error logs only
  - `get_stats` - Statistics
  - `list_projects` - Connected agents
- Real-time log streaming
- Multi-project support
- In-memory storage with FIFO (10,000 logs max)
- Automatic log level inference
- JSON-based protocol
- Rust CLI agent for log capture
- Bun/TypeScript MCP server
- Colored terminal output

---

## Version Summary

| Version | Agent | Server | Date | Highlights |
|---------|-------|--------|------|------------|
| **1.2.0/1.1.0** | ✅ | ✅ | 2025-01-20 | 🔄 Remote restart, bidirectional communication |
| **1.0.0** | ❌ | ✅ | 2026-01-18 | 📊 Analytics, temporal filtering, regex search |
| **0.1.2** | ❌ | ✅ | 2026-01-07 | ⚙️ Enhanced config CLI |
| **0.1.1** | ✅ | ✅ | 2026-01-06 | 🎨 Configuration system, themes |
| **0.0.1** | ✅ | ✅ | 2025-12-24 | 🎉 Initial release |

---

[Unreleased]: https://github.com/Yaici-Yacine/mcp-logs/compare/v1.2.0...HEAD
[1.2.0]: https://github.com/Yaici-Yacine/mcp-logs/compare/v1.0.0...v1.2.0
[1.0.0]: https://github.com/Yaici-Yacine/mcp-logs/compare/v0.1.2...v1.0.0
[0.1.2]: https://github.com/Yaici-Yacine/mcp-logs/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/Yaici-Yacine/mcp-logs/compare/v0.0.1...v0.1.1
[0.0.1]: https://github.com/Yaici-Yacine/mcp-logs/releases/tag/v0.0.1
