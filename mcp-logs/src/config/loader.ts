import { homedir } from "os";
import { join } from "path";
import { existsSync, readFileSync, writeFileSync, mkdirSync } from "fs";
import type { Config } from "./types.js";
import { DEFAULT_CONFIG } from "./types.js";

/**
 * Load configuration from all sources and merge them
 */
export function loadConfig(): Config {
  let config = { ...DEFAULT_CONFIG };

  // 1. Load global config if exists
  const globalPath = getGlobalConfigPath();
  if (globalPath && existsSync(globalPath)) {
    const globalConfig = loadFromFile(globalPath);
    config = mergeConfigs(config, globalConfig);
  }

  // 2. Load local config if exists
  const localPath = getLocalConfigPath();
  if (existsSync(localPath)) {
    const localConfig = loadFromFile(localPath);
    config = mergeConfigs(config, localConfig);
  }

  // 3. Apply environment variables
  config = applyEnvVars(config);

  return config;
}

/**
 * Load config from a JSON file
 */
function loadFromFile(path: string): Partial<Config> {
  try {
    const contents = readFileSync(path, "utf-8");
    return JSON.parse(contents);
  } catch (error) {
    console.error(`Failed to load config from ${path}:`, error);
    return {};
  }
}

/**
 * Merge two configs (second overrides first)
 */
function mergeConfigs(base: Config, override: Partial<Config>): Config {
  return {
    server: { ...base.server, ...override.server },
    storage: { ...base.storage, ...override.storage },
    logging: { ...base.logging, ...override.logging },
    performance: { ...base.performance, ...override.performance },
    features: { ...base.features, ...override.features },
  };
}

/**
 * Apply environment variables to config
 */
function applyEnvVars(config: Config): Config {
  const env = process.env;

  // Server
  if (env.MCP_LOGS_SOCKET_PATH) {
    config.server.socket_path = env.MCP_LOGS_SOCKET_PATH;
  }
  if (env.VERBOSE !== undefined || env.MCP_LOGS_VERBOSE !== undefined) {
    config.server.verbose = (env.VERBOSE || env.MCP_LOGS_VERBOSE) === "true";
  }
  if (env.MCP_LOGS_NAME) {
    config.server.name = env.MCP_LOGS_NAME;
  }
  if (env.MCP_LOGS_VERSION) {
    config.server.version = env.MCP_LOGS_VERSION;
  }

  // Storage
  if (env.MCP_LOGS_MAX_LOGS) {
    const maxLogs = parseInt(env.MCP_LOGS_MAX_LOGS, 10);
    if (!isNaN(maxLogs)) {
      config.storage.max_logs = maxLogs;
    }
  }
  if (env.MCP_LOGS_STORAGE_TYPE) {
    config.storage.storage_type = env.MCP_LOGS_STORAGE_TYPE as any;
  }
  if (env.MCP_LOGS_SQLITE_PATH) {
    config.storage.sqlite_path = env.MCP_LOGS_SQLITE_PATH;
  }
  if (env.MCP_LOGS_POSTGRES_URL || env.DATABASE_URL) {
    config.storage.postgres_url = env.MCP_LOGS_POSTGRES_URL || env.DATABASE_URL;
  }

  // Logging
  if (env.MCP_LOGS_LOG_LEVEL) {
    config.logging.log_level = env.MCP_LOGS_LOG_LEVEL as any;
  }
  if (env.MCP_LOGS_LOG_FILE) {
    config.logging.log_file = env.MCP_LOGS_LOG_FILE;
  }
  if (env.MCP_LOGS_LOG_FORMAT) {
    config.logging.log_format = env.MCP_LOGS_LOG_FORMAT as any;
  }

  // Performance
  if (env.MCP_LOGS_BUFFER_SIZE) {
    const bufferSize = parseInt(env.MCP_LOGS_BUFFER_SIZE, 10);
    if (!isNaN(bufferSize)) {
      config.performance.buffer_size = bufferSize;
    }
  }
  if (env.MCP_LOGS_CONNECTION_TIMEOUT) {
    const timeout = parseInt(env.MCP_LOGS_CONNECTION_TIMEOUT, 10);
    if (!isNaN(timeout)) {
      config.performance.connection_timeout = timeout;
    }
  }
  if (env.MCP_LOGS_MAX_CONNECTIONS) {
    const maxConns = parseInt(env.MCP_LOGS_MAX_CONNECTIONS, 10);
    if (!isNaN(maxConns)) {
      config.performance.max_connections = maxConns;
    }
  }

  // Features
  if (env.MCP_LOGS_AUTO_CLEANUP) {
    config.features.auto_cleanup = env.MCP_LOGS_AUTO_CLEANUP === "true";
  }
  if (env.MCP_LOGS_MAX_LOG_AGE_HOURS) {
    const maxAge = parseInt(env.MCP_LOGS_MAX_LOG_AGE_HOURS, 10);
    if (!isNaN(maxAge)) {
      config.features.max_log_age_hours = maxAge;
    }
  }
  if (env.MCP_LOGS_ENABLE_STATS) {
    config.features.enable_stats = env.MCP_LOGS_ENABLE_STATS === "true";
  }

  return config;
}

/**
 * Get global config path
 */
export function getGlobalConfigPath(): string | null {
  try {
    const configDir = join(homedir(), ".config", "mcp-logs");
    return join(configDir, "config.json");
  } catch {
    return null;
  }
}

/**
 * Get local config path
 */
export function getLocalConfigPath(): string {
  return ".mcp-logs.json";
}

/**
 * Create default config file
 */
export function createDefaultConfig(path: string): void {
  const config = DEFAULT_CONFIG;
  const json = JSON.stringify(config, null, 2);

  // Create parent directory if needed
  const dir = path.substring(0, path.lastIndexOf("/"));
  if (dir && !existsSync(dir)) {
    mkdirSync(dir, { recursive: true });
  }

  writeFileSync(path, json, "utf-8");
}

/**
 * Save config to file
 */
export function saveConfig(config: Config, path: string): void {
  const json = JSON.stringify(config, null, 2);

  // Create parent directory if needed
  const dir = path.substring(0, path.lastIndexOf("/"));
  if (dir && !existsSync(dir)) {
    mkdirSync(dir, { recursive: true });
  }

  writeFileSync(path, json, "utf-8");
}

/**
 * Check if local config exists
 */
export function hasLocalConfig(): boolean {
  return existsSync(getLocalConfigPath());
}

/**
 * Check if global config exists
 */
export function hasGlobalConfig(): boolean {
  const path = getGlobalConfigPath();
  return path ? existsSync(path) : false;
}

/**
 * Check if current directory is a git repository
 */
export function isGitRepository(): boolean {
  return existsSync('.git');
}

/**
 * Check if config file is already in .gitignore
 */
export function isConfigInGitignore(configFilename: string): boolean {
  const gitignorePath = '.gitignore';
  
  if (!existsSync(gitignorePath)) {
    return false;
  }
  
  try {
    const content = readFileSync(gitignorePath, 'utf-8');
    const lines = content.split('\n').map(line => line.trim());
    
    // Chercher le fichier exact ou un pattern qui le couvre
    return lines.some(line => {
      if (line.startsWith('#') || line === '') return false;
      return line === configFilename || line === `/${configFilename}`;
    });
  } catch (error) {
    console.error(`Error reading .gitignore: ${error}`);
    return false;
  }
}

/**
 * Add config file to .gitignore
 */
export function addToGitignore(configFilename: string): void {
  const gitignorePath = '.gitignore';
  let content = '';
  
  // Lire le contenu existant si le fichier existe
  if (existsSync(gitignorePath)) {
    content = readFileSync(gitignorePath, 'utf-8');
    
    // S'assurer qu'il y a une nouvelle ligne à la fin
    if (!content.endsWith('\n')) {
      content += '\n';
    }
  }
  
  // Ajouter notre entrée avec un commentaire
  content += '\n# mcp-logs local configuration\n';
  content += `${configFilename}\n`;
  
  writeFileSync(gitignorePath, content, 'utf-8');
}
