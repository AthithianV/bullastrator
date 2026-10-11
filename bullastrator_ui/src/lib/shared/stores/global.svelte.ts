import { handleZoom } from "shared/helpers/zoom";

class GlobalStore {
  shouldShowSidebar: boolean = $state(true);
  shouldShowSearchCommand: boolean = $state(false);

  toggleSearchCommand(newState: boolean) {
    this.shouldShowSearchCommand = newState;
  }

  toggleSidebar(newState: boolean) {
    this.shouldShowSidebar = newState;
  }

  handleKeyDown = (e: KeyboardEvent) => {
    const isMod = e.ctrlKey || e.metaKey;
    if (!isMod) return;

    const key = e.key.toLowerCase();

    // Sidebar Toggle
    if (key === "b") {
      e.preventDefault();
      this.toggleSidebar(!this.shouldShowSidebar);
    }

    // Search Command
    if (e.key === "k" && (e.metaKey || e.ctrlKey)) {
      e.preventDefault();
      this.toggleSearchCommand(!globalStore.shouldShowSearchCommand);
    }

    // Zoom Controls
    if (key === "=" || key === "+") {
      e.preventDefault();
      handleZoom("in");
    } else if (key === "-") {
      e.preventDefault();
      handleZoom("out");
    } else if (key === "0") {
      e.preventDefault();
      handleZoom("reset");
    }
  };
}

export const globalStore = new GlobalStore();
