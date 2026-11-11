import { confirm as tauriConfirm } from "@tauri-apps/plugin-dialog";
import { runtime } from "shared/runtime";
import { get, writable } from "svelte/store";

export type ConfirmationRequest = {
  message: string;
  title: string;
  resolve: (confirmed: boolean) => void;
};

export const confirmationRequest = writable<ConfirmationRequest | null>(null);

function requestWebConfirmation(message: string, title: string) {
  return new Promise<boolean>((resolve) => {
    confirmationRequest.set({ message, title, resolve });
  });
}

export function resolveConfirmation(confirmed: boolean) {
  const request = get(confirmationRequest);
  confirmationRequest.set(null);
  request?.resolve(confirmed);
}

export function confirmAction(
  message: string,
  options: { title: string; kind?: "warning" | "info" } = {
    title: "Confirm action",
  },
) {
  if (runtime.isWeb) {
    return requestWebConfirmation(message, options.title);
  }

  return tauriConfirm(message, options);
}
