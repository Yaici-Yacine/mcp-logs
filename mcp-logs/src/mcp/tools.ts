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
      "Get logs with advanced filtering options. Search by project, level, source, time range, or text content.",
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
    name: "search_logs",
    description:
      "Search logs by text content. Returns matching logs with context.",
    inputSchema: {
      type: "object",
      properties: {
        query: {
          type: "string",
          description: "Search query (case-insensitive)",
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
