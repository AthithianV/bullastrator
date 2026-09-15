import {
  createMutation,
  createQuery,
  useQueryClient,
} from "@tanstack/svelte-query";
import { confirm } from "@tauri-apps/plugin-dialog";
import { queueStore } from "queue/store/queueStore.svelte";
import { invokeWrapper } from "shared/helpers/invokeWrapper";
import type {
  ConnectionHealth,
  CreateConnection,
  ReadConnection,
  UpdateConnection,
} from "../interface/connection.types";

// Define the unique key for the connections list
const CONNECTION_QUERY_KEY = ["connections"];

// --- QUERY HOOKS (READ) ---

/**
 * 🎣 Hook to fetch and cache ALL connections.
 * Calls the Rust command `get_all_connections`.
 */
export const useGetConnections = () => {
  return createQuery(() => ({
    queryKey: [...CONNECTION_QUERY_KEY],
    queryFn: () =>
      invokeWrapper<ReadConnection[]>(
        "get_all_connections",
        {},
        {
          shouldToast: false,
          errorMessage: "Failed to fetch connections",
          shouldLogResult: false,
        },
      ),
    refetchOnWindowFocus: false,
    staleTime: Infinity,
  }));
};

/**
 * 🎣 Hook to fetch a SINGLE connection by ID.
 * Calls the Rust command `get_connection`.
 * @param id The ID of the connection to fetch.
 */
export const useGetConnectionById = (id: string) => {
  return createQuery(() => ({
    queryKey: [...CONNECTION_QUERY_KEY, id],
    queryFn: () =>
      invokeWrapper<ReadConnection | null>(
        "get_connection",
        { id },
        {
          shouldToast: false,
          errorMessage: "Failed to fetch connections",
          shouldLogResult: false,
        },
      ),
    enabled: !!id,
  }));
};

// --- MUTATION HOOKS (CREATE, UPDATE, DELETE) ---

/**
 * 🛠️ Hook to create a new connection.
 * Calls the Rust command `create_connection`.
 */
export const useCreateConnection = () => {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: (data: CreateConnection) =>
      invokeWrapper<ReadConnection>(
        "create_connection",
        { data },
        {
          successMessage: "Connection created successfully",
          errorMessage: "Failed to create connection",
          shouldToast: true,
          shouldLogResult: false,
        },
      ),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: CONNECTION_QUERY_KEY });
      queueStore.isSyncing = false;
    },
    retry: false,
  }));
};

/**
 * 🛠️ Hook to update an existing connection.
 * Calls the Rust command `update_connection`.
 */
export const useUpdateConnection = () => {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: ({ id, data }: { id: string; data: UpdateConnection }) => {
      const promise = invokeWrapper<ReadConnection>(
        "update_connection",
        { id, data },
        {
          shouldToast: true,
          successMessage: "Connection updated successfully",
          errorMessage: "Failed to update connection",
          shouldLogResult: false,
        },
      );

      return promise;
    },
    onSuccess: (updatedConnection) => {
      queryClient.setQueryData<ReadConnection | null>(
        [...CONNECTION_QUERY_KEY, updatedConnection.id],
        updatedConnection,
      );

      queryClient.invalidateQueries({ queryKey: CONNECTION_QUERY_KEY });
    },
  }));
};

/**
 * 🛠️ Hook to delete a connection.
 * Calls the Rust command `delete_connection`.
 */
export const useDeleteConnection = () => {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: async (id: string) => {
      const confirmation = await confirm(
        "Do you really want to delete the connection?",
        {
          title: "Connection Deletion",
          kind: "warning",
        },
      );

      if (!confirmation) {
        throw new Error("USER_CANCELLED");
      }

      return invokeWrapper("delete_connection", { id });
    },
    onSuccess: (_result, deletedId) => {
      queryClient.setQueryData<ReadConnection[]>(
        CONNECTION_QUERY_KEY,
        (oldData) => {
          return oldData ? oldData.filter((conn) => conn.id !== deletedId) : [];
        },
      );
      queryClient.removeQueries({
        queryKey: [...CONNECTION_QUERY_KEY, deletedId],
      });
    },
  }));
};

/**
 * 🛠️ Hook to check connection status.
 * Calls the Rust command `delete_connection`.
 */
export const useCheckHealthForAllConnections = () => {
  return createQuery(() => ({
    queryKey: [...CONNECTION_QUERY_KEY, "all_conection_health_check"],
    refetchInterval: 5000,
    queryFn: () =>
      invokeWrapper<ConnectionHealth[]>(
        "check_health_for_all_connections",
        {},
        {
          shouldToast: false,
          errorMessage: "Failed to fetch connections",
          shouldLogResult: false,
        },
      ),
  }));
};

export const useManualConnectionsHealth = () => {
  return createMutation(() => ({
    mutationFn: (payload: CreateConnection) =>
      invokeWrapper(
        "test_redis_connection",
        { ...payload },
        {
          successMessage: "Connection tested successfully",
          errorMessage: "Connection Refused, Check Redis Credentials",
          shouldToast: true,
          shouldLogResult: false,
        },
      ),
  }));
};
