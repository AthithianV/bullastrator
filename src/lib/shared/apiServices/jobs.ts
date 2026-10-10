import { id, type CommandArgs, type WebCommand } from ".";

const withoutRouteArgs = (args: CommandArgs, ...keys: string[]) => {
  const body = { ...args };
  for (const key of keys) delete body[key];
  return body;
};

export const JOB_ROUTES: Record<string, WebCommand> = {
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
    body: (args) => ({ queueNames: [args.queueName] }),
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
    body: (args) => ({ jobId: args.jobId, data: args.jobData }),
  },
  retry_failed_jobs: {
    method: "POST",
    path: (args) =>
      `/connections/${id(args.connectionId, "connectionId")}/queues/${id(args.queueName, "queueName")}/jobs/retry`,
    body: (args) => ({
      jobIds: args.jobIds,
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
    body: (args) => ({ jobIds: args.jobIds }),
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
      jobIds: args.jobIds,
      removeChildren: args.removeChildren ?? false,
    }),
  },
  delete_all_jobs: {
    method: "POST",
    path: (args) =>
      `/connections/${id(args.connectionId, "connectionId")}/queues/${id(args.queueName, "queueName")}/jobs/delete-state`,
    body: (args) => ({
      state: args.status,
      removeChildren: args.removeChildren ?? false,
    }),
  },
};
