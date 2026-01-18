import type { Tool } from "@modelcontextprotocol/sdk/types.js";

export const TOOLS: Tool[] = [
  {
    name: "get_recent_logs",
    description:
      "Get the most recent logs from all projects. Use this to see what's happening right now.",
    inputSchema: {
      type: "object",
      properties: {
        count: {
          type: "number",
          description:
            "Number of recent logs to retrieve (default: 50, max: 500)",
          default: 50,
        },
      },
    },
  },
  {
    name: "get_logs",
    description:
      "Get logs with advanced filtering options. Search by project, level, source, time range, or text content. Supports relative time formats like 'last 1h', 'last 30m', 'last 2d'.",
    inputSchema: {
      type: "object",
      properties: {
        project: {
          type: "string",
          description: "Filter by project name",
        },
        level: {
          type: "string",
          enum: ["info", "warn", "error", "debug"],
          description: "Filter by log level",
        },
        source: {
          type: "string",
          enum: ["stdout", "stderr"],
          description: "Filter by log source",
        },
        search: {
          type: "string",
          description: "Search for text in log messages",
        },
        startTime: {
          type: ["string", "number"],
          description:
            "Start time filter. Formats: ISO 8601 ('2026-01-18T10:00:00Z'), timestamp (1737201600000), or relative ('last 1h', 'last 30m', 'last 2d')",
        },
        endTime: {
          type: ["string", "number"],
          description:
            "End time filter. Same formats as startTime",
        },
        limit: {
          type: "number",
          description: "Maximum number of logs to return (default: 100)",
          default: 100,
        },
      },
    },
  },
  {
    name: "get_stats",
    description:
      "Get statistics about captured logs: total count, projects, log levels distribution.",
    inputSchema: {
      type: "object",
      properties: {},
    },
  },
  {
    name: "get_analytics",
    description:
      "Get advanced analytics and aggregations on logs. Provides insights like error rates, trends over time, top messages, and more. Perfect for understanding patterns and detecting issues.",
    inputSchema: {
      type: "object",
      properties: {
        project: {
          type: "string",
          description: "Optional: filter analytics by project name",
        },
        timeRange: {
          type: "string",
          enum: ["1h", "6h", "24h", "7d"],
          description:
            "Time range for analysis (default: all logs). Examples: '1h' (last hour), '24h' (last day), '7d' (last week)",
        },
        groupBy: {
          type: "string",
          enum: ["minute", "hour", "project", "level"],
          description:
            "Group results by time interval or dimension. 'minute' or 'hour' creates a timeline",
        },
        startTime: {
          type: ["string", "number"],
          description:
            "Custom start time (alternative to timeRange). Formats: ISO 8601, timestamp, or relative ('last 2h')",
        },
        endTime: {
          type: ["string", "number"],
          description: "Custom end time. Same formats as startTime",
        },
      },
    },
  },
  {
    name: "search_logs",
    description:
      "Search logs by text content. Returns matching logs with context. Supports both simple text search and regex patterns.",
    inputSchema: {
      type: "object",
      properties: {
        query: {
          type: "string",
          description:
            "Search query (case-insensitive by default). Can be a regex pattern if regex=true",
        },
        regex: {
          type: "boolean",
          description:
            "Enable regex pattern matching. Example: 'error:\\s+\\d+' to find 'error: 404'",
          default: false,
        },
        project: {
          type: "string",
          description: "Optional: filter by project name",
        },
        limit: {
          type: "number",
          description: "Maximum number of results (default: 50)",
          default: 50,
        },
      },
      required: ["query"],
    },
  },
  {
    name: "get_errors",
    description:
      "Get all error-level logs. Useful for debugging and finding issues.",
    inputSchema: {
      type: "object",
      properties: {
        project: {
          type: "string",
          description: "Optional: filter by project name",
        },
        limit: {
          type: "number",
          description: "Maximum number of errors to return (default: 50)",
          default: 50,
        },
      },
    },
  },
  {
    name: "clear_logs",
    description: "Clear all logs from memory. Use with caution!",
    inputSchema: {
      type: "object",
      properties: {},
    },
  },
  {
    name: "list_projects",
    description:
      "List all projects that have sent logs. Shows which log agents are connected.",
    inputSchema: {
      type: "object",
      properties: {},
    },
  },
];
