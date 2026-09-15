<script lang="ts">
    import { getCurrentWindow } from "@tauri-apps/api/window";
    import { platform } from "@tauri-apps/plugin-os";
    import QueueSearch from "queue/components/QueueSearch.svelte";
    import Close from "shared/components/icons/Close.svelte";
    import Minimize from "shared/components/icons/Minimize.svelte";
    import RestoreDown from "shared/components/icons/RestoreDown.svelte";
    import RestoreUp from "shared/components/icons/RestoreUp.svelte";
    import { Button } from "shared/components/ui/button";
    import WorkspaceSelector from "workspace/components/WorkspaceSelector.svelte";
    import Brand from "./Brand.svelte";
    import ThemeToggler from "$lib/shared/components/actions/ThemeToggler.svelte";

    const currentPlatform = platform();
    const isMacos = currentPlatform === "macos";

    let isMaximized = $state(false);
    $effect(() => {
        const appWindow = getCurrentWindow();

        let cleanup: () => void;
        (async () => {
            isMaximized = await appWindow.isMaximized();

            const unlisten = await appWindow.onResized(async () => {
                isMaximized = await appWindow.isMaximized();
            });

            cleanup = unlisten;
        })();

        return () => {
            if (cleanup) cleanup();
        };
    });

    const handleMinimize = async () => {
        const appWindow = getCurrentWindow();
        await appWindow.minimize();
    };

    const handleMaximize = async () => {
        const appWindow = getCurrentWindow();
        await appWindow.toggleMaximize();
    };

    const handleClose = async () => {
        const appWindow = getCurrentWindow();
        await appWindow.close();
    };
</script>

<div
    class="w-full h-(--titlebar-height) flex justify-between items-center"
    data-tauri-drag-region
>
    <div class="flex items-center">
        {#if !isMacos || (isMacos && isMaximized)}
            <!-- <div class={cn("brand-container")}> -->
            <Brand />
            <!-- </div> -->
        {/if}
        <WorkspaceSelector />
    </div>
    <QueueSearch />
    <div
        role="button"
        tabindex="0"
        class="flex-1 px-3 flex gap-2 text-sm items-center justify-center font-semibold"
    ></div>
    <div class="flex items-center h-full gap-1 px-2">
        <ThemeToggler />
        {#if !isMacos}
            <div class="h-full flex items-center">
                <Button
                    variant="ghost"
                    size="icon"
                    class="px-6 h-full rounded-none text-foreground hover:bg-slate-200 dark:hover:bg-slate-800"
                    onclick={handleMinimize}
                >
                    <Minimize />
                </Button>
                <Button
                    variant="ghost"
                    size="icon"
                    class="px-6 text-foreground h-full rounded-none hover:bg-slate-200 dark:hover:bg-slate-800"
                    onclick={handleMaximize}
                >
                    {#if isMaximized}
                        <RestoreDown />
                    {:else}
                        <RestoreUp />
                    {/if}
                </Button>
                <Button
                    variant="ghost"
                    size="icon"
                    class="px-6 text-foreground h-full rounded-none hover:bg-red-500 dark:hover:bg-red-800 hover:text-white"
                    onclick={handleClose}
                >
                    <Close />
                </Button>
            </div>
        {/if}
    </div>
</div>
