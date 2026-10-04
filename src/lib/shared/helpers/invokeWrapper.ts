import { invoke as tauriInvoke } from "@tauri-apps/api/core";
import { error, info } from "@tauri-apps/plugin-log";
import { toast } from "svelte-sonner";
import { callWebApi, type CommandArgs } from "./apiWrapper";

interface InvokeOptions {
  shouldToast?: boolean;
  successMessage?: string;
  errorMessage?: string;
  shouldLogResult?: boolean;
  loadMessage?: string;
}

const isWeb = [true, "true", "1"].includes(import.meta.env.IS_WEB as any);

export async function invokeWrapper<T>(
  cmd: string,
  args: CommandArgs = {},
  options: InvokeOptions = { shouldToast: false },
): Promise<T> {
  const { shouldToast, successMessage, errorMessage, shouldLogResult } =
    options;

  try {
    const result = isWeb
      ? await callWebApi<T>(cmd, args)
      : await tauriInvoke<T>(cmd, args);

    if (shouldToast)
      toast.success(successMessage || `${cmd} completed successfully`);
    if (shouldLogResult) {
      if (isWeb) console.info(`[API Result] ${cmd}`, result);
      else await info(`[IPC Result]: ${JSON.stringify(result)}`);
    }
    return result;
  } catch (err) {
    const errorString = err instanceof Error ? err.message : String(err);
    if (isWeb) console.error(`[API Error] ${cmd}: ${errorString}`);
    else await error(`[IPC Error] ${cmd}: ${errorString}`);
    if (shouldToast)
      toast.error(errorString || errorMessage || "Something went wrong");
    throw new Error(errorString || errorMessage || "Something went wrong");
  }
}
