import type { LogMessage, LogFilter, Analytics, AnalyticsOptions } from "../types/index.js";
import { isInTimeRange, parseTimeInput, formatDuration, groupByTimeInterval } from "../utils/time.js";

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
    if (filter.startTime !== undefined || filter.endTime !== undefined) {
      filtered = filtered.filter((log) =>
        isInTimeRange(log.data.timestamp, filter.startTime, filter.endTime)
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

  /**
   * Génère des analytics avancées
   */
  getAnalytics(options: AnalyticsOptions = {}): Analytics {
    // Déterminer la plage de temps
    let startTime: string | number | undefined = options.startTime;
    let endTime: string | number | undefined = options.endTime;

    if (options.timeRange) {
      endTime = Date.now();
      startTime = `last ${options.timeRange}`;
    }

    // Filtrer les logs selon les critères
    let filteredLogs = this.logs;

    if (options.project) {
      filteredLogs = filteredLogs.filter((log) => log.data.project === options.project);
    }

    if (startTime !== undefined || endTime !== undefined) {
      filteredLogs = filteredLogs.filter((log) =>
        isInTimeRange(log.data.timestamp, startTime, endTime)
      );
    }

    // Calculer les statistiques
    const projects = new Set(filteredLogs.map((log) => log.data.project));
    
    const byLevel = filteredLogs.reduce(
      (acc, log) => {
        acc[log.data.level] = (acc[log.data.level] || 0) + 1;
        return acc;
      },
      {} as Record<string, number>
    );

    const byProject = filteredLogs.reduce(
      (acc, log) => {
        acc[log.data.project] = (acc[log.data.project] || 0) + 1;
        return acc;
      },
      {} as Record<string, number>
    );

    // Timeline (groupé par intervalle)
    let timeline: Array<{ timestamp: string; count: number }> | undefined;
    if (options.groupBy === "minute" || options.groupBy === "hour") {
      const intervalMinutes = options.groupBy === "minute" ? 1 : 60;
      const grouped = groupByTimeInterval(
        filteredLogs.map((log) => ({ timestamp: log.data.timestamp })),
        intervalMinutes
      );
      timeline = Array.from(grouped.entries()).map(([timestamp, count]) => ({
        timestamp,
        count,
      }));
    }

    // Top messages les plus fréquents
    const messageCounts = new Map<string, { count: number; level: string }>();
    for (const log of filteredLogs) {
      const msg = log.data.message.substring(0, 100); // Tronquer pour regrouper
      const existing = messageCounts.get(msg);
      if (existing) {
        existing.count++;
      } else {
        messageCounts.set(msg, { count: 1, level: log.data.level });
      }
    }

    const topMessages = Array.from(messageCounts.entries())
      .map(([message, data]) => ({
        message,
        count: data.count,
        level: data.level as any,
      }))
      .sort((a, b) => b.count - a.count)
      .slice(0, 10);

    // Taux d'erreurs
    const errorCount = byLevel.error || 0;
    const totalCount = filteredLogs.length;
    const errorRate = {
      total: totalCount,
      errors: errorCount,
      percentage: totalCount > 0 ? (errorCount / totalCount) * 100 : 0,
    };

    // Calculer la durée réelle
    let actualStart: Date;
    let actualEnd: Date;

    if (filteredLogs.length > 0) {
      const timestamps = filteredLogs.map((log) => new Date(log.data.timestamp).getTime());
      actualStart = new Date(Math.min(...timestamps));
      actualEnd = new Date(Math.max(...timestamps));
    } else {
      actualStart = startTime ? parseTimeInput(startTime) : new Date();
      actualEnd = endTime ? parseTimeInput(endTime) : new Date();
    }

    const duration = formatDuration(actualEnd.getTime() - actualStart.getTime());

    return {
      summary: {
        totalLogs: filteredLogs.length,
        timeRange: {
          start: actualStart.toISOString(),
          end: actualEnd.toISOString(),
          duration,
        },
        projects: Array.from(projects),
      },
      byLevel: byLevel as any,
      byProject,
      timeline,
      topMessages,
      errorRate,
    };
  }
}
