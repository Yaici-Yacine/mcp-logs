import { unlink } from "node:fs/promises";
import type { LogMessage, CommandMessage, CommandResponse, AgentHelloMessage } from "../types/index.js";
import { LogStore } from "../store/index.js";

export const SOCKET_PATH = "/tmp/log-agent.sock";

/**
 * Serveur Unix socket pour recevoir les logs de plusieurs CLI Rust
 * et envoyer des commandes aux agents
 */
export class SocketServer {
  private server: ReturnType<typeof Bun.listen> | null = null;
  private store: LogStore;
  private socketPath: string;
  private verbose: boolean;
  private connectedProjects: Set<string> = new Set();
  private clients: Map<string, any> = new Map(); // Map project name -> socket
  private commandSockets: Map<string, any> = new Map(); // Map project name -> command listener socket

  constructor(store: LogStore, socketPath: string = SOCKET_PATH, verbose = false) {
    this.store = store;
    this.socketPath = socketPath;
    this.verbose = verbose;
  }

  /**
   * Démarre le serveur Unix socket
   */
  async start(): Promise<void> {
    // Supprime le socket s'il existe déjà
    try {
      await unlink(this.socketPath);
    } catch {
      // Ignore si le fichier n'existe pas
    }

    this.server = Bun.listen({
      unix: this.socketPath,
      socket: {
        data: (socket, data) => {
          this.handleData(socket, data);
        },
        open: (socket) => {
          if (this.verbose) console.log("✓ Client connected");
        },
        close: (socket) => {
          // Remove socket from clients map when disconnected
          for (const [project, s] of this.clients.entries()) {
            if (s === socket) {
              this.clients.delete(project);
              this.connectedProjects.delete(project);
              console.log(`✓ Agent disconnected: ${project}`);
            }
          }
          if (this.verbose) console.log("✓ Client disconnected");
        },
        error: (socket, error) => {
          console.error("Socket error:", error);
        },
      },
    });

    console.log(`✓ Socket server listening `);
  }

  /**
   * Traite les données reçues (logs ou command responses)
   */
  private handleData(socket: any, data: Buffer): void {
    const text = data.toString("utf-8");
    const lines = text.split("\n").filter((line) => line.trim());

    for (const line of lines) {
      try {
        const msg = JSON.parse(line);
        
        if (msg.type === "agent_hello") {
          // Handle agent hello message to identify command listener
          const hello: AgentHelloMessage = msg;
          if (hello.data.mode === "command_listener") {
            this.commandSockets.set(hello.data.project, socket);
            this.connectedProjects.add(hello.data.project);
            if (this.verbose) {
              console.log(`✓ Command listener connected for: ${hello.data.project}`);
            }
          }
        } else if (msg.type === "log_entry") {
          // Handle log message
          const log: LogMessage = msg;
          this.store.add(log);

          // Register socket for this project
          if (!this.clients.has(log.data.project)) {
            this.clients.set(log.data.project, socket);
            this.connectedProjects.add(log.data.project);
            console.log(`✓ Agent connected: ${log.data.project}`);
          }

          // Affiche le log dans la console du serveur seulement en mode verbose
          if (this.verbose) {
            const emoji = this.getLevelEmoji(log.data.level);
            console.log(
              `${emoji} [${log.data.project}] ${log.data.message.substring(0, 100)}`
            );
          }
        } else if (msg.type === "command_response") {
          // Handle command response from agent
          const response: CommandResponse = msg;
          if (this.verbose) {
            console.log(`📩 Command response from ${response.data.project}: ${response.data.message}`);
          }
          if (response.data.success) {
            console.log(`✓ ${response.data.project}: ${response.data.message}${response.data.pid ? ` (PID: ${response.data.pid})` : ''}`);
          } else {
            console.error(`✗ ${response.data.project}: ${response.data.message}`);
          }
        }
      } catch (error) {
        console.error("Failed to parse message:", error, "Line:", line);
      }
    }
  }

  /**
   * Envoie une commande à un agent spécifique
   */
  async sendCommand(project: string, command: string): Promise<boolean> {
    // Use the command socket if available, otherwise fall back to log socket
    const socket = this.commandSockets.get(project) || this.clients.get(project);
    if (!socket) {
      if (this.verbose) {
        console.log(`✗ No socket found for project: ${project}`);
      }
      return false;
    }

    const message: CommandMessage = {
      version: "1.0",
      type: "command",
      data: {
        command,
        project,
        requestId: crypto.randomUUID(),
      },
    };

    try {
      socket.write(JSON.stringify(message) + "\n");
      if (this.verbose) {
        console.log(`📤 Sent command '${command}' to project '${project}'`);
      }
      return true;
    } catch (error) {
      console.error(`Failed to send command to ${project}:`, error);
      return false;
    }
  }

  /**
   * Retourne la liste des projets connectés
   */
  getConnectedProjects(): string[] {
    return Array.from(this.connectedProjects);
  }

  /**
   * Retourne un emoji pour le niveau de log
   */
  private getLevelEmoji(level: string): string {
    switch (level) {
      case "error":
        return "🔴";
      case "warn":
        return "🟡";
      case "debug":
        return "🔵";
      default:
        return "⚪";
    }
  }

  /**
   * Arrête le serveur
   */
  async stop(): Promise<void> {
    if (this.server) {
      this.server.stop();
      this.server = null;
    }

    try {
      await unlink(this.socketPath);
    } catch {
      // Ignore
    }

    console.log("✓ Socket server stopped");
  }
}
