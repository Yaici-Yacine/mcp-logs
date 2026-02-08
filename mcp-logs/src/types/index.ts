export interface LogMessage {
  version: string;
  type: string;
  data: LogData;
}

export interface LogData {
  timestamp: string;
  level: LogLevel;
  source: LogSource;
  project: string;
  message: string;
  pid: number;
}

export type LogLevel = "info" | "warn" | "error" | "debug";
export type LogSource = "stdout" | "stderr";

export interface LogFilter {
  project?: string;
  level?: LogLevel;
  source?: LogSource;
  startTime?: string | number;
  endTime?: string | number;
  search?: string;
  regex?: boolean; // Pour la recherche par regex
  limit?: number;
}

export interface AnalyticsOptions {
  project?: string;
  timeRange?: string; // "1h", "6h", "24h", "7d"
  groupBy?: "minute" | "hour" | "project" | "level";
  startTime?: string | number;
  endTime?: string | number;
}

export interface Analytics {
  summary: {
    totalLogs: number;
    timeRange: {
      start: string;
      end: string;
      duration: string;
    };
    projects: string[];
  };
  byLevel: Record<LogLevel, number>;
  byProject: Record<string, number>;
  timeline?: Array<{
    timestamp: string;
    count: number;
  }>;
  topMessages?: Array<{
    message: string;
    count: number;
    level: LogLevel;
  }>;
  errorRate?: {
    total: number;
    errors: number;
    percentage: number;
  };
}

// Command protocol types for bidirectional communication
export interface CommandMessage {
  version: string;
  type: "command";
  data: CommandData;
}

export interface CommandData {
  command: string; // "restart"
  project: string;
  requestId: string;
}

export interface CommandResponse {
  version: string;
  type: "command_response";
  data: CommandResponseData;
}

export interface CommandResponseData {
  requestId: string;
  success: boolean;
  message: string;
  pid?: number;
  project: string;
}

// Agent hello message to identify connection type
export interface AgentHelloMessage {
  version: string;
  type: "agent_hello";
  data: {
    project: string;
    mode: "command_listener" | "log_sender";
  };
}
