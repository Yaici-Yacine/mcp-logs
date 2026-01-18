# mcp-log-agent

[![Crates.io](https://img.shields.io/crates/v/mcp-log-agent.svg)](https://crates.io/crates/mcp-log-agent)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Real-time log capture CLI for development projects with MCP (Model Context Protocol) integration.

## Features

- Capture stdout/stderr from any process in real-time
- **Interactive TUI (Terminal User Interface)** with watch mode
  - Real-time scrollable log viewer
  - Mouse support (scroll, click to select)
  - Process control (restart, quit, clear logs)
  - Auto-countdown on process exit
  - Performance optimized (frame rate limiting)
- Stream logs to MCP server via Unix socket
- JSON-based structured logging
- Automatic log level inference (info, warn, error, debug)
- Support for multiple simultaneous projects
- Zero file I/O - all logs in memory

## Installation

### From crates.io (recommended)

```bash
cargo install mcp-log-agent
```

The binary will be installed in `~/.cargo/bin/` (ensure this is in your `$PATH`).

### From Source

```bash
git clone https://github.com/Yaici-Yacine/mcp-logs.git
cd mcp-logs/log-agent

# Install globally
cargo install --path .

# Or build without installing
cargo build --release
# Binary will be in ./target/release/mcp-log-agent
```

## Quick Start

### 1. Install dependencies

```bash
# Install the log agent
cargo install mcp-log-agent

# Install the MCP server
npm install -g mcp-logs
# or: bun install -g mcp-logs
```

### 2. Configure your MCP client

Add to your MCP client configuration (OpenCode, Claude Desktop, Cline, etc.):

```json
{
  "mcpServers": {
    "mcp-logs": {
      "command": "mcp-logs"
    }
  }
}
```

**Configuration file locations:**
- **OpenCode:** `~/.config/opencode/mcp.json`
- **Claude Desktop (macOS):** `~/Library/Application Support/Claude/claude_desktop_config.json`
- **Claude Desktop (Windows):** `%APPDATA%\Claude\claude_desktop_config.json`
- **Cline (VSCode):** VSCode `settings.json` under `cline.mcpServers`

### 3. Restart your MCP client

Restart your client (OpenCode, Claude Desktop, Cline) to load the MCP server.

### 4. Set up your project (Recommended)

Create a local configuration file in your project directory:

```bash
cd your-project
mcp-log-agent config init --local
```

Edit `.mcp-log-agent.toml` and set your default command:

```toml
[agent]
default_project = "my-app"
# Uncomment and set your command:
default_command = ["npm", "start"]
# Or: default_command = ["bun", "dev"]
# Or: default_command = ["cargo", "run"]
```

### 5. Capture logs from your project

**With local config (simple):**
```bash
# Just run without arguments!
mcp-log-agent run
```

**Without config (specify command):**
```bash
# Capture from any command
mcp-log-agent run --project my-app -- bun dev

# Node.js project
mcp-log-agent run --project api -- npm start

# Rust project
mcp-log-agent run --project backend -- cargo run

# Python project
mcp-log-agent run --project ml-script -- python train.py
```

> **Note:** The `--` separator is needed when providing a command to separate mcp-log-agent options from your command arguments.

Logs will be displayed in your terminal (colorized) AND captured by the MCP server.

### 6. Query logs via your MCP client

In your MCP client (OpenCode, Claude Desktop, Cline), ask questions like:

```
Show me the recent logs
What errors occurred in my-app?
Search for "database" in the logs
List all connected projects
```

The client will automatically call the appropriate MCP tools (`get_recent_logs`, `get_errors`, `search_logs`, `list_projects`, etc.).

## Usage

### Run Command

Spawn a process and capture its logs:

```bash
# With local config containing default_command
mcp-log-agent run

# With specific command
mcp-log-agent run [OPTIONS] -- <COMMAND> [ARGS...]
```

**Options:**
- `--project, -p`: Project name for identification (overrides config)
- `--verbose, -v`: Enable verbose output
- `--watch, -w`: Enable interactive TUI (Terminal User Interface) mode
- `--cmd, -C`: Use a predefined command from config
- Command and arguments: The command to run (uses `default_command` from config if not provided)

#### Predefined Commands

Define multiple commands in your config for quick access:

```toml
[agent]
# Simple syntax (uses default watch setting)
[agent.commands]
dev = ["npm", "run", "dev"]
build = ["npm", "run", "build"]

# Detailed syntax (specify watch mode per command)
test = { command = ["npm", "test"], watch = true }
serve = { command = ["python", "-m", "http.server"], watch = false }
```

**Usage:**

```bash
# Run predefined commands
mcp-log-agent run --cmd dev      # Uses dev command
mcp-log-agent run --cmd test     # Uses test command with watch mode enabled
mcp-log-agent run -C build       # Short flag works too

# List available commands when none found
mcp-log-agent run --cmd unknown
# Error: Predefined command 'unknown' not found in config
# Available commands in config:
#   dev = ["npm", "run", "dev"]
#   test = ["npm", "test"] (watch: true)
#   build = ["npm", "run", "build"]
```

**Watch Mode Priority:**

When using predefined commands, the watch mode is determined by:
1. CLI flag (`--watch` / `-w`) - highest priority
2. Command-specific `watch` setting - if using detailed syntax
3. Global `watch` setting in `[agent]` - default for simple syntax
4. Default value (`false`)

**Examples:**

```bash
# Command uses its own watch setting
mcp-log-agent run --cmd test      # watch = true (from command config)

# Override with CLI flag
mcp-log-agent run --cmd test -w   # Always uses watch mode
mcp-log-agent run --cmd dev       # Uses global watch setting

# Mix predefined and inline commands
mcp-log-agent run --cmd dev       # Uses config command
mcp-log-agent run -- npm start    # Uses inline command
```

#### Watch Mode (TUI)

Watch mode provides an interactive terminal interface for monitoring and controlling your process:

```bash
# Enable TUI via CLI flag
mcp-log-agent run --watch -- npm start
mcp-log-agent run -w -- bun dev

# Or enable in config file
# In .mcp-log-agent.toml:
# [agent]
# watch = true
mcp-log-agent run  # Auto-launches in TUI mode
```

**TUI Features:**
- Real-time scrollable log viewer with color-coded log levels
- Mouse support: scroll with wheel, click to select lines
- Keyboard controls:
  - **Navigation:**
    - `↑/↓` or `j/k` - Scroll up/down
    - `Page Up/Down` - Fast scroll
    - `Home/End` - Jump to top/bottom
  - **Process Control:**
    - `r` - Restart the process (without quitting the agent)
    - `q` - Quit
  - **Log Management:**
    - `c` - Clear all logs
    - `p` / `Space` - Pause/Resume log capture
    - `/` - Search logs (supports regex)
    - `s` - Save logs to file
    - `y` - Copy selected line to clipboard
    - `?` - Show help overlay with all shortcuts
- **Search & Filter:** Regex-based search with live highlighting (matching logs highlighted, others dimmed)
- **Pause/Resume:** Freeze log capture to read, resume when ready (buffered logs are retained)
- **Save to File:** Export current logs to a text file
- **Copy to Clipboard:** Copy any selected log line
- **Network Stats:** Real-time display of logs received/sent and rate per second
- Auto-countdown: When process exits, shows 5-second countdown before auto-quit
  - Press `r` to restart immediately
  - Press `q` to quit immediately
- Performance optimized: Frame rate limiting prevents lag with high-frequency logs

**TUI Configuration:**

Add to your `.mcp-log-agent.toml`:

```toml
[agent]
watch = true  # Enable TUI mode by default

[performance.tui]
max_logs = 5000          # Max logs kept in memory (default: 5000)
tick_rate_ms = 250       # Countdown refresh rate (default: 250ms)
frame_rate_ms = 100      # Max 10 FPS, prevents lag (default: 100ms)
```

**Examples:**

```bash
# Simple usage with config
cd my-project
mcp-log-agent config init --local
# Edit .mcp-log-agent.toml: default_command = ["npm", "start"]
mcp-log-agent run

# Watch mode for development server
mcp-log-agent run -w -- bun dev

# Watch mode with custom project name
mcp-log-agent run --watch --project frontend -- npm start

# Web server (normal mode)
mcp-log-agent run --project frontend -- bun dev

# Build process
mcp-log-agent run --project build -- npm run build

# Tests with TUI
mcp-log-agent run -w --project tests -- cargo test

# Shell script
mcp-log-agent run --project demo -- bash ./script.sh

# Override project name from config
mcp-log-agent run --project custom-name -- npm start
```

### Test Command

Test the connection to the MCP server:

```bash
mcp-log-agent test [--message <TEXT>]
```

**Options:**
- `--message, -m`: Custom test message (optional)

**Example:**
```bash
mcp-log-agent test --message "Hello from CLI"
```

> **Note:** The MCP server must be running (via your MCP client) for the test to succeed.

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

### Configuration Files

`mcp-log-agent` supports flexible configuration via files, environment variables, and CLI arguments.

**Configuration priority (highest to lowest):**
1. CLI arguments
2. Environment variables (`MCP_LOG_*`)
3. Local config file (`.mcp-log-agent.toml`)
4. Global config file (`~/.config/mcp-log-agent/config.toml`)
5. Default values

### Quick Start: Create Configuration

```bash
# Create local config (project directory)
mcp-log-agent config init --local

# Create global config (user-wide)
mcp-log-agent config init --global
```

This generates a fully commented configuration file with explanations for each parameter.

**Pro tip:** Set `default_command` in your local config to avoid typing the command every time:

```toml
[agent]
default_project = "my-awesome-app"
default_command = ["npm", "run", "dev"]
# Now just run: mcp-log-agent run

# Or define multiple commands for quick access:
[agent.commands]
dev = ["npm", "run", "dev"]
test = { command = ["npm", "test"], watch = true }
build = ["npm", "run", "build"]
# Now run: mcp-log-agent run --cmd dev
```

### Configuration Commands

```bash
# Initialize config with detailed comments
mcp-log-agent config init [--global|--local]

# Show current merged configuration
mcp-log-agent config show [--json]

# Get specific config value
mcp-log-agent config get <key>

# Set a config value
mcp-log-agent config set <key> <value> [--global]

# List all available config keys
mcp-log-agent config list

# Validate configuration
mcp-log-agent config validate

# Detect which config files are loaded
mcp-log-agent config detect

# Reset to defaults
mcp-log-agent config reset [--global|--local]
```

### Theme Management

Customize colors for both CLI output and TUI interface using themes. Themes are stored as TOML files in `~/.config/mcp-log-agent/themes/`.

#### Using Themes

Set a theme in your configuration file:

```toml
# In .mcp-log-agent.toml or ~/.config/mcp-log-agent/config.toml
theme = "dracula"
```

Or use the config command:

```bash
# Set theme in local config
mcp-log-agent config set theme dracula

# Set theme in global config
mcp-log-agent config set theme nord --global
```

#### Built-in Themes

The following themes are automatically created on first run:

- `default` - Standard vibrant colors with blue/cyan accents
- `dracula` - Popular dark theme with purple accents (#282A36 background)
- `nord` - Arctic, north-bluish color palette (#2E3440 background)
- `monokai` - Monokai Pro inspired scheme (#272822 background)
- `solarized-dark` - Solarized Dark color scheme (#002B36 background)
- `minimal` - Minimal monochrome theme (black/white/gray)

#### Creating Custom Themes

1. Navigate to the themes directory:
   ```bash
   cd ~/.config/mcp-log-agent/themes/
   ```

2. Copy an existing theme as a template:
   ```bash
   cp default.toml my-theme.toml
   ```

3. Edit your theme file:
   ```toml
   name = "my-theme"
   description = "My custom color theme"
   author = "Your Name"
   
   # Log level colors (CLI output)
   [colors.error]
   fg = "#FF5555"  # Hex color
   style = ["bold"]
   
   [colors.warn]
   fg = "yellow"   # Named color
   style = []
   
   [colors.info]
   fg = "88C0D0"   # Hex without #
   style = []
   
   [colors.debug]
   fg = "blue"
   style = []
   
   # System messages
   [colors.system.success]
   fg = "green"
   style = ["bold"]
   
   [colors.system.error]
   fg = "red"
   style = ["bold"]
   
   [colors.system.info]
   fg = "cyan"
   style = []
   
   [colors.system.dim]
   fg = "bright_black"
   style = ["dimmed"]
   
   # TUI interface colors
   [tui]
   header_bg = "#2E3440"
   header_fg = "#ECEFF4"
   status_bg = "#2E3440"
   status_fg = "#A3BE8C"
   border = "#88C0D0"
   selected_bg = "#3B4252"
   selected_fg = "#ECEFF4"
   search_match = "#EBCB8B"
   search_dimmed = "#4C566A"
   help_bg = "#2E3440"
   help_fg = "#ECEFF4"
   ```

4. Use your custom theme:
   ```bash
   mcp-log-agent config set theme my-theme
   ```

#### Supported Color Formats

Themes support three color formats:

1. **Hex colors**: `"#FF5733"` or `"FF5733"` - Full RGB control
2. **Named colors**: `"red"`, `"bright_cyan"`, `"blue"` - Standard terminal colors
3. **RGB tuples**: `"255,87,51"` - Direct RGB values

**Available named colors:**
- Basic: `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`
- Bright: `bright_black`, `bright_red`, `bright_green`, `bright_yellow`, `bright_blue`, `bright_magenta`, `bright_cyan`, `bright_white`

**Available styles:**
- `bold`, `dimmed`, `italic`, `underline`, `blink`, `reverse`, `strikethrough`

#### Theme Structure

A complete theme file includes:

- **Metadata**: `name`, `description`, `author` (optional)
- **Log colors** (`colors.*`): Colors for error, warn, info, debug logs
- **System colors** (`colors.system.*`): Colors for agent messages (success, error, info, dim)
- **TUI colors** (`tui.*`): Colors for the interactive Terminal UI
  - `header_bg/fg`: Top bar (project name, command)
  - `status_bg/fg`: Bottom bar (stats, status)
  - `border`: All borders and frames
  - `selected_bg/fg`: Selected log line highlight
  - `search_match`: Highlighted search results
  - `search_dimmed`: Dimmed/filtered out logs
  - `help_bg/fg`: Help overlay popup

### Setting Configuration Values

You can modify configuration values directly using the `config set` command:

```bash
# Set a boolean value
mcp-log-agent config set agent.verbose true

# Set an integer value
mcp-log-agent config set agent.connection_timeout 10

# Set a string value
mcp-log-agent config set agent.socket_path "/tmp/my-socket.sock"

# Set an enum value
mcp-log-agent config set output.format plain
mcp-log-agent config set filters.min_level warn

# Set an array value (JSON format)
mcp-log-agent config set agent.default_command '["npm", "run", "dev"]'

# Set an array value (comma-separated)
mcp-log-agent config set filters.ignore_patterns "node_modules,webpack,DEBUG:"

# Modify global config instead of local
mcp-log-agent config set agent.verbose true --global
```

**Key format:** `section.field` (e.g., `agent.socket_path`, `output.colors`, `filters.min_level`)

**Supported types:**
- Booleans: `true`, `false`
- Integers: `5`, `1000`, `100`
- Strings: `"value"` or `value`
- Enums: `colored`, `plain`, `json`, `debug`, `info`, `warn`, `error`
- Arrays: `["item1", "item2"]` (JSON) or `item1,item2` (comma-separated)

### Configuration File Structure

Example `.mcp-log-agent.toml`:

```toml
[agent]
socket_path = "/tmp/log-agent.sock"
default_project = "my-app"

# Set default command to run with just "mcp-log-agent run"
default_command = ["npm", "start"]
# Or: default_command = ["bun", "dev"]
# Or: default_command = ["cargo", "run", "--release"]

watch = false                    # Enable TUI mode by default
verbose = false
connection_timeout = 5
retry_attempts = 3

# Predefined commands for quick execution
[agent.commands]
dev = ["npm", "run", "dev"]                          # Simple syntax
test = { command = ["npm", "test"], watch = true }   # Detailed syntax
build = ["npm", "run", "build"]
serve = { command = ["python", "-m", "http.server"], watch = true }

[output]
colors = true                    # Enable/disable colors
format = "colored"               # colored | plain | json
show_timestamps = false
show_pid = false

# Theme configuration (colors are loaded from theme file)
theme = "default"                # default | dracula | nord | monokai | solarized-dark | minimal

[filters]
ignore_patterns = []             # Regex patterns to exclude
min_level = "debug"              # debug | info | warn | error

[performance]
buffer_size = 1000
flush_interval = 100

[performance.tui]
max_logs = 5000                  # Max logs in TUI memory (default: 5000)
tick_rate_ms = 250               # Countdown refresh rate (default: 250ms)
frame_rate_ms = 100              # Max 10 FPS, prevents lag (default: 100ms)
```

### Environment Variables

Override any config value using environment variables:

```bash
# Agent settings
export MCP_LOG_AGENT_SOCKET_PATH="/custom/path.sock"
export MCP_LOG_AGENT_DEFAULT_PROJECT="my-project"
export MCP_LOG_AGENT_VERBOSE=true
export MCP_LOG_AGENT_CONNECTION_TIMEOUT=10
export MCP_LOG_AGENT_RETRY_ATTEMPTS=5

# Output settings
export MCP_LOG_AGENT_COLORS=false
export MCP_LOG_AGENT_FORMAT=json
export MCP_LOG_AGENT_SHOW_TIMESTAMPS=true
export MCP_LOG_AGENT_SHOW_PID=true

# Color customization
export MCP_LOG_COLOR_ERROR_FG=bright_red
export MCP_LOG_COLOR_WARN_FG=bright_yellow
export MCP_LOG_COLOR_INFO_FG=cyan
export MCP_LOG_COLOR_DEBUG_FG=bright_blue

# Filter settings
export MCP_LOG_FILTER_MIN_LEVEL=warn

# Performance settings
export MCP_LOG_AGENT_BUFFER_SIZE=2000
export MCP_LOG_AGENT_FLUSH_INTERVAL=50

# Run with env vars
mcp-log-agent run -- npm start
```

**Configuration priority** (highest to lowest):
1. Environment variables (`MCP_LOG_*`)
2. Local config file (`.mcp-log-agent.toml`)
3. Global config file (`~/.config/mcp-log-agent/config.toml`)
4. Default values

### Socket Path

Default: `/tmp/log-agent.sock`

Change via:
- Config file: `agent.socket_path = "/custom/path.sock"`
- Environment: `MCP_LOG_AGENT_SOCKET_PATH=/custom/path.sock`
- Both server and agent must use the same socket path

## Integration with MCP

This CLI works in tandem with the MCP server to provide real-time log analysis capabilities:

1. **Configure MCP server in your client** - The server is automatically started by your MCP client (OpenCode, Claude Desktop, Cline)
2. **Run mcp-log-agent** - Captures and streams logs from your projects to the MCP server
3. **Query via natural language** - Ask questions in your MCP client to search, filter, and analyze logs

### Available MCP Tools

Once configured, your MCP client will have access to 7 tools:

| Tool | Description |
|------|-------------|
| `get_recent_logs` | Get the most recent logs |
| `get_logs` | Advanced filtering (project, level, source, text search) |
| `search_logs` | Text search across all logs |
| `get_errors` | Get only error-level logs |
| `get_stats` | Statistics about captured logs |
| `list_projects` | List all connected log agents |
| `clear_logs` | Clear all logs from memory |

See the [mcp-logs documentation](https://github.com/Yaici-Yacine/mcp-logs) for detailed MCP server setup and configuration.


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

### 0.2.0 (2026-01-17)

- **Interactive TUI (Terminal User Interface)**: NEW watch mode for interactive process monitoring
  - Enable with `--watch` / `-w` flag or `watch = true` in config
  - Real-time scrollable log viewer with color-coded log levels
  - Mouse support: scroll wheel, click to select lines
  - Keyboard controls: ↑/↓, Page Up/Down, Home/End, j/k navigation
  - Process control: `r` to restart, `c` to clear logs, `q` to quit
  - Auto-countdown: 5-second countdown when process exits (configurable)
  - Performance optimized: Frame rate limiting prevents lag with high-frequency logs
- **Search & Filter**: NEW regex-based log search
  - Press `/` to enter search mode
  - Live highlighting: matching logs highlighted, others dimmed
  - Shows match count in real-time
  - Supports full regex syntax
- **Pause/Resume**: NEW log capture control
  - Press `p` or `Space` to pause/resume
  - Paused logs are buffered and shown when resumed
  - Visual indicator shows LIVE/PAUSED status
- **Save to File**: NEW export logs feature
  - Press `s` to save current logs to file
  - Custom filename with auto-suggestion
  - Saves all visible logs with timestamps and levels
- **Copy to Clipboard**: NEW clipboard integration
  - Click to select any log line
  - Press `y` to copy selected line
  - Works across all platforms (X11, Wayland, macOS, Windows)
- **Network Stats**: NEW real-time statistics
  - Logs received/sent counters
  - Logs per second rate
  - Visual display in status bar
- **Help Overlay**: NEW interactive help
  - Press `?` to show full keyboard shortcuts
  - Comprehensive guide with all features
  - Press any key to close
- **Process Supervision**: NEW supervisor module for process lifecycle management
  - Start/stop/restart processes without quitting agent
  - Clean task cleanup and state management
  - Graceful shutdown handling
- **TUI Configuration**: New `[performance.tui]` section in config
  - `max_logs` - Max logs kept in TUI memory (default: 5000)
  - `tick_rate_ms` - Countdown refresh rate (default: 250ms)
  - `frame_rate_ms` - Max 10 FPS, prevents lag (default: 100ms)
- **Modular UI Architecture**: Refactored UI code into reusable components
  - `ShortcutList` and `StatusInfoList` builders for DRY code
  - Separate modules for header, logs, status, help, and widgets
  - Eliminated code duplication
- **Dependencies**: Added `ratatui` 0.29, `crossterm` 0.29, `regex` 1.11, and `arboard` 3.4
- **Bug Fixes**: 
  - Fixed countdown display showing "0s" before quitting (was skipping from 1s)
  - Added comprehensive documentation to supervisor module
  - Fixed Event::Resize variant to include dimensions

### 0.1.1 (2026-01-06)

- **Configuration System**: Complete configuration management with TOML files
  - Global config: `~/.config/mcp-log-agent/config.toml`
  - Local config: `.mcp-log-agent.toml`
  - Environment variable support (`MCP_LOG_*`)
  - Configuration priority: CLI args > env vars > local > global > defaults
  - **Field-by-field config merging**: Local configs only override specified fields, preserving unset values from global/defaults
- **Default Command**: NEW `default_command` setting
  - Set once in config, then just run `mcp-log-agent run` without arguments
  - Example: `default_command = ["npm", "start"]`
  - Simplifies workflow: no need to type the full command every time
- **Color Customization**: 
  - 5 predefined color schemes (default, solarized-dark, high-contrast, minimal, monochrome)
  - Custom color configuration per log level
  - Style options: bold, italic, underline, dimmed, etc.
  - Environment variable support for colors: `MCP_LOG_COLOR_ERROR_FG`, `MCP_LOG_COLOR_WARN_FG`, etc.
- **Config Commands**: 12 new commands for configuration management
  - `config init`, `show`, `get`, `list`, `validate`, `detect`, `reset`
  - **`config set`**: Modify config values directly from CLI
    - Supports all types: boolean, integer, string, enum, array
    - Example: `mcp-log-agent config set agent.verbose true`
    - Example: `mcp-log-agent config set agent.default_command '["npm", "run", "dev"]'`
  - `config colors list/set/preview/test`
- **Environment Variables**: Extended support
  - All agent, output, performance settings
  - Color foreground customization (error, warn, info, debug)
  - Filter min_level configuration
  - Complete list in documentation
- **Detailed Documentation**: Every config parameter has inline comments with:
  - Description of what it does
  - Default value
  - Possible values
  - Corresponding environment variable
- **Bug Fixes**: Improved error handling and connection retry logic

### 0.1.0 (2025-12-24)

- Initial release
- Real-time log capture from any process
- Unix socket streaming to MCP server
- Automatic log level inference
- Multi-project support
- Test command for connection verification
