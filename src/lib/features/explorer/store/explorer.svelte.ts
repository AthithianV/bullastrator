
class ExplorerStore {
  createFolderForConnectionId = $state<string | null>(null);
  renameFolderId = $state<string | null>(null);
  openDropdownId = $state<string | null>(null);
}

export const explorerStore = new ExplorerStore();
