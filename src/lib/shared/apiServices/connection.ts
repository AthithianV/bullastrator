import { id, type WebCommand } from ".";

export const CONNECTION_ROUTES: Record<string, WebCommand> = {
  get_all_connections: {
    method: "GET",
    path: (args) => `/connections`,
  },
  get_all_queues_by_workspace: {
    method: "GET",
    path: () => "/connections/queues/list",
  },
  get_connection: {
    method: "GET",
    path: (args) => `/connections/${id(args.id, "id")}`,
  },
  create_connection: {
    method: "POST",
    path: () => "/connections",
    body: (args) => args.data,
  },
  update_connection: {
    method: "PATCH",
    path: (args) => `/connections/${id(args.id, "id")}`,
    body: (args) => args.data,
  },
  delete_connection: {
    method: "DELETE",
    path: (args) => `/connections/${id(args.id, "id")}`,
  },
  check_health_for_all_connections: {
    method: "POST",
    path: (args) =>
      `/connections/${id(args.connectionId, "connectionId")}/redis/health`,
  },
  test_redis_connection: {
    method: "POST",
    path: () => "/connections/redis/test",
    // RedisVersionRequest uses camelCase (`isTlsEnabled`) on the web API.
    body: (args) => args,
  },
};
