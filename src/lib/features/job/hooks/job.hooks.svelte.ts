import type {
  JobDetails,
  JobSearchResult,
  JobStatus,
  QueueJobCounts,
} from "job/interface/job.types";
import type {
  AddJobPayload,
  DeleteJobsPayload,
  GetJobsPayload,
  PromoteJobsPayload,
  RetryJobPayload,
  UpdateJobPayload,
} from "job/interface/jobPayload.types";
import { tabStore } from "tabs/store/tabStore.svelte";

import {
  createInfiniteQuery,
  createMutation,
  createQuery,
  useQueryClient,
} from "@tanstack/svelte-query";
import { confirm } from "@tauri-apps/plugin-dialog";
import { QUEUE_QUERY_KEY } from "queue/hooks/queue.hooks";
import { useQueueState } from "queue/store/queueContext.svelte";
import { invokeWrapper } from "shared/helpers/invokeWrapper";
import { useDebounce } from "shared/hooks/useDebounce.svelte";
import { toast } from "svelte-sonner";

export const JOB_QUERY_KEY = ["jobs"];

export const useGetJobById = () => {
  const queueState = useQueueState();

  const getDebouncedJobId = useDebounce(
    () => queueState.active.selectedJob,
    300,
  );

  return createQuery(() => {
    const debouncedId = getDebouncedJobId();

    const payload = {
      queueName: queueState.queueName,
      connectionId: queueState.connectionId,
      jobId: debouncedId,
      status: queueState.currentStatus,
    };

    const isEnabled =
      !!debouncedId &&
      !!queueState.queueName &&
      !!queueState.connectionId &&
      !!queueState.currentStatus;

    return {
      queryKey: [
        ...JOB_QUERY_KEY,
        "JOB_BY_ID",
        queueState.connectionId,
        queueState.queueName,
        debouncedId,
        queueState.currentStatus,
        queueState.tabId,
      ],
      queryFn: () =>
        invokeWrapper<JobDetails>(
          "get_jobs_by_id",
          { ...payload },
          {
            shouldToast: false,
            errorMessage: "Failed to fetch job with Id " + debouncedId,
            shouldLogResult: false,
          },
        ),
      retry: false,
      enabled: isEnabled,
    };
  });
};

export const useGetJobData = () => {
  const queueState = useQueueState();

  return createQuery(() => {

    const payload = {
      queueName: queueState.queueName,
      connectionId: queueState.connectionId,
      jobIds: queueState.active.selectedJobIds,
      status: queueState.currentStatus,
    };

    return {
      queryKey: [
        ...JOB_QUERY_KEY,
        "JOB_DATA",
        queueState.connectionId,
        queueState.queueName,
        queueState.currentStatus,
        queueState.tabId,
      ],
      queryFn: () =>
        invokeWrapper<JobDetails>(
          "get_jobs_data",
          { ...payload },
          {
            shouldToast: false,
            errorMessage: "Failed to fetch job data",
            shouldLogResult: false,
          },
        ),
      retry: false,
      enabled: false,
    };
  });
};

export const useGetJobLog = () => {
  const queueState = useQueueState();

  return createQuery(() => {
    const jobId = queueState.active.selectedJob;

    const payload = {
      queueName: queueState.queueName,
      connectionId: queueState.connectionId,
      jobId,
      status: queueState.currentStatus,
    };

    const isEnabled =
      !!jobId &&
      !!queueState.queueName &&
      !!queueState.connectionId &&
      !!queueState.currentStatus;

    return {
      queryKey: [
        ...JOB_QUERY_KEY,
        "JOB_LOGS",
        queueState.connectionId,
        queueState.queueName,
        jobId,
        queueState.currentStatus,
        queueState.tabId,
      ],
      queryFn: () =>
        invokeWrapper<string[]>(
          "get_job_logs",
          { ...payload },
          {
            shouldToast: false,
            errorMessage: "Failed to fetch job with Id " + jobId,
            shouldLogResult: false,
          },
        ),
      retry: false,
      enabled: isEnabled,
    };
  });
};

export const useGetJobCountInQueue = () => {
  const queueState = useQueueState();

  return createQuery(() => {
    return {
      queryKey: [
        "JOB_COUNT",
        queueState.queueName,
        queueState.tabId,
        queueState.connectionId,
      ],
      queryFn: async () =>
        invokeWrapper<QueueJobCounts>(
          "get_job_count_in_queue",
          {
            queueName: queueState.queueName,
            connectionId: queueState.connectionId,
          },
          {
            shouldToast: false,
            shouldLogResult: false,
          },
        ),
      staleTime: 5 * 1000,
      enabled: !!queueState.queueName && !!queueState.connectionId,
    };
  });
};

export const useAddJob = () => {
  const queueState = useQueueState();
  const refetchJobs = useRefetchJobsinQueue();

  return createMutation(() => ({
    mutationFn: async (jobs: AddJobPayload[]) => {
      return invokeWrapper<String[]>(
        "add_job_to_queue",
        {
          jobs,
          connectionId: queueState.connectionId,
          queueName: queueState.queueName,
        },
        {
          shouldToast: true,
          successMessage: "Jobs added successfully",
          errorMessage: "Failed to add jobs",
          shouldLogResult: true,
        },
      );
    },
    onSuccess: async () => {
      await refetchJobs();
    },
    onMutate: () => {
      queueState.actionInProgress = "ADD";
    },
    onSettled: () => {
      queueState.actionInProgress = null;
    },
  }));
};

export const usePromoteJobs = () => {
  const refetchJobs = useRefetchJobsinQueue();
  const queueState = useQueueState();

  return createMutation(() => ({
    mutationFn: async (isAll: boolean = false) => {
      const payload: PromoteJobsPayload = {
        queueName: queueState.queueName,
        connectionId: queueState.connectionId,
      };

      const count = isAll
        ? queueState.jobCounts.delayed
        : queueState.active.selectedJobIds.length;

      let confirmationMesssage = "Are you sure want to promote ";
      confirmationMesssage += isAll ? "all" : "selected";
      confirmationMesssage += ` (${count})`;
      confirmationMesssage += " jobs? ";

      const confirmation = await confirm(
        `This action cannot be reverted. ${confirmationMesssage}`,
        { title: "Promote", kind: "warning" },
      );

      if (!confirmation) throw new Error("Cancelled");

      queueState.actionInProgress = "PROMOTE";
      try {
        if (isAll) {
          return await invokeWrapper("promote_all_jobs", { ...payload });
        }

        return await invokeWrapper("promote_jobs", {
          ...payload,
          jobIds: queueState.active.selectedJobIds,
        });
      } catch (e: any) {
        console.error(e);
        throw e;
      }
    },
    onSuccess: async (_, variables) => {
      toast.success("Jobs promoted successfully");
      queueState.active.selectedJobIds = [];
      queueState.active.selectedJob = null;
      await refetchJobs();
    },
    onError: (err: any) => {
      if (err.message === "Cancelled") return;
      toast.error(err.message || "Failed to promote jobs");
    },
    onSettled: () => {
      queueState.actionInProgress = null;
    },
  }));
};

export const useDeleteJobs = () => {
  const queueState = useQueueState();
  const refetchJobs = useRefetchJobsinQueue();

  return createMutation(() => ({
    mutationFn: async (isAll: boolean) => {
      let payload: DeleteJobsPayload = {
        queueName: queueState.queueName,
        connectionId: queueState.connectionId,
        status: queueState.currentStatus,
      };

      let count = isAll
        ? queueState.jobCounts[queueState.currentStatus]
        : queueState.active.selectedJobIds.length;
      let confirmationMesssage = "Are you sure want to delete ";
      confirmationMesssage += isAll ? "all" : "selected";
      confirmationMesssage += ` (${count})`;
      confirmationMesssage += " jobs? ";

      const confirmation = await confirm(
        `This action cannot be reverted. ${confirmationMesssage}`,
        { title: "Delete job(s)", kind: "warning" },
      );

      if (!confirmation) throw new Error("Cancelled");

      queueState.actionInProgress = "DELETE";
      try {
        if (isAll) {
          return await invokeWrapper("delete_all_jobs", { ...payload });
        }
        return await invokeWrapper("delete_jobs", {
          ...payload,
          jobIds: queueState.active.selectedJobIds,
        });
      } catch (e: any) {
        console.error(e);
        throw e;
      }
    },
    onSuccess: async (_, variables) => {
      toast.success("Jobs deleted successfully");
      queueState.active.selectedJobIds = [];
      queueState.active.selectedJob = null;
      await refetchJobs();
    },
    onError: (err: any) => {
      if (err.message === "Cancelled") return;
      console.error(err);
      toast.error(err.message || "Failed to delete jobs");
    },
    onSettled: () => {
      queueState.actionInProgress = null;
    },
  }));
};

export const useGetJobsInQueues = () => {
  const queueState = useQueueState();

  return createInfiniteQuery<JobSearchResult, Error>(() => {
    const payload: GetJobsPayload = {
      queueName: queueState.queueName,
      status: queueState.currentStatus,
      limit: queueState.itemsPerPage,
      connectionId: queueState.connectionId,
      filters: queueState.searchFilters,
    };

    return {
      queryKey: [
        "get_jobs",
        queueState.connectionId,
        queueState.queueName,
        queueState.currentStatus,
        queueState.tabId,
        JSON.stringify(queueState.searchFilters),
      ],
      initialPageParam: 0,
      queryFn: ({ pageParam, signal }) => {
        if (signal.aborted) throw new Error("Fetch aborted by user");
        return invokeWrapper<JobSearchResult>(
          "get_jobs_in_queue",
          { ...payload, cursor: pageParam },
          {
            shouldToast: false,
            errorMessage: "Failed to search jobs",
          },
        );
      },
      getNextPageParam: (lastPage) => {
        // Only return the next cursor if the backend says there's more.
        // If hasMore is 0, returning null stops the infinite loop.
        return lastPage.hasMore > 0 ? lastPage.nextCursor : null;
      },
      enabled: tabStore.activeTab === queueState.tabId,
      staleTime: 5 * 1000,
      placeholderData: (previousData) => previousData,
    };
  });
};

export const useAbortJobsFetch = () => {
  const queryClient = useQueryClient();
  const queueState = useQueueState();

  return async () => {
    await queryClient.cancelQueries({
      queryKey: [
        "get_jobs",
        queueState.connectionId,
        queueState.queueName,
        queueState.currentStatus,
        queueState.tabId,
        JSON.stringify(queueState.searchFilters),
      ],
    });
  };
};

export const useRefetchJobsinQueue = () => {
  const queryClient = useQueryClient();
  const queueState = useQueueState();

  return async () => {
    await queryClient.resetQueries({
      queryKey: [
        "get_jobs",
        queueState.connectionId,
        queueState.queueName,
      ],
    });

    queryClient.invalidateQueries({
      queryKey: [
        ...QUEUE_QUERY_KEY,
        "QUEUE_DETAILS",
        queueState.connectionId,
        queueState.queueName,
        queueState.tabId,
      ],
    });

    await queryClient.invalidateQueries({
      queryKey: [
        "JOB_COUNT",
        queueState.queueName,
        queueState.tabId,
        queueState.connectionId,
      ],
    });
  };
};

export const useUpdateJobData = () => {
  const queryClient = useQueryClient();

  const queueState = useQueueState();

  return createMutation(() => ({
    mutationFn: async (payload: UpdateJobPayload) => {
      try {
        const result = await invokeWrapper("update_job_data", {
          ...payload,
          connectionId: queueState.connectionId,
          queueName: queueState.queueName,
        });
        return { result, payload };
      } catch (e: any) {
        console.error(e);
        throw e;
      }
    },
    onSuccess: ({ result, payload: updatedJobPayload }) => {
      toast.success("Job updated successfully");
      queryClient.invalidateQueries({
        queryKey: [
          ...JOB_QUERY_KEY,
          "JOB_BY_ID",
          queueState.queueName,
          updatedJobPayload.jobId,
        ],
      });
    },
    onError: (err: any) => {
      console.error(err);
      toast.error(err.message || "Failed to update job");
    },
    onMutate: () => {
      queueState.actionInProgress = "UPDATE";
    },
    onSettled: () => {
      queueState.actionInProgress = null;
    },
  }));
};

export const useRetryJob = () => {
  const refetchJobs = useRefetchJobsinQueue();
  const queueState = useQueueState();

  return createMutation(() => ({
    mutationFn: async ({
      isAll,
      strategy,
      retryJobStatus
    }: {
      isAll: boolean;
      strategy: "ToBack" | "ToFront";
      retryJobStatus: JobStatus
    }) => {
      const payload: RetryJobPayload = {
        queueName: queueState.queueName,
        connectionId: queueState.connectionId,
        strategy,
        retryJobStatus: retryJobStatus === 'completed' ? 'completed' : 'failed',
      };

      let count = isAll
        ? queueState.jobCounts.failed
        : queueState.active.selectedJobIds.length;

      let confirmationMesssage = "Are you sure want to retry ";
      confirmationMesssage += isAll ? "all" : "selected";
      confirmationMesssage += ` (${count})`;
      confirmationMesssage += " jobs? ";

      const confirmation = await confirm(
        `This action cannot be reverted. ${confirmationMesssage}`,
        { title: "Retry", kind: "warning" },
      );

      if (!confirmation) throw new Error("Cancelled");

      queueState.actionInProgress = "RETRY";

      try {
        if (isAll) {
          return await invokeWrapper("retry_all_failed_jobs", { ...payload });
        }
        return await invokeWrapper(
          "retry_failed_jobs",
          {
            ...payload,
            jobIds: queueState.active.selectedJobIds,
          },
          { shouldLogResult: true },
        );
      } catch (e: any) {
        throw e;
      }
    },
    onSuccess: async () => {
      toast.success("Job(s) retried successfully");
      queueState.active.selectedJobIds = [];
      queueState.active.selectedJob = null;
      await refetchJobs();
    },
    onError: (err: any) => {
      if (err.message === "Cancelled") return;
      console.error(err);
      toast.error(err.message || "Failed to retry job");
    },
    onSettled: () => {
      queueState.actionInProgress = null;
    },
  }));
};
