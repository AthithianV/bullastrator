import { id, type WebCommand } from ".";

export const QUEUE_ROUTES: Record<string, WebCommand> = {
  sync_all_queue_names: {
    method: "GET",
    path: (args) => `/queues/sync/${id(args.connectionId, "connectionId")}`,
  },
  get_queue_details: {
    method: "GET",
    path: (args) =>
      `/queues/${id(args.connectionId, "connectionId")}/${id(args.queueName, "queueName")}`,
  },
  pause_queue: {
    method: "POST",
    path: (args) =>
      `/queues/${id(args.connectionId, "connectionId")}/${id(args.queueName, "queueName")}/pause`,
    body: (args) => ({ paused: args.shouldPause }),
  },
};
