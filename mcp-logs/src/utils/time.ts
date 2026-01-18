/**
 * Utilitaires pour la gestion du temps et des dates
 */

/**
 * Parse un format de temps relatif ou absolu
 * Formats supportés:
 * - ISO 8601: "2026-01-18T10:00:00Z"
 * - Timestamp: 1737201600000
 * - Relatif: "last 1h", "last 30m", "last 2d", "last 1w"
 */
export function parseTimeInput(input: string | number): Date {
  // Si c'est un nombre, c'est un timestamp
  if (typeof input === 'number') {
    return new Date(input);
  }

  // Si c'est une date ISO, parser directement
  if (input.includes('T') || input.includes('-')) {
    return new Date(input);
  }

  // Format relatif: "last 1h", "last 30m", etc.
  const relativeMatch = input.match(/^last\s+(\d+)\s*(s|m|h|d|w)$/i);
  if (relativeMatch && relativeMatch[1] && relativeMatch[2]) {
    const amount = parseInt(relativeMatch[1]);
    const unit = relativeMatch[2].toLowerCase() as 's' | 'm' | 'h' | 'd' | 'w';

    const now = new Date();
    const milliseconds: Record<string, number> = {
      s: 1000,           // secondes
      m: 60 * 1000,      // minutes
      h: 60 * 60 * 1000, // heures
      d: 24 * 60 * 60 * 1000, // jours
      w: 7 * 24 * 60 * 60 * 1000, // semaines
    };

    const ms = milliseconds[unit];
    if (!ms) {
      throw new Error(`Invalid time unit: ${unit}`);
    }

    return new Date(now.getTime() - amount * ms);
  }

  // Si aucun format reconnu, essayer de parser comme date
  const parsed = new Date(input);
  if (isNaN(parsed.getTime())) {
    throw new Error(`Invalid time format: ${input}. Use ISO 8601, timestamp, or "last Xh/Xm/Xd"`);
  }

  return parsed;
}

/**
 * Vérifie si un timestamp est dans une plage donnée
 */
export function isInTimeRange(
  timestamp: string,
  startTime?: string | number,
  endTime?: string | number
): boolean {
  const logTime = new Date(timestamp).getTime();

  if (startTime !== undefined) {
    const start = parseTimeInput(startTime).getTime();
    if (logTime < start) return false;
  }

  if (endTime !== undefined) {
    const end = parseTimeInput(endTime).getTime();
    if (logTime > end) return false;
  }

  return true;
}

/**
 * Formate une durée en format lisible
 */
export function formatDuration(milliseconds: number): string {
  const seconds = Math.floor(milliseconds / 1000);
  const minutes = Math.floor(seconds / 60);
  const hours = Math.floor(minutes / 60);
  const days = Math.floor(hours / 24);

  if (days > 0) return `${days}d ${hours % 24}h`;
  if (hours > 0) return `${hours}h ${minutes % 60}m`;
  if (minutes > 0) return `${minutes}m ${seconds % 60}s`;
  return `${seconds}s`;
}

/**
 * Groupe les logs par intervalle de temps
 */
export function groupByTimeInterval(
  logs: Array<{ timestamp: string }>,
  intervalMinutes: number = 60
): Map<string, number> {
  const groups = new Map<string, number>();
  const intervalMs = intervalMinutes * 60 * 1000;

  for (const log of logs) {
    const timestamp = new Date(log.timestamp).getTime();
    const bucket = Math.floor(timestamp / intervalMs) * intervalMs;
    const bucketKey = new Date(bucket).toISOString();

    groups.set(bucketKey, (groups.get(bucketKey) || 0) + 1);
  }

  return groups;
}
