import { createMutation, createQuery, useQueryClient } from "@tanstack/svelte-query";
import { invokeWrapper } from "shared/helpers/invokeWrapper";
import { useFolderState } from "../store/folder.context.svelte";
import type { ReadQueueWithCounts } from "queue/interface/queue.types";
import type {
  CreateFolderModel,
  ReadFolderModel,
  ReadFolderWithQueuesModel,
} from "../interface/folder.types";
import { TAB_QUERY_KEY, useSetActiveTab } from "tabs/hooks/tab.hooks";
import { confirm } from "@tauri-apps/plugin-dialog";
import { tabStore } from "$lib/features/tabs/store/tabStore.svelte";
import type { ReadTabModel } from "tabs/interface/tab.types";

// Unique keys for caching
export const FOLDER_QUERY_KEY = ["folder"];

// --- QUERY HOOKS (READ) ---

/**
 * 🎣 Hook to fetch all folders for a specific connection.
 */
export const useGetAllFolders = (
  connectionId: string | null | (() => string | null),
) => {
  return createQuery(() => ({
    queryKey: [
      ...FOLDER_QUERY_KEY,
      "list",
      typeof connectionId === "function" ? connectionId() : connectionId,
    ],
    queryFn: () => {
      const currentConnectionId =
        typeof connectionId === "function" ? connectionId() : connectionId;
      return invokeWrapper<ReadFolderWithQueuesModel[]>("get_all_folders", {
        connectionId: currentConnectionId,
      });
    },
    enabled: !!(
      typeof connectionId === "function" ? connectionId() : connectionId
    ),
  }));
};

/** Fetch folders for all connections in the active workspace. */
export const useGetAllFoldersByConnections = (
  connectionIds: () => string[],
) => {
  return createQuery(() => {
    const ids = connectionIds();

    return {
      queryKey: [...FOLDER_QUERY_KEY, "workspace", ids],
      queryFn: async () => {
        const folders = await Promise.all(
          ids.map((connectionId) =>
            invokeWrapper<ReadFolderWithQueuesModel[]>("get_all_folders", {
              connectionId,
            }),
          ),
        );

        return folders.flat();
      },
      enabled: ids.length > 0,
    };
  });
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
        },
      ),
    onSuccess: (newFolder) => {
      queryClient.invalidateQueries({
        queryKey: [...FOLDER_QUERY_KEY, "list", newFolder.connectionId],
      });
      queryClient.invalidateQueries({ queryKey: TAB_QUERY_KEY });
      queryClient.invalidateQueries({
        queryKey: [...TAB_QUERY_KEY, "ACTIVE_TAB"],
      });
      tabStore.setActiveTab(newFolder.id);
      queryClient.setQueryData([...TAB_QUERY_KEY, "ACTIVE_TAB"], newFolder.id);
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
      queryClient.setQueryData<ReadTabModel[]>(TAB_QUERY_KEY, (tabs) =>
        tabs?.map((tab) =>
          tab.id === updatedFolder.id
            ? { ...tab, title: updatedFolder.title }
            : tab,
        ),
      );
      queryClient.setQueryData<ReadTabModel | null>(
        [...TAB_QUERY_KEY, updatedFolder.id],
        (tab) => (tab ? { ...tab, title: updatedFolder.title } : tab),
      );
    },
  }));
};

/**
 * 🚀 Hook to delete a folder.
 */
export const useDeleteFolder = () => {
  const queryClient = useQueryClient();
  const { mutateAsync: setActiveTab } = useSetActiveTab();

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
        },
      );
    },
    onSuccess: (_result, deletedFolderId) => {
      const tabs = queryClient.getQueryData<ReadTabModel[]>(TAB_QUERY_KEY) ?? [];
      const { remaining, nextId } = tabStore.selectAfterRemoval(
        [deletedFolderId],
        tabs,
      );
      queryClient.setQueryData(TAB_QUERY_KEY, remaining);
      queryClient.setQueryData([...TAB_QUERY_KEY, "ACTIVE_TAB"], nextId);
      queryClient.invalidateQueries({ queryKey: FOLDER_QUERY_KEY });
      queryClient.invalidateQueries({ queryKey: TAB_QUERY_KEY });
      queryClient.invalidateQueries({
        queryKey: [...TAB_QUERY_KEY, "ACTIVE_TAB"],
        exact: true, // Be specific
        refetchType: "all",
      });
      if (nextId) {
        void setActiveTab(nextId);
      }
    },
  }));
};

/**
 * 🚀 Hook to add/remove a queue from a folder (Toggle).
 */
export const useToggleQueueInFolder = () => {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: ({ folderId, queueId }: { folderId: string; queueId: string }) =>
      invokeWrapper<boolean>("toggle_queue_in_folder", { folderId, queueId }),
    onSuccess: (_, variables) => {
      // Invalidate the specific folder's queue list
      queryClient.invalidateQueries({
        queryKey: [...FOLDER_QUERY_KEY, "queues", variables.folderId],
      });
      queryClient.invalidateQueries({ queryKey: [ ...FOLDER_QUERY_KEY,
      "list"] });
    },
  }));
};

/**
 * 🚀 Hook to reorder queues within a folder.
 */
export const useReorderFolderQueues = () => {
  return createMutation(() => ({
    mutationFn: ({ folderId, orderedQueueIds }: { folderId: string; orderedQueueIds: string[] }) =>
      invokeWrapper<void>("reorder_folder_queues", {
        folderId,
        orderedQueueIds,
      }),
  }));
};

/**
 * 🚀 Hook to reorder folders within a connection.
 */
export const useReorderFolders = () => {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: ({ connectionId, orderedFolderIds }: {
      connectionId: string;
      orderedFolderIds: string[];
    }) =>
      invokeWrapper<void>("reorder_folders", {
        connectionId,
        orderedFolderIds,
      }),
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({
        queryKey: [...FOLDER_QUERY_KEY, "list", variables.connectionId],
      });
    },
  }));
};
