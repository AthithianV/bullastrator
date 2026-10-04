const apiBaseUrl = (
  import.meta.env.VITE_API_URL || "http://127.0.0.1:3000"
).replace(/\/$/, "");

export type CommandArgs = Record<string, any>;
export type HttpMethod = "GET" | "POST" | "PATCH" | "DELETE";

interface WebCommand {
  method: HttpMethod;
  path: (args: CommandArgs) => string;
  query?: (args: CommandArgs) => Record<string, unknown>;
  body?: (args: CommandArgs) => unknown;
}

const id = (value: unknown, name: string): string => {
  if (typeof value !== "string" || value.length === 0) {
    throw new Error(`Missing ${name}`);
  }
  return encodeURIComponent(value);
};

const snakeCase = (value: unknown): unknown => {
  if (Array.isArray(value)) return value.map(snakeCase);
  if (!value || typeof value !== "object") return value;

  return Object.fromEntries(
    Object.entries(value as Record<string, unknown>).map(([key, item]) => [
      key.replace(/[A-Z]/g, (letter) => `_${letter.toLowerCase()}`),
      snakeCase(item),
    ]),
  );
};

const withoutRouteArgs = (args: CommandArgs, ...keys: string[]) => {
  const body = { ...args };
  for (const key of keys) delete body[key];
  return snakeCase(body);
};

/** Maps the existing desktop command names to the web API. */
export const webCommandMap: Record<string, WebCommand> = {
  get_all_queues_by_connection: {
    method: "GET",
    path: (args) =>
      `/connections/${id(args.connectionId, "connectionId")}/queues`,
  },
  get_queue_details: {
    method: "GET",
    path: (args) =>
      `/connections/${id(args.connectionId, "connectionId")}/queues/${id(args.queueName, "queueName")}`,
  },
  pause_queue: {
    method: "POST",
    path: (args) =>
      `/connections/${id(args.connectionId, "connectionId")}/queues/${id(args.queueName, "queueName")}/pause`,
    body: (args) => ({ paused: args.shouldPause }),
  },
  get_jobs_in_queue: {
    method: "GET",
    path: (args) =>
      `/connections/${id(args.connectionId, "connectionId")}/queues/${id(args.queueName, "queueName")}/jobs`,
    query: (args) => ({
      status: args.status,
      cursor: args.cursor,
      limit: args.limit,
    }),
  },
  get_jobs_by_id: {
    method: "GET",
    path: (args) =>
      `/connections/${id(args.connectionId, "connectionId")}/queues/${id(args.queueName, "queueName")}/jobs/${id(args.jobId, "jobId")}`,
    query: (args) => ({ status: args.status }),
  },
  get_job_logs: {
    method: "GET",
    path: (args) =>
      `/connections/${id(args.connectionId, "connectionId")}/queues/${id(args.queueName, "queueName")}/jobs/${id(args.jobId, "jobId")}/logs`,
  },
  get_jobs_data: {
    method: "POST",
    path: (args) =>
      `/connections/${id(args.connectionId, "connectionId")}/queues/${id(args.queueName, "queueName")}/jobs/data`,
    body: (args) => withoutRouteArgs(args, "connectionId", "queueName"),
  },
  get_job_count_in_queue: {
    method: "POST",
    path: (args) =>
      `/connections/${id(args.connectionId, "connectionId")}/queues/${id(args.queueName, "queueName")}/jobs/counts`,
    body: (args) => ({ queue_names: [args.queueName] }),
  },
  add_job_to_queue: {
    method: "POST",
    path: (args) =>
      `/connections/${id(args.connectionId, "connectionId")}/queues/${id(args.queueName, "queueName")}/jobs`,
    body: (args) => args.jobs,
  },
  update_job_data: {
    method: "POST",
    path: (args) =>
      `/connections/${id(args.connectionId, "connectionId")}/queues/${id(args.queueName, "queueName")}/jobs/update`,
    body: (args) => ({ job_id: args.jobId, data: args.jobData }),
  },
  retry_failed_jobs: {
    method: "POST",
    path: (args) =>
      `/connections/${id(args.connectionId, "connectionId")}/queues/${id(args.queueName, "queueName")}/jobs/retry`,
    body: (args) => ({
      job_ids: args.jobIds,
      strategy: args.strategy,
      status: args.retryJobStatus,
    }),
  },
  retry_all_failed_jobs: {
    method: "POST",
    path: (args) =>
      `/connections/${id(args.connectionId, "connectionId")}/queues/${id(args.queueName, "queueName")}/jobs/retry-all`,
    body: (args) => ({ strategy: args.strategy, status: args.retryJobStatus }),
  },
  promote_jobs: {
    method: "POST",
    path: (args) =>
      `/connections/${id(args.connectionId, "connectionId")}/queues/${id(args.queueName, "queueName")}/jobs/promote`,
    body: (args) => ({ job_ids: args.jobIds }),
  },
  promote_all_jobs: {
    method: "POST",
    path: (args) =>
      `/connections/${id(args.connectionId, "connectionId")}/queues/${id(args.queueName, "queueName")}/jobs/promote-all`,
  },
  delete_jobs: {
    method: "POST",
    path: (args) =>
      `/connections/${id(args.connectionId, "connectionId")}/queues/${id(args.queueName, "queueName")}/jobs/delete`,
    body: (args) => ({
      job_ids: args.jobIds,
      remove_children: args.removeChildren ?? false,
    }),
  },
  delete_all_jobs: {
    method: "POST",
    path: (args) =>
      `/connections/${id(args.connectionId, "connectionId")}/queues/${id(args.queueName, "queueName")}/jobs/delete-state`,
    body: (args) => ({
      state: args.status,
      remove_children: args.removeChildren ?? false,
    }),
  },
  get_workspace_by_id: {
    method: "GET",
    path: (args) => `/workspaces/${id(args.id, "id")}`,
  },
  create_workspace: {
    method: "POST",
    path: () => "/workspaces",
    body: (args) => snakeCase(args.data),
  },
  update_workspace: {
    method: "PATCH",
    path: (args) => `/workspaces/${id(args.id, "id")}`,
    body: (args) => snakeCase(args.data),
  },
  delete_workspace: {
    method: "DELETE",
    path: (args) => `/workspaces/${id(args.id, "id")}`,
  },
  set_active_workspace: {
    method: "POST",
    path: (args) => `/workspaces/${id(args.workspaceId, "workspaceId")}/select`,
  },
  get_active_workspace: {
    method: "GET",
    path: () => "/workspaces/active",
  },
};

export async function callWebApi<T>(
  cmd: string,
  args: CommandArgs,
): Promise<T> {
  const route = webCommandMap[cmd];
  if (!route) throw new Error(`No web API mapping exists for command: ${cmd}`);

  const query = route.query?.(args);
  const queryString = query
    ? new URLSearchParams(
        Object.entries(query)
          .filter(([, value]) => value !== undefined && value !== null)
          .map(([key, value]) => [key, String(value)]),
      ).toString()
    : "";
  const path = `${apiBaseUrl}${route.path(args)}${queryString ? `?${queryString}` : ""}`;
  const body = route.body?.(args);

  const response = await fetch(path, {
    method: route.method,
    headers:
      body === undefined ? undefined : { "content-type": "application/json" },
    body: body === undefined ? undefined : JSON.stringify(body),
  });
  const contentType = response.headers.get("content-type") || "";
  const result = contentType.includes("application/json")
    ? await response.json()
    : await response.text();

  if (!response.ok) {
    const message =
      typeof result === "object" && result && "error" in result
        ? String((result as { error: unknown }).error)
        : String(result || response.statusText);
    throw new Error(message);
  }
  return result as T;
}
