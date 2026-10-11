import { invoke } from "@tauri-apps/api/core";

// We track state here because reading it back from the OS
// is an async round-trip we don't need for every keypress.
let currentScale = 1.0;
const STEP = 0.1;
const MAX = 2.0;
const MIN = 0.5;

export const handleZoom = async (direction: "in" | "out" | "reset") => {
  if (direction === "in") {
    currentScale = Math.min(currentScale + STEP, MAX);
  } else if (direction === "out") {
    currentScale = Math.max(currentScale - STEP, MIN);
  } else {
    currentScale = 1.0;
  }

  // Call our Rust command directly
  await invoke("set_app_zoom", { scale: currentScale });
};
