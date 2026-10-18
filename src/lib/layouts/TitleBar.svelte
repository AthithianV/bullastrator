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
    import { globalStore } from "shared/stores/global.svelte";
    import { Search } from "@lucide/svelte";

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
            <Brand />
        {/if}
        <WorkspaceSelector />
    </div>
    <QueueSearch />
    <div
        class="flex-1 px-3 flex gap-2 text-sm items-center justify-center font-semibold"
    >
        <Button
            variant="ghost"
            class="bg-background/40 hover:bg-background! h-8 gap-2 px-3 w-[15%] flex justify-between"
            onclick={() => globalStore.toggleSearchCommand(true)}
        >
            <div
                class="flex justify-start items-center gap-2 text-muted-foreground"
            >
                <Search class="h-4 w-4" />
                <span>Search</span>
            </div>

            <kbd class="text-xs opacity-60">{globalStore.shortcut("P")}</kbd>
        </Button>
    </div>
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
