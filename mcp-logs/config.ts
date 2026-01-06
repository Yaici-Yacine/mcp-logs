#!/usr/bin/env bun

/**
 * CLI pour la gestion de la configuration mcp-logs
 * Usage: bun run config.ts <command>
 */

import { existsSync, mkdirSync, writeFileSync } from 'fs';
import { homedir } from 'os';
import { join } from 'path';
import { generateConfigWithComments, generateMinimalConfig } from './src/config/generator.js';
import { loadConfig } from './src/config/loader.js';

const commands = {
  init: initConfig,
  show: showConfig,
  help: showHelp,
};

function getGlobalConfigPath(): string {
  return join(homedir(), '.config', 'mcp-logs', 'config.json');
}

function getLocalConfigPath(): string {
  return '.mcp-logs.json';
}

function initConfig() {
  const args = process.argv.slice(3);
  const isGlobal = args.includes('--global') || args.includes('-g');
  const isMinimal = args.includes('--minimal') || args.includes('-m');
  
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

function showHelp() {
  console.log('MCP Logs Configuration CLI');
  console.log();
  console.log('Usage: bun run config.ts <command> [options]');
  console.log();
  console.log('Commands:');
  console.log('  init [--global|-g] [--minimal|-m]  Create configuration file');
  console.log('                                       --global: Create in ~/.config/mcp-logs/');
  console.log('                                       --minimal: Create without comments');
  console.log('  show                                 Display current configuration');
  console.log('  help                                 Show this help message');
  console.log();
  console.log('Examples:');
  console.log('  bun run config.ts init              Create local config with comments');
  console.log('  bun run config.ts init --global     Create global config');
  console.log('  bun run config.ts init --minimal    Create minimal config');
  console.log('  bun run config.ts show              Show merged configuration');
  console.log();
  console.log('Configuration files:');
  console.log(`  Local:  ${getLocalConfigPath()}`);
  console.log(`  Global: ${getGlobalConfigPath()}`);
}

// Main
const command = process.argv[2];

if (!command || !commands[command as keyof typeof commands]) {
  showHelp();
  process.exit(command ? 1 : 0);
}

commands[command as keyof typeof commands]();
