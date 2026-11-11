// src/lib/shared/runtime.ts
import { isTauri } from "@tauri-apps/api/core";

export const runtime = {
    get isDesktop() {
        return isTauri();
    },
    get isWeb() {
        return !isTauri();
    },
};
