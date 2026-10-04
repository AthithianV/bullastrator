import { id, type WebCommand } from ".";

export const QUEUE_ROUTES: Record<string, WebCommand> = {
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
};
