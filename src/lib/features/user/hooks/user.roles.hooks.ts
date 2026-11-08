import { createMutation, createQuery, useQueryClient } from "@tanstack/svelte-query";
import { invokeWrapper } from "shared/helpers/invokeWrapper";
import type { RoleEntityOption, UserRole } from "../interface";

export type RoleEntityType = UserRole["entityType"];
export const USER_ROLES_QUERY_KEY = ["user-roles"];
export const ROLE_OPTIONS_QUERY_KEY = ["role-options"];

export const useGetUserRoles = (
    userId: () => string | null,
    enabled: () => boolean = () => true,
) =>
    createQuery(() => ({
        queryKey: [...USER_ROLES_QUERY_KEY, userId()],
        queryFn: () =>
            invokeWrapper<UserRole[]>("get_user_roles", { userId: userId() }),
        enabled: enabled() && !!userId(),
    }));

export const useGetRoleOptions = (
    entityType: () => RoleEntityType,
    enabled: () => boolean = () => true,
) =>
    createQuery(() => ({
        queryKey: [...ROLE_OPTIONS_QUERY_KEY, entityType()],
        queryFn: () =>
            invokeWrapper<RoleEntityOption[]>("get_role_options", {
                entityType: entityType(),
            }),
        enabled: enabled(),
    }));

export const useCreateUserRole = (userId: () => string | null) => {
    const queryClient = useQueryClient();

    return createMutation(() => ({
        mutationFn: (data: object) =>
            invokeWrapper<UserRole>("create_user_role", { data }),
        onSuccess: () => {
            queryClient.invalidateQueries({
                queryKey: [...USER_ROLES_QUERY_KEY, userId()],
            });
        },
    }));
};

export const useDeleteUserRole = (userId: () => string | null) => {
    const queryClient = useQueryClient();

    return createMutation(() => ({
        mutationFn: (id: string) => invokeWrapper("delete_user_role", { id }),
        onSuccess: () => {
            queryClient.invalidateQueries({
                queryKey: [...USER_ROLES_QUERY_KEY, userId()],
            });
        },
    }));
};
