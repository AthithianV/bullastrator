import { handleZoom } from "shared/helpers/zoom";
import { platform, type Platform } from "@tauri-apps/plugin-os";
import { PUBLIC_IS_WEB } from "$env/static/public";

export type OperatingSystem = Platform | "unknown";

export function getShortcutModifier(os: OperatingSystem): "⌘" | "Ctrl" {
  return os === "macos" || os === "ios" ? "⌘" : "Ctrl";
}

const isWebEnvironment = [true, "true", "1"].includes(PUBLIC_IS_WEB);

function detectOperatingSystem(): OperatingSystem {
  if (!isWebEnvironment) {
    try {
      return platform();
    } catch {
      return "unknown";
    }
  }

  if (typeof navigator === "undefined") return "unknown";

  const userAgent = navigator.userAgent.toLowerCase();
  if (userAgent.includes("mac")) return "macos";
  if (userAgent.includes("win")) return "windows";
  if (userAgent.includes("linux")) return "linux";

  return "unknown";
}

class GlobalStore {
  readonly isWeb = isWebEnvironment;
  readonly os: OperatingSystem = detectOperatingSystem();
  shouldShowSidebar: boolean = $state(true);
  shouldShowSearchCommand: boolean = $state(false);

  shortcut(key: string, shift = false) {
    const displayKey = key === "Enter" ? "↵" : key;
    return `${getShortcutModifier(this.os)}${shift ? " + Shift" : ""} + ${displayKey}`;
  }

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
    if (key === "p") {
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
