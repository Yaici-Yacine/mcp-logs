/**
 * Configuration module for mcp-logs
 * Re-exports configuration functions and types
 */

export { loadConfig, getGlobalConfigPath, getLocalConfigPath } from "./loader.js";
export type { Config } from "./types.js";
export { DEFAULT_CONFIG } from "./types.js";
