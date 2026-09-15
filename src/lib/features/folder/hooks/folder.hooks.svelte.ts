import {
  createMutation,
  createQuery,
  useQueryClient,
} from "@tanstack/svelte-query";
import { invokeWrapper } from "shared/helpers/invokeWrapper";
import { useFolderState } from "../store/folder.context.svelte";
import type { ReadQueueWithCounts } from "queue/interface/queue.types";
import type {
  CreateFolderModel,
  ReadFolderModel,
  ReadFolderWithQueuesModel,
} from "../interface/folder.types";
import { TAB_QUERY_KEY } from "tabs/hooks/tab.hooks";
import { confirm } from "@tauri-apps/plugin-dialog";
import { tabStore } from "$lib/features/tabs/store/tabStore.svelte";

// Unique keys for caching
export const FOLDER_QUERY_KEY = ["folder"];

// --- QUERY HOOKS (READ) ---

/**
 * 🎣 Hook to fetch all folders for a specific connection.
 */
export const useGetAllFolders = (connectionId: string | null) => {
  return createQuery(() => ({
    queryKey: [...FOLDER_QUERY_KEY, "list", connectionId],
    queryFn: () =>
      invokeWrapper<ReadFolderWithQueuesModel[]>("get_all_folders", {
        connectionId,
      }),
    enabled: !!connectionId,
  }));
};

/**
 * 🎣 Hook to fetch all folders for a specific connection.
 */
export const useGetFolderById = () => {
  let folderState = useFolderState();
  return createQuery(() => ({
    queryKey: [...FOLDER_QUERY_KEY, folderState.tabId],
    queryFn: () =>
      invokeWrapper<ReadFolderModel>("get_folder_by_id", {
        id: folderState.tabId,
      }),
  }));
};

/**
 * 🎣 Hook to fetch all queues linked to a specific folder.
 */
export const useGetQueuesOfFolder = () => {
  let folderState = useFolderState();
  return createQuery(() => ({
    queryKey: [...FOLDER_QUERY_KEY, "queues", folderState.tabId],
    queryFn: () =>
      invokeWrapper<ReadQueueWithCounts[]>("get_queues_for_folder", {
        folderId: folderState.tabId,
      }),
    enabled: !!folderState.tabId && folderState.tabId === tabStore.activeTab,
    refetchInterval: false,
  }));
};

// --- MUTATION HOOKS (WRITE) ---

/**
 * 🚀 Hook to create a new folder.
 */
export const useCreateFolder = () => {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: (data: CreateFolderModel) =>
      invokeWrapper<ReadFolderModel>(
        "create_folder",
        { data },
        {
          successMessage: "Folder created successfully",
          shouldToast: true,
        },
      ),
    onSuccess: (newFolder) => {
      queryClient.invalidateQueries({
        queryKey: [...FOLDER_QUERY_KEY, "list", newFolder.connectionId],
      });
    },
  }));
};

/**
 * 🚀 Hook to rename/update a folder.
 */
export const useUpdateFolder = () => {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: ({ id, title }: { id: string; title: string }) =>
      invokeWrapper<ReadFolderModel>("update_folder", { id, title }),
    onSuccess: (updatedFolder) => {
      queryClient.invalidateQueries({ queryKey: FOLDER_QUERY_KEY });
    },
  }));
};

/**
 * 🚀 Hook to delete a folder.
 */
export const useDeleteFolder = () => {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: async (id: string) => {
      const confirmation = await confirm(
        `This action cannot be reverted. Are you sure want to delete folder?`,
        { title: "Promote", kind: "warning" },
      );

      if (!confirmation) throw new Error("Cancelled");

      return invokeWrapper<string>(
        "delete_folder",
        { id },
        {
          successMessage: "Folder deleted",
          shouldToast: true,
        },
      );
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: FOLDER_QUERY_KEY });
      queryClient.invalidateQueries({ queryKey: TAB_QUERY_KEY });
      queryClient.invalidateQueries({
        queryKey: [...TAB_QUERY_KEY, "ACTIVE_TAB"],
        exact: true, // Be specific
        refetchType: "all",
      });
    },
  }));
};

/**
 * 🚀 Hook to add/remove a queue from a folder (Toggle).
 */
export const useToggleQueueInFolder = () => {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: ({
      folderId,
      queueId,
    }: {
      folderId: string;
      queueId: string;
    }) =>
      invokeWrapper<boolean>("toggle_queue_in_folder", { folderId, queueId }),
    onSuccess: (_, variables) => {
      // Invalidate the specific folder's queue list
      queryClient.invalidateQueries({
        queryKey: [...FOLDER_QUERY_KEY, "queues", variables.folderId],
      });
    },
  }));
};

/**
 * 🚀 Hook to reorder queues within a folder.
 */
export const useReorderFolderQueues = () => {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: ({
      folderId,
      orderedQueueIds,
    }: {
      folderId: string;
      orderedQueueIds: string[];
    }) =>
      invokeWrapper<void>("reorder_folder_queues", {
        folderId,
        orderedQueueIds,
      }),
  }));
};
