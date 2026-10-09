import { id, snakeCase, type WebCommand } from ".";

export const WORKSPACE_ROUTES: Record<string, WebCommand> = {
  get_all_workspaces: {
    method: "GET",
    path: (args) => `/workspaces`,
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
