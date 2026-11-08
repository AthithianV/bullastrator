import type { WebCommand } from ".";

// ../../../../crates/bullastrator_web/src/routes/user.rs
export const USER_ROUTES: Record<string, WebCommand> = {
  get_all_users: {
    method: "GET",
    path: () => "/users",
  },
  get_user_roles: {
    method: "GET",
    path: () => "/roles",
    query: (args) => ({ userId: args.userId }),
  },
  get_role_options: {
    method: "GET",
    path: () => "/roles/options",
    query: (args) => ({ entityType: args.entityType }),
  },
  create_user_role: {
    method: "POST",
    path: () => "/roles",
    body: (args) => args.data,
  },
  update_user_role: {
    method: "PATCH",
    path: (args) => `/roles/${encodeURIComponent(args.id)}`,
    body: (args) => ({ role: args.role }),
  },
  delete_user_role: {
    method: "DELETE",
    path: (args) => `/roles/${encodeURIComponent(args.id)}`,
  },
  check_session: {
    method: "GET",
    path: () => "/auth/session",
  },
  register: {
    method: "POST",
    path: () => "/auth/register",
    body: (args) => args.data,
  },
  login: {
    method: "POST",
    path: () => "/auth/login",
    body: (args) => args.data,
  },
  logout: {
    method: "POST",
    path: () => "/auth/logout",
  },
};
