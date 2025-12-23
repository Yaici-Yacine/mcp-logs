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
  startTime?: string;
  endTime?: string;
  search?: string;
  limit?: number;
}
