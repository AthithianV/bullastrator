import { createMutation, createQuery, useQueryClient } from "@tanstack/svelte-query";
import { invokeWrapper } from "shared/helpers/invokeWrapper";
import { toast } from "svelte-sonner";
import type {
  AuthResponse,
  AuthenticatedSession,
  LoginRequest,
  LogoutResponse,
  RegisterRequest,
} from "../interface";

export const USER_QUERY_KEY = ["user"];
export const USERS_QUERY_KEY = ["users"];

export const useGetAllUsers = (enabled: () => boolean = () => true) => {
  return createQuery(() => ({
    queryKey: USERS_QUERY_KEY,
    queryFn: () => invokeWrapper<User[]>("get_all_users"),
    enabled: enabled(),
  }));
};

/** Check whether the current HTTP-only session cookie is valid. */
export const useCheckSession = () => {
  return async (): Promise<boolean> => {
    try {
      await invokeWrapper<AuthenticatedSession>(
        "check_session",
        {},
        {
          shouldToast: false,
          shouldLogResult: false,
        },
      );
      return true;
    } catch {
      return false;
    }
  };
};

/** Register a new user. */
export const useRegister = () => {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: (data: RegisterRequest) => invokeWrapper<AuthResponse>("register", { data }),
    onSuccess: (response) => {
      queryClient.setQueryData(USER_QUERY_KEY, response.user);
      toast.success("Account created successfully");
    },
    onError: (error: Error) => toast.error(error.message),
  }));
};

/** Log in an existing user. */
export const useLogin = () => {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: (data: LoginRequest) => invokeWrapper<AuthResponse>("login", { data }),
    onSuccess: (response) => {
      queryClient.setQueryData(USER_QUERY_KEY, response.user);
      toast.success("Logged in successfully");
    },
    onError: (error: Error) => toast.error(error.message),
  }));
};

/** Log out the current user and clear cached user data. */
export const useLogout = () => {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: () => invokeWrapper<LogoutResponse>("logout"),
    onSuccess: () => {
      queryClient.removeQueries({ queryKey: USER_QUERY_KEY });
      queryClient.clear();
      toast.success("Logged out successfully");
    },
    onError: (error: Error) => toast.error(error.message),
  }));
};
