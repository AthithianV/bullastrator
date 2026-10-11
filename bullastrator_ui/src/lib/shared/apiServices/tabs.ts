import { id, type WebCommand } from ".";

export const TAB_ROUTES: Record<string, WebCommand> = {
  get_tab: {
    method: "GET",
    path: (args) => `/tabs/${id(args.id, "id")}`,
  },
  get_all_tabs: {
    method: "GET",
    path: () => "/tabs",
  },
  create_tab: {
    method: "POST",
    path: () => "/tabs",
    body: (args) => args,
  },
  update_tab: {
    method: "PATCH",
    path: (args) => `/tabs/${id(args.id, "id")}`,
    body: (args) => args.data,
  },
  delete_tab: {
    method: "DELETE",
    path: (args) => `/tabs/${id(args.id, "id")}`,
  },
  set_active_tab: {
    method: "POST",
    path: (args) => `/tabs/${id(args.tabId, "tabId")}/select`,
  },
  get_active_tab: {
    method: "GET",
    path: () => "/tabs/active",
  },
  reorder_tabs: {
    method: "POST",
    path: () => "/tabs/reorder",
    body: (args) => args.data,
  },
};
