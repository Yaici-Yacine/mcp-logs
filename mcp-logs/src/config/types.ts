/**
 * Configuration types for mcp-logs server
 */

export interface Config {
  server: ServerConfig;
  storage: StorageConfig;
  logging: LoggingConfig;
  performance: PerformanceConfig;
  features: FeaturesConfig;
}

export interface ServerConfig {
  socket_path: string;
  verbose: boolean;
  name: string;
  version: string;
}

export interface StorageConfig {
  max_logs: number;
  storage_type: "memory" | "sqlite" | "postgresql";
  sqlite_path?: string;
  postgres_url?: string;
}

export interface LoggingConfig {
  log_level: "debug" | "info" | "warn" | "error";
  log_file: string | null;
  log_format: "text" | "json";
}

export interface PerformanceConfig {
  buffer_size: number;
  connection_timeout: number;
  max_connections: number;
}

export interface FeaturesConfig {
  auto_cleanup: boolean;
  max_log_age_hours: number;
  enable_stats: boolean;
  compress_logs?: boolean;
}

/**
 * Default configuration values
 */
export const DEFAULT_CONFIG: Config = {
  server: {
    socket_path: process.env.MCP_LOGS_SOCKET_PATH || '/tmp/log-agent.sock',
    verbose: process.env.MCP_LOGS_VERBOSE === 'true' || process.env.VERBOSE === 'true',
    name: 'mcp-logs',
    version: '0.1.1',
  },
  storage: {
    max_logs: 10000,
    storage_type: "memory",
  },
  logging: {
    log_level: "info",
    log_file: null,
    log_format: "text",
  },
  performance: {
    buffer_size: 65536,
    connection_timeout: 300,
    max_connections: 100,
  },
  features: {
    auto_cleanup: false,
    max_log_age_hours: 24,
    enable_stats: true,
  },
};
