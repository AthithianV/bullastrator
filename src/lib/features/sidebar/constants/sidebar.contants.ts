import {
  Bell,
  Box,
  ChartArea,
  FolderClosed,
  HardHat,
  History,
  Power,
  Search,
} from "@lucide/svelte";
import { globalStore } from "shared/stores/global.svelte";
import { sidebarStore } from "sidebar/store/sidebarStore.svelte";
import type { Component } from "svelte";
import type { SidebarMode } from "../interfaces/sidebar.types";
import Explorer from "../../explorer/components/Explorer.svelte";

export const SIDEBAR_MODES: {
  id: SidebarMode;
  icon: Component;
  label: string;
  component?: Component;
  handleClick: () => void;
  shouldShow: boolean;
}[] = [
  {
    id: "EXPLORER",
    icon: Box,
    label: "Explorer",
    component: Explorer,
    handleClick: () => {
      globalStore.toggleSidebar(true);
      sidebarStore.setMode("EXPLORER");
    },
    shouldShow: true,
  },
  {
    id: "HISTORY",
    icon: History,
    label: "HISTORY",
    handleClick: () => {
      globalStore.toggleSidebar(true);
      sidebarStore.setMode("HISTORY");
    },
    shouldShow: false,
  },
  {
    id: "NOTIFICATION",
    icon: Bell,
    label: "Notifications",
    handleClick: () => {
      globalStore.toggleSidebar(true);
      sidebarStore.setMode("NOTIFICATION");
    },
    shouldShow: false,
  },
];
