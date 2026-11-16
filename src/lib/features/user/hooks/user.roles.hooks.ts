import {
    createInfiniteQuery,
    createMutation,
    useQueryClient,
} from "@tanstack/svelte-query";
import { invokeWrapper } from "shared/helpers/invokeWrapper";
import type { Paginated, RoleEntityOption, UserRole } from "../interface";

export type RoleEntityType = UserRole["entityType"];
export const USER_ROLES_QUERY_KEY = ["user-roles"];
export const ROLE_OPTIONS_QUERY_KEY = ["role-options"];

export const useGetUserRoles = (
    userId: () => string | null,
    entityType: () => RoleEntityType,
    enabled: () => boolean = () => true,
) =>
    createInfiniteQuery<Paginated<UserRole>>(() => ({
        queryKey: [...USER_ROLES_QUERY_KEY, userId(), entityType()],
        initialPageParam: 0,
        queryFn: ({ pageParam }) =>
            invokeWrapper<Paginated<UserRole>>("get_user_roles", {
                userId: userId(),
                entityType: entityType(),
                offset: pageParam,
                limit: 50,
            }),
        getNextPageParam: (lastPage) =>
            lastPage.hasMore ? lastPage.nextOffset : undefined,
        enabled: enabled() && !!userId(),
    }));

export const useGetRoleOptions = (
    entityType: () => RoleEntityType,
    userId: () => string | null,
    search: () => string = () => "",
    enabled: () => boolean = () => true,
) =>
    createInfiniteQuery<Paginated<RoleEntityOption>>(() => ({
        queryKey: [...ROLE_OPTIONS_QUERY_KEY, entityType(), userId(), search()],
        initialPageParam: 0,
        queryFn: ({ pageParam }) =>
            invokeWrapper<Paginated<RoleEntityOption>>("get_role_options", {
                entityType: entityType(),
                userId: userId(),
                search: search(),
                offset: pageParam,
                limit: 50,
            }),
        getNextPageParam: (lastPage) =>
            lastPage.hasMore ? lastPage.nextOffset : undefined,
        enabled: enabled() && !!userId(),
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

export const useUpdateUserRole = (userId: () => string | null) => {
    const queryClient = useQueryClient();

    return createMutation(() => ({
        mutationFn: ({ id, role }: { id: string; role: string }) =>
            invokeWrapper<UserRole>("update_user_role", {
                id,
                data: { role },
            }),
        onSuccess: () => {
            queryClient.invalidateQueries({
                queryKey: [...USER_ROLES_QUERY_KEY, userId()],
            });
        },
    }));
};
