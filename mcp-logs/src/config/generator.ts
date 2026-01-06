/**
 * Générateur de fichier de configuration avec commentaires détaillés
 */

export function generateConfigWithComments(): string {
  return `{
  "_comment": "MCP Logs Server Configuration File",
  "_priority": "Configuration priority: Environment variables > Local config > Global config > Defaults",

  "server": {
    "_comment": "Server connection and identification settings",
    
    "socket_path": "/tmp/log-agent.sock",
    "_socket_path_comment": "Path to Unix domain socket for receiving logs from agents",
    "_socket_path_default": "/tmp/log-agent.sock",
    "_socket_path_env": "MCP_LOGS_SOCKET_PATH",
    
    "name": "mcp-logs",
    "_name_comment": "Server name identifier for MCP protocol",
    "_name_default": "mcp-logs",
    
    "version": "0.1.1",
    "_version_comment": "Server version number",
    
    "verbose": false,
    "_verbose_comment": "Enable verbose logging from the server itself",
    "_verbose_values": "true | false",
    "_verbose_default": false,
    "_verbose_env": "MCP_LOGS_VERBOSE or VERBOSE"
  },

  "storage": {
    "_comment": "Log storage and retention settings",
    
    "max_logs": 10000,
    "_max_logs_comment": "Maximum number of logs to keep in memory (FIFO)",
    "_max_logs_default": 10000,
    "_max_logs_note": "Older logs are discarded when limit is reached",
    "_max_logs_env": "MCP_LOGS_MAX_LOGS",
    
    "storage_type": "memory",
    "_storage_type_comment": "Storage backend type",
    "_storage_type_values": "memory | sqlite | postgres",
    "_storage_type_default": "memory",
    "_storage_type_note": "Only 'memory' is currently implemented",
    
    "sqlite_path": "./mcp-logs.db",
    "_sqlite_path_comment": "Path to SQLite database file (when storage_type='sqlite')",
    "_sqlite_path_default": "./mcp-logs.db",
    
    "postgres_url": "postgresql://localhost/mcp_logs",
    "_postgres_url_comment": "PostgreSQL connection URL (when storage_type='postgres')",
    "_postgres_url_format": "postgresql://user:pass@host:port/database"
  },

  "logging": {
    "_comment": "Server logging configuration (not captured logs)",
    
    "log_level": "info",
    "_log_level_comment": "Logging level for server's own logs",
    "_log_level_values": "debug | info | warn | error",
    "_log_level_default": "info",
    "_log_level_env": "MCP_LOGS_LOG_LEVEL",
    
    "log_file": null,
    "_log_file_comment": "Path to log file (null = console only)",
    "_log_file_default": null,
    "_log_file_example": "./logs/mcp-logs.log",
    
    "log_format": "text",
    "_log_format_comment": "Log output format",
    "_log_format_values": "text | json",
    "_log_format_default": "text"
  },

  "performance": {
    "_comment": "Performance and resource limits",
    
    "buffer_size": 1000,
    "_buffer_size_comment": "Internal buffer size for incoming log messages",
    "_buffer_size_default": 1000,
    "_buffer_size_note": "Higher values handle bursts better but use more memory",
    
    "connection_timeout": 30000,
    "_connection_timeout_comment": "Socket connection timeout in milliseconds",
    "_connection_timeout_default": 30000,
    
    "max_connections": 100,
    "_max_connections_comment": "Maximum concurrent agent connections",
    "_max_connections_default": 100
  },

  "features": {
    "_comment": "Optional feature toggles",
    
    "auto_cleanup": true,
    "_auto_cleanup_comment": "Automatically remove old logs based on max_log_age_hours",
    "_auto_cleanup_default": true,
    
    "max_log_age_hours": 24,
    "_max_log_age_hours_comment": "Maximum age of logs in hours before cleanup (when auto_cleanup=true)",
    "_max_log_age_hours_default": 24,
    "_max_log_age_hours_note": "Set to 0 to disable age-based cleanup",
    
    "enable_stats": true,
    "_enable_stats_comment": "Enable statistics collection for monitoring",
    "_enable_stats_default": true
  },

  "_notes": [
    "Lines starting with '_' are comments and will be ignored by the loader",
    "Environment variables take precedence over config file values",
    "Use MCP_LOGS_* prefix for environment variables",
    "Local config: .mcp-logs.json (project directory)",
    "Global config: ~/.config/mcp-logs/config.json"
  ]
}
`;
}

/**
 * Génère une configuration minimale sans commentaires
 */
export function generateMinimalConfig(): string {
  return `{
  "server": {
    "socket_path": "/tmp/log-agent.sock",
    "name": "mcp-logs",
    "version": "0.1.1",
    "verbose": false
  },
  "storage": {
    "max_logs": 10000,
    "storage_type": "memory"
  },
  "logging": {
    "log_level": "info",
    "log_file": null,
    "log_format": "text"
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
`;
}
