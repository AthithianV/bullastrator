import type { ReadTabModel } from "../interface/tab.types";

class TabStore {
  activeTab = $state<string | null>(null);

  setActiveTab(tab: string | null) {
    this.activeTab = tab;
  }

  moveToNextTabOnClose(closedTabId: string, allTabs: ReadTabModel[]) {
    if (allTabs.length === 0) return null;

    // 1. If we aren't closing the active tab, we don't need to move the focus.
    if (this.activeTab !== closedTabId) return this.activeTab;

    // 2. If it's the last tab in the app, focus nothing.
    if (allTabs.length === 1) {
      this.activeTab = null;
      return null;
    }

    const index = allTabs.findIndex((tab) => tab.id === closedTabId);
    if (index === -1) return null;

    let newIndex: number;

    if (index === allTabs.length - 1) {
      // We are closing the LAST tab. Move to the new last tab (index - 1).
      newIndex = index - 1;
    } else {
      // We are closing any other tab.
      // The "next" tab will slide into the CURRENT index.
      newIndex = index + 1;
    }

    const newId = allTabs[newIndex].id;
    this.activeTab = newId;
    return newId;
  }
}

export const tabStore = new TabStore();
