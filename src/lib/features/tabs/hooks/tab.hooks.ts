import {
  createMutation,
  createQuery,
  useQueryClient,
} from "@tanstack/svelte-query";
import { toast } from "svelte-sonner";
import type {
  CreateTabModel,
  ReadTabModel,
  UpdateTabModel,
} from "../interface/tab.types";
import { invokeWrapper } from "shared/helpers/invokeWrapper";

export const TAB_QUERY_KEY = ["tabs"];

// --- QUERY HOOKS (READ) ---

/**
 * 🎣 Hook to fetch and cache ALL tabs for the active workspace.
 * Calls the Rust command `get_all_tabs`.
 */
export const useGetTabs = () => {
  return createQuery(() => ({
    queryKey: [...TAB_QUERY_KEY],
    queryFn: () =>
      invokeWrapper<ReadTabModel[]>(
        "get_all_tabs",
        {},
        {
          shouldToast: false,
          errorMessage: "Failed to fetch tabs",
          shouldLogResult: false,
        },
      ),
  }));
};

/**
 * 🎣 Hook to fetch a SINGLE tab by ID.
 * Calls the Rust command `get_tab`.
 */
export const useGetTabById = (id: string) => {
  return createQuery(() => ({
    queryKey: [...TAB_QUERY_KEY, id],
    queryFn: () => invokeWrapper<ReadTabModel | null>("get_tab", { id }),
    enabled: !!id, // Only run if ID is valid
  }));
};

/**
 * 🎣 Hook to fetch a SINGLE Actve Tab Id.
 * Calls the Rust command `get_tab`.
 */
export const useGetActiveTabId = () => {
  return createQuery(() => ({
    queryKey: [...TAB_QUERY_KEY, "ACTIVE_TAB"],
    queryFn: () => invokeWrapper<string | null>("get_active_tab"),
  }));
};

// --- MUTATION HOOKS (CREATE, UPDATE, DELETE) ---

/**
 * 🛠️ Hook to create a new tab.
 * Calls the Rust command `create_tab`.
 * Requires both the deterministic `id` and the `data` payload.
 */
export const useCreateTab = () => {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: ({ id, data }: { id: string; data: CreateTabModel }) =>
      invokeWrapper<ReadTabModel>("create_tab", {
        id,
        data,
      }),
    onSuccess: (updatedTab) => {
      // Invalidate the list so the new tab appears
      queryClient.invalidateQueries({ queryKey: TAB_QUERY_KEY });
      queryClient.setQueryData(
        TAB_QUERY_KEY,
        (oldData: ReadTabModel[] | undefined) => {
          if (!oldData) return [];
          return oldData.map((tab) =>
            tab.id === updatedTab.id ? updatedTab : tab,
          );
        },
      );

      queryClient.invalidateQueries({
        queryKey: [...TAB_QUERY_KEY, "ACTIVE_TAB"],
      });
    },
    onError: (err: Error) => {
      console.error(err);
      toast.error(`Failed to create tab: ${err.message}`);
    },
  }));
};

/**
 * 🛠️ Hook to update an existing tab.
 * Calls the Rust command `update_tab`.
 */
export const useUpdateTab = () => {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: ({ id, data }: { id: string; data: UpdateTabModel }) =>
      invokeWrapper<ReadTabModel>("update_tab", { id, data }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: TAB_QUERY_KEY });
      queryClient.invalidateQueries({
        queryKey: [...TAB_QUERY_KEY, "ACTIVE_TAB"],
        exact: true, // Be specific
        refetchType: "all",
      });
    },
    onError: (err: Error) => {
      toast.error(`Failed to update tab: ${err.message}`);
    },
  }));
};

/**
 * 🛠️ Hook to delete a tab.
 * Calls the Rust command `delete_tab`.
 */
export const useDeleteTab = () => {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: (id: string) => invokeWrapper<void>("delete_tab", { id }),
    onSuccess: (_result, deletedId) => {
      // Optimistically remove from list
      queryClient.setQueryData<ReadTabModel[]>(TAB_QUERY_KEY, (oldData) => {
        return oldData ? oldData.filter((tab) => tab.id !== deletedId) : [];
      });
    },
    onError: (err: Error) => {
      toast.error(`Failed to delete tab: ${err.message}`);
    },
  }));
};

/**
 * 🛠️ Hook to reorder tabs (Drag & Drop).
 * Calls the Rust command `reorder_tabs`.
 */
export const useReorderTabs = () => {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: (orderedIds: string[]) =>
      invokeWrapper<void>("reorder_tabs", { orderedIds }),

    onMutate: async (orderedIds) => {
      // Cancel outgoing refetches so they don't overwrite our optimistic update
      await queryClient.cancelQueries({ queryKey: TAB_QUERY_KEY });

      // Snapshot the previous value
      const previousTabs =
        queryClient.getQueryData<ReadTabModel[]>(TAB_QUERY_KEY);

      // Optimistically update to the new order
      if (previousTabs) {
        // Sort the existing data based on the new ID array
        const sortedTabs = [...previousTabs].sort((a, b) => {
          return orderedIds.indexOf(a.id) - orderedIds.indexOf(b.id);
        });
        queryClient.setQueryData(TAB_QUERY_KEY, sortedTabs);
      }

      return { previousTabs };
    },
    onError: (err: Error, _newOrder, context) => {
      // Rollback on error
      if (context?.previousTabs) {
        queryClient.setQueryData(TAB_QUERY_KEY, context.previousTabs);
      }
      toast.error(`Failed to reorder tabs: ${err.message}`);
    },
    onSettled: () => {
      // Always refetch after error or success to ensure sync with DB
      queryClient.invalidateQueries({ queryKey: TAB_QUERY_KEY });
    },
  }));
};

/**
 * 🛠️ Hook to set active tab.
 * Calls the Rust command `set_active_tab`.
 */
export const useSetActiveTab = () => {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: (tabId: string | null) =>
      invokeWrapper<void>("set_active_tab", { tabId }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: TAB_QUERY_KEY });
      queryClient.invalidateQueries({
        queryKey: [...TAB_QUERY_KEY, "ACTIVE_TAB"],
        exact: true, // Be specific
        refetchType: "all",
      });
    },
  }));
};
