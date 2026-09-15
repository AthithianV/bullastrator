import type { ReadQueueModel } from "queue/interface/queue.types";
import { getContext, setContext } from "svelte";

const FOLDER_KEY = Symbol("FOLDER_STATE");

export class FolderState {
  // 1. Context (The "Where")
  connectionId = $state<string>("");
  folderName = $state<string>("");
  tabId = $state<string>("");
  queues = $state<string[]>([]);
  shouldRefresh = $state<boolean>(false);

  constructor(tabId: string, initialParams: any) {
    this.tabId = tabId;
    this.hydrate(initialParams);
  }

  hydrate(params: any) {
    if (!params) return;
    this.connectionId = params.connectionId;
    // this.folderName = params.folderName;
  }
}

export function setFolderState(tabId: string, params: any) {
  const state = new FolderState(tabId, params);
  setContext(FOLDER_KEY, state);
  return state;
}

export function useFolderState() {
  return getContext<FolderState>(FOLDER_KEY);
}
