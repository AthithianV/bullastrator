// Tauri doesn't have a Node.js server to do proper SSR
// so we use adapter-static with a fallback to index.html to put the site in SPA mode
// See: https://svelte.dev/docs/kit/single-page-apps
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";
import path from "path";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  kit: {
    adapter: adapter({
      fallback: "index.html",
    }),
    alias: {
      // Feature Aliases
      connection: path.resolve("src/lib/features/connection"),
      job: path.resolve("src/lib/features/job"),
      metrics: path.resolve("src/lib/features/metrics"),
      queue: path.resolve("src/lib/features/queue"),
      search: path.resolve("src/lib/features/search"),
      sidebar: path.resolve("src/lib/features/sidebar"),
      settings: path.resolve("src/lib/features/settings"),
      tabs: path.resolve("src/lib/features/tabs"),
      worker: path.resolve("src/lib/features/worker"),
      titlebar: path.resolve("src/lib/features/titlebar"),
      workspace: path.resolve("src/lib/features/workspace"),
      folder: path.resolve("src/lib/features/folder"),

      // Shadcn aliases
      components: path.resolve("src/lib/shared/components"),
      shared: path.resolve("src/lib/shared"),
      ui: path.resolve("src/lib/shared/components/ui"),
      utils: path.resolve("src/lib/shared/utils"),
      hooks: path.resolve("src/lib/shared/hooks"),
      helpers: path.resolve("src/lib/shared/helpers"),
    },
  },
};

export default config;
