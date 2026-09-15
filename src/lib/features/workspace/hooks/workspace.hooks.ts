import {
  createMutation,
  createQuery,
  useQueryClient,
} from "@tanstack/svelte-query";
import { invoke } from "@tauri-apps/api/core";
import { invokeWrapper } from "shared/helpers/invokeWrapper";
import { toast } from "svelte-sonner";
import type {
  CreateWorkspaceModel,
  ReadWorkspaceModel,
  UpdateWorkspaceModel,
} from "../interface/workspace.types";

const WORKSPACE_QUERY_KEY = ["workspaces"];

// --- QUERY HOOKS (READ) ---

/**
 * 🎣 Hook to fetch ALL workspaces.
 */
export const useGetWorkspaces = () => {
  return createQuery(() => ({
    queryKey: WORKSPACE_QUERY_KEY,
    queryFn: () => invokeWrapper<ReadWorkspaceModel[]>("get_all_workspaces"),
  }));
};

/**
* 🎣 Hook to fetch the DEFAULT workspace.
*/
// Command: file://./../../../../../src-tauri/src/commands/workspace_command.rs
export const useGetActiveWorkspace = () => {
  return createQuery(() => ({
    queryKey: [...WORKSPACE_QUERY_KEY, "active_workspace"],
    queryFn: () => invoke<ReadWorkspaceModel>("get_active_workspace"),
  }));
};

/**
 * 🎣 Hook to fetch a workspace by ID.
 */
export const useGetWorkspaceById = (id: string | null) => {
  return createQuery(() => ({
    queryKey: [...WORKSPACE_QUERY_KEY, id],
    queryFn: () =>
      invokeWrapper<ReadWorkspaceModel | null>("get_workspace_by_id", { id }),
    enabled: !!id,
  }));
};

// --- MUTATION HOOKS (CREATE, UPDATE, DELETE) ---

/**
 * 🛠️ Hook to create a new workspace.
 */
export const useCreateWorkspace = () => {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: (data: CreateWorkspaceModel) =>
      invokeWrapper<ReadWorkspaceModel>("create_workspace", { data }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: WORKSPACE_QUERY_KEY });
      toast.success("Workspace created successfully");
    },
    onError: (err: string) => {
      toast.error(err);
    },
  }));
};

/**
 * 🛠️ Hook to update an existing workspace.
 */
export const useUpdateWorkspace = () => {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: ({ id, data }: { id: string; data: UpdateWorkspaceModel }) =>
      invokeWrapper<ReadWorkspaceModel>("update_workspace", { id, data }),
    onSuccess: (updated) => {
      // Update individual cache
      queryClient.setQueryData([...WORKSPACE_QUERY_KEY, updated.id], updated);
      // Invalidate list to ensure sorting/default flags are correct
      queryClient.invalidateQueries({ queryKey: WORKSPACE_QUERY_KEY });
      toast.success("Workspace updated");
    },
    onError: (err: string) => {
      toast.error(err);
    },
  }));
};

/**
 * 🛠️ Hook to Change active workspace.
 */
export const useSetActiveWorkspace = () => {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: (workspaceId: string) =>
      invokeWrapper<string>("set_active_workspace", { workspaceId }),
    onSuccess: (_msg) => {
      queryClient.cancelQueries();
      queryClient.invalidateQueries();
    },
    onError: (err: string) => {
      toast.error(err);
    },
  }));
};

export const useRefreshSession = () => {
  const queryClient = useQueryClient();

  return async () => {
    queryClient.cancelQueries();
    queryClient.invalidateQueries();
  };
};

/**
 * 🛠️  Hook to delete a workspace.
 */
export const useDeleteWorkspace = () => {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: (id: string) =>
      invokeWrapper<string>("delete_workspace", { id }),
    onSuccess: (_msg, deletedId) => {
      // Optimistic update: remove from list
      queryClient.setQueryData<ReadWorkspaceModel[]>(
        WORKSPACE_QUERY_KEY,
        (old) => (old ? old.filter((ws) => ws.id !== deletedId) : []),
      );
      queryClient.removeQueries({
        queryKey: [...WORKSPACE_QUERY_KEY, deletedId],
      });
      toast.success("Workspace deleted");
    },
    onError: (err: string) => {
      toast.error(err);
    },
  }));
};
