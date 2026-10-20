<script lang="ts">
    import { untrack } from "svelte";

    import {
        setFolderState,
        useFolderState,
    } from "../store/folder.context.svelte";
    import {
        useGetFolderById,
        useGetQueuesOfFolder,
    } from "../hooks/folder.hooks.svelte";
    import { Button } from "shared/components/ui/button";
    import { Pencil, RefreshCcw, Search } from "@lucide/svelte";
    import type { FolderTabParams } from "../interface/folder.types";
    import { cn } from "shared/utils";
    import TooltipContent from "shared/components/ui/tooltip/tooltip-content.svelte";
    import TooltipTrigger from "shared/components/ui/tooltip/tooltip-trigger.svelte";
    import Tooltip from "shared/components/ui/tooltip/tooltip.svelte";
    import { globalStore } from "shared/stores/global.svelte";

    const {
        tabParams,
        tabId,
        onEdit,
        onSearch,
    }: {
        tabParams: FolderTabParams;
        tabId: string;
        onEdit?: () => void;
        onSearch: () => void;
    } = $props();

    const initialTabId = untrack(() => tabId);
    const initialTabParams = untrack(() => tabParams);
    setFolderState(initialTabId, initialTabParams);

    const folderState = useFolderState();
    const isAllQueuesFolder = $derived(
        folderState.tabId === folderState.connectionId,
    );

    const folderQueueQuery = useGetQueuesOfFolder();
    const folderQuery = useGetFolderById();

    $effect(() => {
        if (folderQuery.data) {
            folderState.folderName = folderQuery.data.title;
        }
    });
</script>

<div
    class={cn(
        "absolute z-20 w-fit max-w-[calc(100%-1rem)] rounded-md border",
        "bg-background/70 backdrop-blur-sm shadow-sm opacity-75",
        "transition-opacity hover:opacity-100",
        "bottom-2 left-1/2 -translate-x-1/2",
    )}
>
    <div class="flex w-fit items-center gap-1 p-1">
        <Tooltip>
            <TooltipTrigger>
                <Button
                    variant="ghost"
                    onclick={onSearch}
                    class="h-8 w-8 p-1 hover:bg-card!"
                    aria-label="Search queues"
                >
                    <Search class="h-4 w-4" />
                </Button>
            </TooltipTrigger>
            <TooltipContent
                class="bg-card border text-white"
                arrowClasses="bg-card rounded-none"
            >
                Search queues
                <kbd class="ml-2 rounded border border-border/50 px-1.5 py-0.5 font-mono text-[10px]">
                    {globalStore.shortcut("F")}
                </kbd>
            </TooltipContent>
        </Tooltip>

        {#if onEdit}
            <Tooltip>
                <TooltipTrigger>
                    <Button
                        variant="ghost"
                        disabled={isAllQueuesFolder}
                        onclick={() => {
                            if (!isAllQueuesFolder) onEdit?.();
                        }}
                        class="h-8 w-8 p-1 hover:bg-card!"
                        aria-label="Edit folder"
                    >
                        <Pencil class="h-4 w-4" />
                    </Button>
                </TooltipTrigger>
                <TooltipContent
                    class="bg-card text-white border"
                    arrowClasses="bg-card rounded-none"
                >
                    {#if isAllQueuesFolder}
                        The All Queues folder cannot be edited
                    {:else}
                        Edit folder
                        <kbd class="ml-2 rounded border border-border/50 px-1.5 py-0.5 font-mono text-[10px]">
                            {globalStore.shortcut("E")}
                        </kbd>
                    {/if}
                </TooltipContent>
            </Tooltip>
        {/if}

        <Tooltip>
            <TooltipTrigger>
                <Button
                    variant="ghost"
                    onclick={() => {
                        folderQueueQuery.refetch();
                    }}
                    class="h-8 w-8 p-1 hover:bg-card!"
                    aria-label="Refresh queues"
                >
                    <RefreshCcw class="h-4 w-4" />
                </Button>
            </TooltipTrigger>
            <TooltipContent
                class="bg-card text-white border"
                arrowClasses="bg-card rounded-none"
            >
                Refresh queues
                <kbd class="ml-2 rounded border border-border/50 px-1.5 py-0.5 font-mono text-[10px]">
                    {globalStore.shortcut("R", true)}
                </kbd>
            </TooltipContent>
        </Tooltip>
    </div>
</div>
