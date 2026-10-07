import { snakeCase, type WebCommand } from ".";

// ../../../../crates/bullastrator_web/src/routes/user.rs
export const USER_ROUTES: Record<string, WebCommand> = {
  register: {
    method: "POST",
    path: () => "/auth/register",
    body: (args) => snakeCase(args.data),
  },
  login: {
    method: "POST",
    path: () => "/auth/login",
    body: (args) => snakeCase(args.data),
  },
  logout: {
    method: "POST",
    path: () => "/auth/logout",
    body: (args) => snakeCase({ token: args.token }),
  },
};
