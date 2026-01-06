#!/usr/bin/env bun

import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import {
  CallToolRequestSchema,
  ListToolsRequestSchema,
} from "@modelcontextprotocol/sdk/types.js";

import { LogStore } from "./src/store/index.js";
import { SocketServer } from "./src/server/index.js";
import { ToolHandlers } from "./src/mcp/handlers.js";
import { TOOLS } from "./src/mcp/tools.js";
import { loadConfig } from "./src/config/index.js";

async function main() {
  // Load configuration
  const config = loadConfig();
  const verbose = config.logging.log_level === "debug" || config.server.verbose;
  
  if (verbose) console.log("🚀 MCP Logs Server starting...");

  // Store global pour les logs
  const logStore = new LogStore(config.storage.max_logs);

  // Serveur Unix socket
  const socketServer = new SocketServer(logStore, config.server.socket_path, verbose);
  await socketServer.start();

  // Handlers pour les outils MCP
  const toolHandlers = new ToolHandlers(logStore, socketServer);

  // Serveur MCP
  const mcpServer = new Server(
    {
      name: config.server.name,
      version: config.server.version,
    },
    {
      capabilities: {
        tools: {},
      },
    }
  );

  // Liste des outils disponibles
  mcpServer.setRequestHandler(ListToolsRequestSchema, async () => {
    return { tools: TOOLS };
  });

  // Gestion des appels d'outils
  mcpServer.setRequestHandler(CallToolRequestSchema, async (request) => {
    const { name, arguments: args } = request.params;
    return toolHandlers.handleTool(name, args);
  });

  // Démarrage du serveur MCP
  const transport = new StdioServerTransport();
  await mcpServer.connect(transport);

  if (verbose) {
    console.log("✓ MCP server ready");
    console.log("ℹ Waiting for logs from log-agent CLI...");
    console.log();
  }

  // Gestion de l'arrêt propre
  const shutdown = async () => {
    console.log("\n🛑 Shutting down...");
    await socketServer.stop();
    process.exit(0);
  };

  process.on("SIGINT", shutdown);
  process.on("SIGTERM", shutdown);
}

main().catch((error) => {
  console.error("Fatal error:", error);
  process.exit(1);
});
