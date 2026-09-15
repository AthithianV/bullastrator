import { relaunch } from "@tauri-apps/plugin-process";
import { check, type Update } from "@tauri-apps/plugin-updater";

export const updateManager = $state({
  status: "idle" as
    | "idle"
    | "checking"
    | "ready"
    | "downloading"
    | "error"
    | "downloaded",
  updater: null as Update | null,
  contentLength: 0,
  downloaded: 0,

  async checkForUpdates() {
    try {
      this.status = "checking";
      const update = await check();
      if (update) {
        this.updater = update;
        this.status = "ready";
      } else {
        this.status = "idle";
      }
    } catch (e) {
      console.error(e);
      this.status = "error";
    }
  },

  async install() {
    if (this.status === "ready" && this.updater) {
      this.status = "downloading";
      this.downloaded = 0;

      await this.updater.downloadAndInstall((event) => {
        switch (event.event) {
          case "Started":
            this.contentLength = event.data.contentLength || 0;
            break;
          case "Progress":
            this.downloaded += event.data.chunkLength;
            break;
          case "Finished":
            this.status = "downloaded";
        }
      });
    }
  },

  async restart() {
    if (this.status === "downloaded") {
      await relaunch();
    }
  },

  cancel() {
    this.status = "idle";
    this.updater = null;
  },
});
