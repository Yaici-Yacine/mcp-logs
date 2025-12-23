import type { CallToolResult } from "@modelcontextprotocol/sdk/types.js";
import type { LogStore } from "../store/index.js";
import type { LogFilter, LogLevel, LogSource } from "../types/index.js";

interface ToolArguments {
  [key: string]: unknown;
}

export class ToolHandlers {
  constructor(private store: LogStore) {}

  async handleTool(name: string, args?: ToolArguments): Promise<CallToolResult> {
    try {
      const matchName: Record<string, () => CallToolResult> = {
        get_recent_logs: () => this.getRecentLogs(args),
        get_logs: () => this.getLogs(args),
        get_stats: () => this.getStats(),
        search_logs: () => this.searchLogs(args),
        get_errors: () => this.getErrors(args),
        clear_logs: () => this.clearLogs(),
      };

      const handler = matchName[name];
      if (!handler) {
        throw new Error(`Unknown tool: ${name}`);
      }

      return handler();
    } catch (error) {
      return {
        content: [
          {
            type: "text",
            text: JSON.stringify({
              error: error instanceof Error ? error.message : String(error),
            }),
          },
        ],
        isError: true,
      };
    }
  }

  private getRecentLogs(args?: ToolArguments): CallToolResult {
    const count = Math.min(
      typeof args?.count === "number" ? args.count : 50,
      500
    );
    const logs = this.store.getRecent(count);

    return {
      content: [
        {
          type: "text",
          text: JSON.stringify(
            {
              count: logs.length,
              logs: logs.map((log) => ({
                timestamp: log.data.timestamp,
                project: log.data.project,
                level: log.data.level,
                source: log.data.source,
                message: log.data.message,
                pid: log.data.pid,
              })),
            },
            null,
            2
          ),
        },
      ],
    };
  }

  private getLogs(args?: ToolArguments): CallToolResult {
    const filter: LogFilter = {
      project:
        typeof args?.project === "string" ? args.project : undefined,
      level: this.isValidLogLevel(args?.level)
        ? (args.level as LogLevel)
        : undefined,
      source: this.isValidLogSource(args?.source)
        ? (args.source as LogSource)
        : undefined,
      search:
        typeof args?.search === "string" ? args.search : undefined,
      limit: Math.min(
        typeof args?.limit === "number" ? args.limit : 100,
        1000
      ),
    };

    const logs = this.store.getAll(filter);

    return {
      content: [
        {
          type: "text",
          text: JSON.stringify(
            {
              filter,
              count: logs.length,
              logs: logs.map((log) => log.data),
            },
            null,
            2
          ),
        },
      ],
    };
  }

  private getStats(): CallToolResult {
    const stats = this.store.getStats();

    return {
      content: [
        {
          type: "text",
          text: JSON.stringify(stats, null, 2),
        },
      ],
    };
  }

  private searchLogs(args?: ToolArguments): CallToolResult {
    const query = args?.query;

    if (typeof query !== "string") {
      throw new Error("Search query is required");
    }

    const project =
      typeof args?.project === "string" ? args.project : undefined;
    const limit = Math.min(
      typeof args?.limit === "number" ? args.limit : 50,
      500
    );

    const logs = this.store.getAll({ search: query, project, limit });

    return {
      content: [
        {
          type: "text",
          text: JSON.stringify(
            {
              query,
              project,
              count: logs.length,
              logs: logs.map((log) => log.data),
            },
            null,
            2
          ),
        },
      ],
    };
  }

  private getErrors(args?: ToolArguments): CallToolResult {
    const project =
      typeof args?.project === "string" ? args.project : undefined;
    const limit = Math.min(
      typeof args?.limit === "number" ? args.limit : 50,
      500
    );

    const logs = this.store.getAll({ level: "error", project, limit });

    return {
      content: [
        {
          type: "text",
          text: JSON.stringify(
            {
              project,
              count: logs.length,
              errors: logs.map((log) => log.data),
            },
            null,
            2
          ),
        },
      ],
    };
  }

  private clearLogs(): CallToolResult {
    const beforeCount = this.store.count();
    this.store.clear();

    return {
      content: [
        {
          type: "text",
          text: JSON.stringify({
            message: "Logs cleared",
            clearedCount: beforeCount,
          }),
        },
      ],
    };
  }

  private isValidLogLevel(value: unknown): value is LogLevel {
    return (
      typeof value === "string" &&
      ["info", "warn", "error", "debug"].includes(value)
    );
  }

  private isValidLogSource(value: unknown): value is LogSource {
    return (
      typeof value === "string" && ["stdout", "stderr"].includes(value)
    );
  }
}
