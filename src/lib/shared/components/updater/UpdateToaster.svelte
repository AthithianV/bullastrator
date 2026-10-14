<script lang="ts">
    import { updateManager } from "$lib/shared/helpers/autoUpdater.svelte";
    import {
        ArrowBigUpDash,
        Download,
        DownloadIcon,
        RefreshCcw,
        TriangleAlert,
    } from "@lucide/svelte";
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
        <Card.Root class="shadow-lg bg-background border-card">
            <Card.Header class="pb-1">
                <Card.Title class="text-sm font-bold flex items-center gap-2">
                    <Download class="w-4 h-4" />
                    Update Available
                </Card.Title>
                <Card.Description class="space-y-2">
                    Version {updateManager.updater?.version} is ready to install.
                    {#if updateManager.updater?.version === "0.2.0"}
                        <div
                            class="flex text-xs justify-start items-center text-yellow-600 mt-2 gap-1"
                        >
                            <TriangleAlert class="h-4 w-4" />
                            Caution: You will lose all data in this update
                        </div>
                    {/if}
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
                                href={"https://releases.bullastrator.dev/"}
                                target="_blank"
                                rel="noopener noreferrer"
                                class="px-3 py-1 bg-primary font-semibold text-black text-sm rounded flex justify-center items-center gap-1"
                            >
                                <DownloadIcon class="h-4 w-4" />
                                Download
                            </a>
                        {:else}
                            <Button
                                size="sm"
                                class="px-3 py-1 bg-primary font-semibold text-black text-sm rounded flex justify-center items-center gap-1"
                                onclick={async () => {
                                    if (updateManager.status === "downloaded") {
                                        await updateManager.restart();
                                    } else {
                                        await updateManager.install();
                                    }
                                }}
                            >
                                {#if updateManager.status === "downloaded"}
                                    <RefreshCcw />
                                    Restart
                                {:else}
                                    <ArrowBigUpDash />
                                    Update Now
                                {/if}
                            </Button>
                        {/if}
                    </div>
                {/if}
            </Card.Content>
        </Card.Root>
    </div>
{/if}
