import type { LogMessage, LogFilter } from "../types/index.js";

/**
 * Store en mémoire pour les logs en temps réel
 */
export class LogStore {
  private logs: LogMessage[] = [];
  private maxLogs: number;

  constructor(maxLogs: number = 10000) {
    this.maxLogs = maxLogs;
  }

  /**
   * Ajoute un log au store
   */
  add(log: LogMessage): void {
    this.logs.push(log);

    // Limite le nombre de logs en mémoire (FIFO)
    if (this.logs.length > this.maxLogs) {
      this.logs.shift();
    }
  }

  /**
   * Récupère tous les logs (ou filtrés)
   */
  getAll(filter?: LogFilter): LogMessage[] {
    let filtered = [...this.logs];

    if (!filter) {
      return filtered;
    }

    // Filtre par projet
    if (filter.project) {
      filtered = filtered.filter((log) => log.data.project === filter.project);
    }

    // Filtre par niveau
    if (filter.level) {
      filtered = filtered.filter((log) => log.data.level === filter.level);
    }

    // Filtre par source
    if (filter.source) {
      filtered = filtered.filter((log) => log.data.source === filter.source);
    }

    // Filtre par timestamp
    if (filter.startTime) {
      filtered = filtered.filter(
        (log) => log.data.timestamp >= filter.startTime!
      );
    }

    if (filter.endTime) {
      filtered = filtered.filter(
        (log) => log.data.timestamp <= filter.endTime!
      );
    }

    // Recherche textuelle
    if (filter.search) {
      const searchLower = filter.search.toLowerCase();
      filtered = filtered.filter((log) =>
        log.data.message.toLowerCase().includes(searchLower)
      );
    }

    // Limite le nombre de résultats
    if (filter.limit && filter.limit > 0) {
      filtered = filtered.slice(-filter.limit);
    }

    return filtered;
  }

  /**
   * Récupère les derniers N logs
   */
  getRecent(count: number = 100): LogMessage[] {
    return this.logs.slice(-count);
  }

  /**
   * Récupère les logs par projet
   */
  getByProject(project: string, limit?: number): LogMessage[] {
    return this.getAll({ project, limit });
  }

  /**
   * Récupère les statistiques
   */
  getStats() {
    const projects = new Set(this.logs.map((log) => log.data.project));
    const levels = this.logs.reduce(
      (acc, log) => {
        acc[log.data.level] = (acc[log.data.level] || 0) + 1;
        return acc;
      },
      {} as Record<string, number>
    );

    return {
      totalLogs: this.logs.length,
      projects: Array.from(projects),
      projectCount: projects.size,
      levels,
    };
  }

  /**
   * Vide le store
   */
  clear(): void {
    this.logs = [];
  }

  /**
   * Compte le nombre de logs
   */
  count(): number {
    return this.logs.length;
  }
}
