import {
  createMutation,
  createQuery,
  useQueryClient,
} from "@tanstack/svelte-query";
import { confirm } from "@tauri-apps/plugin-dialog";
import { useRefetchJobsinQueue } from "job/hooks/job.hooks.svelte";
import { useQueueState } from "queue/store/queueContext.svelte";
import { queueStore } from "queue/store/queueStore.svelte";
import { invokeWrapper } from "shared/helpers/invokeWrapper";
import type {
  ConnectionWithQueue,
  QueueDetails,
  ReadQueueModel,
} from "../interface/queue.types";

// Define the unique key for the connections list
export const QUEUE_QUERY_KEY = ["queue"];

// --- QUERY HOOKS (READ) ---

/**
 * 🎣 Hook to store all queue names in a connection.
 * Calls the Rust command `sync_all_queue_names`.
 */
export const useSyncAllQueues = () => {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: async (connectionId: string) =>
      invokeWrapper<void>(
        "sync_all_queue_names",
        { connectionId },
        {
          errorMessage: "Failed to sync queues, Check Connection",
          successMessage: "Queues synced successfully",
          shouldToast: true,
        },
      ),
    onMutate: (connectionId: string) => {
      queueStore.isSyncing = true;
      queueStore.connectionOnSync = connectionId;
      queueStore.isSyncing = true;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: [...QUEUE_QUERY_KEY, "ACTIVE_WORKSPACE"],
      });
    },
    onSettled: () => {
      queueStore.isSyncing = false;
      queueStore.connectionOnSync = null;
    },
    retry: false,
  }));
};

/**
 * 🎣 Hook to pause a queue in a connection.
 * Calls the Rust command `pause_queue`.
 */
export const usePauseQueue = () => {
  const queueState = useQueueState();
  const refetchJobs = useRefetchJobsinQueue();

  return createMutation(() => ({
    mutationFn: async (shouldPause: boolean) => {
      const confirmation = await confirm(
        `Are you should want to ${
          shouldPause ? "pause" : "resume"
        } this queue: ${queueState.queueName}`,
        { title: "Pause / Resume Queue", kind: "warning" },
      );

      if (!confirmation) throw new Error("Cancelled");

      return invokeWrapper<void>(
        "pause_queue",
        {
          connectionId: queueState.connectionId,
          queueName: queueState.queueName,
          shouldPause,
        },
        {
          errorMessage: "Failed to pause queues, Check Connection",
          successMessage: `Queues ${shouldPause ? "paused" : "resumed"} successfully`,
          shouldToast: true,
        },
      );
    },
    onSuccess: () => {
      refetchJobs();
    },
    retry: false,
  }));
};

/**
 * 🎣 Hook to fetch a queue details.
 * Calls the Rust command `get_queue_details`.
 * @param connection_id The ID of the connection to fetch.
 */
export const useGetQueuesDetails = () => {
  const queueState = useQueueState();

  return createQuery(() => ({
    queryKey: [
      ...QUEUE_QUERY_KEY,
      "QUEUE_DETAILS",
      queueState.connectionId,
      queueState.queueName,
      queueState.tabId,
    ],
    queryFn: () =>
      invokeWrapper<QueueDetails | null>("get_queue_details", {
        connectionId: queueState.connectionId,
        queueName: queueState.queueName,
      }),
  }));
};

/**
 * 🎣 Hook to fetch a All queues of SINGLE connection ID.
 * Calls the Rust command `get_all_queues_by_connection`.
 * @param connectionId The ID of the connection to fetch.
 */
export const useGetAllQueuesByConnection = (
  connectionId: string,
  isSyncing: boolean,
) => {
  return createQuery(() => ({
    queryKey: [...QUEUE_QUERY_KEY, connectionId],
    queryFn: () =>
      invokeWrapper<ReadQueueModel[] | null>(
        "get_all_queues_by_connection",
        {
          connectionId,
        },
        {
          shouldToast: false,
        },
      ),
    enabled: !!connectionId,
    refetchInterval: isSyncing ? 5000 : false,
    placeholderData: (previousData) => previousData,
  }));
};

/**
 * 🎣 Hook to fetch a All queues of Active Workspace.
 * Calls the Rust command `get_all_queues_by_connection`.
 */
export const useGetAllQueues = () => {
  return createQuery(() => ({
    queryKey: [...QUEUE_QUERY_KEY, "ACTIVE_WORKSPACE", queueStore.isSyncing],
    queryFn: () =>
      invokeWrapper<ConnectionWithQueue[]>("get_all_queues_by_workspace"),
    refetchInterval: queueStore.isSyncing ? 5000 : false,
    placeholderData: (previousData) => previousData,
    staleTime: 0,
  }));
};
