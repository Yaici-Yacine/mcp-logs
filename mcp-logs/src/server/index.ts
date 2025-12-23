import { unlink } from "node:fs/promises";
import type { LogMessage } from "../types/index.js";
import { LogStore } from "../store/index.js";

export const SOCKET_PATH = "/tmp/log-agent.sock";

/**
 * Serveur Unix socket pour recevoir les logs du CLI Rust
 */
export class SocketServer {
  private server: ReturnType<typeof Bun.listen> | null = null;
  private store: LogStore;
  private socketPath: string;

  constructor(store: LogStore, socketPath: string = SOCKET_PATH) {
    this.store = store;
    this.socketPath = socketPath;
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
          this.handleData(data);
        },
        open: (socket) => {
          console.log("✓ Client connected");
        },
        close: (socket) => {
          console.log("✓ Client disconnected");
        },
        error: (socket, error) => {
          console.error("Socket error:", error);
        },
      },
    });

    console.log(`✓ Socket server listening on ${this.socketPath}`);
  }

  /**
   * Traite les données reçues
   */
  private handleData(data: Buffer): void {
    const text = data.toString("utf-8");
    const lines = text.split("\n").filter((line) => line.trim());

    for (const line of lines) {
      try {
        const log: LogMessage = JSON.parse(line);
        this.store.add(log);

        // Affiche le log dans la console du serveur
        const emoji = this.getLevelEmoji(log.data.level);
        console.log(
          `${emoji} [${log.data.project}] ${log.data.message.substring(0, 100)}`
        );
      } catch (error) {
        console.error("Failed to parse log:", error, "Line:", line);
      }
    }
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
