import { existsSync, mkdirSync, writeFileSync, readFileSync } from 'fs';
import { homedir } from 'os';
import { join } from 'path';
import { createInterface } from 'readline';
import { generateConfigWithComments, generateMinimalConfig } from '../config/generator.js';
import { loadConfig, isGitRepository, isConfigInGitignore, addToGitignore } from '../config/loader.js';
import type { Config } from '../config/types.js';

function getGlobalConfigPath(): string {
  return join(homedir(), '.config', 'mcp-logs', 'config.json');
}

function getLocalConfigPath(): string {
  return '.mcp-logs.json';
}

/**
 * Prompt user for yes/no answer
 */
function promptYesNo(question: string, defaultYes: boolean = true): Promise<boolean> {
  return new Promise((resolve) => {
    const rl = createInterface({
      input: process.stdin,
      output: process.stdout,
    });

    const defaultHint = defaultYes ? '[Y/n]' : '[y/N]';
    rl.question(`${question} ${defaultHint} `, (answer) => {
      rl.close();
      
      const normalized = answer.trim().toLowerCase();
      if (normalized === '') {
        resolve(defaultYes);
      } else {
        resolve(normalized === 'y' || normalized === 'yes');
      }
    });
  });
}

/**
 * Handle .gitignore management for local config
 */
async function handleGitignoreForLocalConfig(noGitignore: boolean, autoYes: boolean): Promise<void> {
  const configFilename = getLocalConfigPath();
  
  // Si le flag --no-gitignore est présent, on skip complètement
  if (noGitignore) {
    return;
  }
  
  // Vérifier si on est dans un dépôt git
  if (!isGitRepository()) {
    return;
  }
  
  // Vérifier si le fichier est déjà dans .gitignore
  if (isConfigInGitignore(configFilename)) {
    console.log();
    console.log(`ℹ ${configFilename} is already in .gitignore`);
    return;
  }
  
  // Si --yes, ajouter automatiquement
  if (autoYes) {
    addToGitignore(configFilename);
    console.log();
    console.log(`✓ Added ${configFilename} to .gitignore`);
    return;
  }
  
  // Sinon, demander à l'utilisateur
  console.log();
  console.log(`📁 Git repository detected`);
  
  const shouldAdd = await promptYesNo(`   Add ${configFilename} to .gitignore?`, true);
  
  if (shouldAdd) {
    addToGitignore(configFilename);
    console.log(`✓ Added ${configFilename} to .gitignore`);
  } else {
    console.log(`ℹ Skipped adding to .gitignore`);
  }
}

async function initConfig(args: string[]) {
  const isGlobal = args.includes('--global') || args.includes('-g');
  const isMinimal = args.includes('--minimal') || args.includes('-m');
  const autoYes = args.includes('--yes') || args.includes('-y');
  const noGitignore = args.includes('--no-gitignore') || args.includes('-n');
  
  const path = isGlobal ? getGlobalConfigPath() : getLocalConfigPath();
  const content = isMinimal ? generateMinimalConfig() : generateConfigWithComments();
  
  // Créer le répertoire parent si nécessaire
  const dir = path.substring(0, path.lastIndexOf('/'));
  if (dir && !existsSync(dir)) {
    mkdirSync(dir, { recursive: true });
  }
  
  // Écrire le fichier
  writeFileSync(path, content, 'utf-8');
  
  console.log(`✓ Created ${isGlobal ? 'global' : 'local'} configuration file`);
  console.log(`  Location: ${path}`);
  console.log();
  if (!isMinimal) {
    console.log('💡 Tip: Lines starting with "_" are comments explaining each option');
    console.log('   You can safely edit values without removing comments');
  }
  
  // Gestion du .gitignore pour la config locale uniquement
  if (!isGlobal) {
    await handleGitignoreForLocalConfig(noGitignore, autoYes);
  }
}

function showConfig() {
  const config = loadConfig();
  console.log('Current Configuration:');
  console.log(JSON.stringify(config, null, 2));
  console.log();
  console.log('Configuration sources checked:');
  console.log(`  1. Environment variables (MCP_LOGS_*)`);
  console.log(`  2. Local config: ${getLocalConfigPath()} ${existsSync(getLocalConfigPath()) ? '✓' : '✗'}`);
  console.log(`  3. Global config: ${getGlobalConfigPath()} ${existsSync(getGlobalConfigPath()) ? '✓' : '✗'}`);
  console.log(`  4. Default values`);
}

function getConfigValue(args: string[]) {
  const key = args[0];
  
  if (!key) {
    console.error('✗ Error: Missing key argument');
    console.log('Usage: bun run index.ts config get <section.field>');
    console.log('Example: bun run index.ts config get server.socket_path');
    process.exit(1);
  }
  
  const config = loadConfig();
  const [section, field] = key.split('.');
  
  if (!section || !field) {
    console.error('✗ Error: Invalid key format');
    console.log('Key must be in format: section.field');
    console.log('Example: server.socket_path');
    process.exit(1);
  }
  
  const sectionData = (config as any)[section];
  if (!sectionData) {
    console.error(`✗ Error: Section '${section}' not found`);
    process.exit(1);
  }
  
  const value = sectionData[field];
  if (value === undefined) {
    console.error(`✗ Error: Field '${field}' not found in section '${section}'`);
    process.exit(1);
  }
  
  console.log(JSON.stringify(value));
}

function setConfigValue(args: string[]) {
  const isGlobal = args.includes('--global') || args.includes('-g');
  const keyIndex = args.findIndex(arg => !arg.startsWith('--') && !arg.startsWith('-'));
  
  if (keyIndex === -1 || keyIndex + 1 >= args.length) {
    console.error('✗ Error: Missing arguments');
    console.log('Usage: bun run index.ts config set <section.field> <value> [--global]');
    console.log('Example: bun run index.ts config set server.verbose true');
    process.exit(1);
  }
  
  const key = args[keyIndex]!;
  const value = args[keyIndex + 1]!;
  const [section, field] = key.split('.');
  
  if (!section || !field) {
    console.error('✗ Error: Invalid key format');
    console.log('Key must be in format: section.field');
    process.exit(1);
  }
  
  const path = isGlobal ? getGlobalConfigPath() : getLocalConfigPath();
  
  // Charger la config existante ou créer une nouvelle
  let configData: any = {};
  if (existsSync(path)) {
    const content = readFileSync(path, 'utf-8');
    configData = JSON.parse(content);
  }
  
  // Créer la section si elle n'existe pas
  if (!configData[section]) {
    configData[section] = {};
  }
  
  // Parser la valeur selon le type
  const parsedValue = parseValue(section, field, value);
  
  // Mettre à jour la valeur
  configData[section][field] = parsedValue;
  
  // Sauvegarder
  const dir = path.substring(0, path.lastIndexOf('/'));
  if (dir && !existsSync(dir)) {
    mkdirSync(dir, { recursive: true });
  }
  
  writeFileSync(path, JSON.stringify(configData, null, 2), 'utf-8');
  
  console.log(`✓ Configuration updated: ${key} = ${JSON.stringify(parsedValue)}`);
  console.log(`  File: ${path}`);
}

function parseValue(section: string, field: string, value: string): any {
  // Booleans
  if (value === 'true') return true;
  if (value === 'false') return false;
  
  // Null
  if (value === 'null') return null;
  
  // Numbers
  if (/^-?\d+$/.test(value)) return parseInt(value, 10);
  if (/^-?\d+\.\d+$/.test(value)) return parseFloat(value);
  
  // Arrays/Objects (JSON)
  if (value.startsWith('[') || value.startsWith('{')) {
    try {
      return JSON.parse(value);
    } catch {
      throw new Error(`Invalid JSON: ${value}`);
    }
  }
  
  // Enums - valider selon le champ
  if (section === 'storage' && field === 'storage_type') {
    if (!['memory', 'sqlite', 'postgresql'].includes(value)) {
      throw new Error(`Invalid storage_type '${value}'. Must be: memory, sqlite, postgresql`);
    }
  }
  
  if (section === 'logging' && field === 'log_level') {
    if (!['debug', 'info', 'warn', 'error'].includes(value)) {
      throw new Error(`Invalid log_level '${value}'. Must be: debug, info, warn, error`);
    }
  }
  
  if (section === 'logging' && field === 'log_format') {
    if (!['text', 'json'].includes(value)) {
      throw new Error(`Invalid log_format '${value}'. Must be: text, json`);
    }
  }
  
  // String par défaut
  return value;
}

function listConfigKeys() {
  console.log('Available configuration keys:');
  console.log();
  console.log('📦 [server]');
  console.log('  socket_path           - Unix socket path');
  console.log('  verbose               - Enable verbose logging');
  console.log('  name                  - Server name');
  console.log('  version               - Server version');
  console.log();
  console.log('💾 [storage]');
  console.log('  max_logs              - Maximum logs to keep in memory');
  console.log('  storage_type          - Storage backend (memory, sqlite, postgresql)');
  console.log('  sqlite_path           - SQLite database path (if using sqlite)');
  console.log('  postgres_url          - PostgreSQL connection URL (if using postgresql)');
  console.log();
  console.log('📝 [logging]');
  console.log('  log_level             - Logging level (debug, info, warn, error)');
  console.log('  log_file              - Log file path (null for stdout)');
  console.log('  log_format            - Log format (text, json)');
  console.log();
  console.log('⚡ [performance]');
  console.log('  buffer_size           - Socket buffer size in bytes');
  console.log('  connection_timeout    - Connection timeout in seconds');
  console.log('  max_connections       - Maximum simultaneous connections');
  console.log();
  console.log('✨ [features]');
  console.log('  auto_cleanup          - Enable automatic log cleanup');
  console.log('  max_log_age_hours     - Maximum log age before cleanup');
  console.log('  enable_stats          - Enable statistics collection');
  console.log('  compress_logs         - Enable log compression');
}

function showHelp() {
  console.log('MCP Logs Configuration CLI');
  console.log();
  console.log('Usage: bun run index.ts config <command> [options]');
  console.log();
  console.log('Commands:');
  console.log('  init [options]                       Create configuration file');
  console.log('    --global, -g                       Create in ~/.config/mcp-logs/');
  console.log('    --minimal, -m                      Create without comments');
  console.log('    --yes, -y                          Auto-add to .gitignore without prompting');
  console.log('    --no-gitignore, -n                 Skip .gitignore management');
  console.log('  show                                 Display current merged configuration');
  console.log('  get <key>                            Get specific configuration value');
  console.log('  set <key> <value> [--global|-g]     Set configuration value');
  console.log('  list                                 List all available configuration keys');
  console.log('  help                                 Show this help message');
  console.log();
  console.log('Examples:');
  console.log('  bun run index.ts config init               Create local config (prompt for .gitignore)');
  console.log('  bun run index.ts config init --yes         Create and auto-add to .gitignore');
  console.log('  bun run index.ts config init -n            Create without .gitignore');
  console.log('  bun run index.ts config show               Show merged config');
  console.log('  bun run index.ts config get server.verbose');
  console.log('  bun run index.ts config set server.verbose true');
  console.log('  bun run index.ts config set storage.max_logs 20000');
  console.log('  bun run index.ts config set logging.log_level debug --global');
  console.log('  bun run index.ts config list');
  console.log();
  console.log('Server mode:');
  console.log('  bun run index.ts                     Start MCP server (default)');
  console.log();
  console.log('Configuration files:');
  console.log(`  Local:  ${getLocalConfigPath()}`);
  console.log(`  Global: ${getGlobalConfigPath()}`);
}

export async function runConfigCLI(args: string[]) {
  const command = args[0];
  
  const commands: Record<string, (args: string[]) => void | Promise<void>> = {
    init: initConfig,
    show: showConfig,
    get: getConfigValue,
    set: setConfigValue,
    list: listConfigKeys,
    help: showHelp,
  };
  
  if (!command || !commands[command]) {
    showHelp();
    process.exit(command ? 1 : 0);
  }
  
  await commands[command](args.slice(1));
}
