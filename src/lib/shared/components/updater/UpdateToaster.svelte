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
    import * as Popover from "components/ui/popover";
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
    <Popover.Root>
        <Popover.Trigger>
            <Button
                variant="ghost"
                size="icon"
                class="relative size-8 text-muted-foreground hover:text-foreground"
                aria-label="Application update"
                title="Application update"
            >
                <Download class="size-4" />
                {#if updateManager.status !== "ready"}
                    <span
                        class="absolute right-1 top-1 size-1.5 rounded-full bg-primary"
                    ></span>
                {/if}
            </Button>
        </Popover.Trigger>
        <Popover.Content align="end" sideOffset={8} class="w-80 p-4">
            <div class="space-y-4">
                <div class="space-y-1">
                    <div class="flex items-center gap-2 text-sm font-semibold">
                        <Download class="size-4" />
                        Update available
                    </div>
                    <p class="text-xs text-muted-foreground">
                        Version {updateManager.updater?.version} is ready to install.
                    </p>
                    {#if updateManager.updater?.version === "0.2.0"}
                        <div
                            class="flex items-center gap-1.5 pt-1 text-xs text-yellow-600"
                        >
                            <TriangleAlert class="size-4 shrink-0" />
                            This update will remove existing data.
                        </div>
                    {/if}
                </div>

                {#if updateManager.status === "downloading"}
                    <div class="space-y-2">
                        <Progress value={progressPercent} />
                        <p class="text-right text-xs text-muted-foreground">
                            {Math.round(progressPercent)}%
                        </p>
                    </div>
                {:else}
                    <div class="flex justify-end gap-2">
                        {#if isLinux}
                            <a
                                href="https://releases.bullastrator.dev/"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="flex items-center gap-1 rounded bg-primary px-3 py-1 text-sm font-semibold text-black"
                            >
                                <DownloadIcon class="size-4" />
                                Download
                            </a>
                        {:else}
                            <Button
                                size="sm"
                                class="flex items-center gap-1 bg-primary px-3 py-1 font-semibold text-black"
                                onclick={async () => {
                                    if (updateManager.status === "downloaded") {
                                        await updateManager.restart();
                                    } else {
                                        await updateManager.install();
                                    }
                                }}
                            >
                                {#if updateManager.status === "downloaded"}
                                    <RefreshCcw class="size-4" />
                                    Restart
                                {:else}
                                    <ArrowBigUpDash class="size-4" />
                                    Update now
                                {/if}
                            </Button>
                        {/if}
                    </div>
                {/if}
            </div>
        </Popover.Content>
    </Popover.Root>
{/if}
