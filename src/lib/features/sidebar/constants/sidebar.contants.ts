import {
  Bell,
  Box,
  ChartArea,
  FolderClosed,
  HardHat,
  History,
  Power,
  Search,
  type IconProps,
} from "@lucide/svelte";
import { globalStore } from "shared/stores/global.svelte";
import { sidebarStore } from "sidebar/store/sidebarStore.svelte";
import type { Component } from "svelte";
import type { SidebarMode } from "../interfaces/sidebar.types";

export const SIDEBAR_MODES: {
  id: SidebarMode;
  icon: Component<IconProps, {}, "">;
  label: string;
  handleClick: () => void;
  shouldShow: boolean;
}[] = [
  {
    id: "QUEUE",
    icon: Box,
    label: "Queues View",
    handleClick: () => {
      globalStore.toggleSidebar(true);
      sidebarStore.setMode("QUEUE");
    },
    shouldShow: true,
  },
  {
    id: "SEARCH",
    icon: Search,
    label: "Search",
    handleClick: () => {
      globalStore.toggleSidebar(true);
      sidebarStore.setMode("SEARCH");
    },
    shouldShow: true,
  },
  {
    id: "CONNECTION",
    icon: Power,
    label: "Connections",
    handleClick: () => {
      globalStore.toggleSidebar(true);
      sidebarStore.setMode("CONNECTION");
    },
    shouldShow: true,
  },
  {
    id: "FOLDERS",
    icon: FolderClosed,
    label: "Folders",
    handleClick: () => {
      globalStore.toggleSidebar(true);
      sidebarStore.setMode("FOLDERS");
    },
    shouldShow: false,
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
    id: "WORKER",
    icon: HardHat,
    label: "Worker View",
    handleClick: () => {
      globalStore.toggleSidebar(true);
      sidebarStore.setMode("WORKER");
    },
    shouldShow: false,
  },
  {
    id: "METRICS",
    icon: ChartArea,
    label: "Metrics",
    handleClick: () => {
      globalStore.toggleSidebar(true);
      sidebarStore.setMode("METRICS");
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
