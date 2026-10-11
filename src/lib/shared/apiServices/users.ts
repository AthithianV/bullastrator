import type { WebCommand } from ".";

// ../../../../crates/bullastrator_web/src/routes/user.rs
export const USER_ROUTES: Record<string, WebCommand> = {
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
    body: (args) => ({ token: args.token }),
  },
};
