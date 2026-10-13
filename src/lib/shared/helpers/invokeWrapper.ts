import { invoke as tauriInvoke } from "@tauri-apps/api/core";
import { toast } from "svelte-sonner";
import { PUBLIC_IS_WEB } from "$env/static/public";
import { callWebApi, type CommandArgs } from "../apiServices";

interface InvokeOptions {
  shouldToast?: boolean;
  successMessage?: string;
  errorMessage?: string;
  shouldLogResult?: boolean;
  loadMessage?: string;
}

const isWeb = [true, "true", "1"].includes(PUBLIC_IS_WEB);

export async function invokeWrapper<T>(
  cmd: string,
  args: CommandArgs = {},
  options: InvokeOptions = { shouldToast: false },
): Promise<T> {
  const { shouldToast, successMessage, errorMessage, shouldLogResult } = options;

  try {
    const result = isWeb ? await callWebApi<T>(cmd, args) : await tauriInvoke<T>(cmd, args);

    if (shouldToast) toast.success(successMessage || `${cmd} completed successfully`);
    if (shouldLogResult) {
      console.info(`[API Result] ${cmd}`, result);
    }
    return result;
  } catch (err) {
    const errorString = err instanceof Error ? err.message : String(err);
    console.error(`[API Error] ${cmd}: ${errorString}`);
    if (shouldToast) toast.error(errorString || errorMessage || "Something went wrong");
    throw new Error(errorString || errorMessage || "Something went wrong");
  }
}
