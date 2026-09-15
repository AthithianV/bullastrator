import { invoke as tauriInvoke } from "@tauri-apps/api/core";
import { error, info } from "@tauri-apps/plugin-log";
import { toast } from "svelte-sonner";

interface InvokeOptions {
  shouldToast?: boolean;
  successMessage?: string;
  errorMessage?: string;
  shouldLogResult?: boolean;
  loadMessage?: string;
}

export async function invokeWrapper<T>(
  cmd: string,
  args?: Record<string, any>,
  options: InvokeOptions = { shouldToast: false },
): Promise<T> {
  const { shouldToast, successMessage, errorMessage, shouldLogResult } =
    options;

  try {
    // await info(`\n\n[IPC Start] ${cmd}`);
    const result = await tauriInvoke<T>(cmd, args);

    if (shouldToast) {
      toast.success(successMessage || `${cmd} completed successfully`);
    }

    if (shouldLogResult) {
      await info(`[IPC Result]: ${JSON.stringify(result)}`);
    }

    return result;
  } catch (err) {
    const errorString = typeof err === "string" ? err : JSON.stringify(err);

    // Log to Rust backend
    await error(`[IPC Error] ${cmd}: ${errorString}\n\n`);

    // Failure handling
    if (shouldToast) {
      toast.error(errorString || errorMessage || `Some thing Went Wrong`);
    }

    throw new Error(errorString);
  }
}
