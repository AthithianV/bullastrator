<script lang="ts">
    import { updateManager } from "$lib/shared/helpers/autoUpdater.svelte";
    import { Download } from "@lucide/svelte";
    import { platform } from "@tauri-apps/plugin-os";
    import { Button } from "components/ui/button";
    import * as Card from "components/ui/card";
    import { Progress } from "components/ui/progress";

    const currentPlatform = platform();
    const isLinux = currentPlatform === "linux";

    updateManager.checkForUpdates();

    const progressPercent = $derived(
        updateManager.contentLength > 0
            ? (updateManager.downloaded / updateManager.contentLength) * 100
            : 0,
    );
</script>

{#if updateManager.status === "ready" || updateManager.status === "downloading" || updateManager.status === "downloaded"}
    <div
        class="fixed bottom-6 right-6 z-50 w-100 animate-in fade-in slide-in-from-bottom-4"
    >
        <Card.Root class="shadow-lg">
            <Card.Header class="pb-1">
                <Card.Title class="text-sm font-bold flex items-center gap-2">
                    <Download class="w-4 h-4" />
                    Update Available
                </Card.Title>
                <Card.Description>
                    Version {updateManager.updater?.version} is ready to install.
                </Card.Description>
            </Card.Header>

            <Card.Content>
                {#if updateManager.status === "downloading"}
                    <div class="space-y-2">
                        <Progress value={progressPercent} />
                        <p class="text-xs text-muted-foreground text-right">
                            {Math.round(progressPercent)}%
                        </p>
                    </div>
                {:else}
                    <div class="flex justify-end gap-2">
                        <Button
                            variant="ghost"
                            size="sm"
                            onclick={() => updateManager.cancel()}
                        >
                            Dismiss
                        </Button>
                        {#if isLinux}
                            <a
                                href={"https://releases.asyncian.dev/"}
                                target="_blank"
                                rel="noopener noreferrer"
                                class="px-3 py-1 bg-primary font-semibold text-black text-sm rounded flex justify-center items-center"
                            >
                                Download
                            </a>
                        {:else}
                            <Button
                                size="sm"
                                onclick={async () => {
                                    if (updateManager.status === "downloaded") {
                                        await updateManager.restart();
                                    } else {
                                        await updateManager.install();
                                    }
                                }}
                            >
                                {updateManager.status === "downloaded"
                                    ? "Restart"
                                    : "Update Now"}
                            </Button>
                        {/if}
                    </div>
                {/if}
            </Card.Content>
        </Card.Root>
    </div>
{/if}
