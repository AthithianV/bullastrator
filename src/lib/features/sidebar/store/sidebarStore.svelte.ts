import type { SidebarMode } from "../interfaces/sidebar.types";

class SidebarStore {
    mode: SidebarMode = $state("QUEUE");

    setMode(mode: SidebarMode) {
        this.mode = mode;
    }
}

export const sidebarStore = new SidebarStore();
