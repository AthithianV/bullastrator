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
    import { Pencil, RefreshCcw } from "@lucide/svelte";
    import type { FolderTabParams } from "../interface/folder.types";
    import { tabStore } from "$lib/features/tabs/store/tabStore.svelte";
    import { cn } from "shared/utils";

    export type FolderToolbarPosition =
        | "top-left"
        | "top-center"
        | "top-right"
        | "bottom-right"
        | "left"
        | "center";

    const {
        tabParams,
        tabId,
        position = "center",
        onEdit,
    }: {
        tabParams: FolderTabParams;
        tabId: string;
        position?: FolderToolbarPosition;
        onEdit?: () => void;
    } = $props();

    const positionClasses: Record<FolderToolbarPosition, string> = {
        "top-left": "top-2 left-2",
        "top-center": "top-2 left-1/2 -translate-x-1/2",
        "top-right": "top-2 right-2",
        "bottom-right": "bottom-2 right-2",
        left: "left-2 top-1/2 -translate-y-1/2",
        center: "top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2",
    };

    const toolbarPosition = position ?? "top-right";

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

    let refreshCountdown = $state(5);

    $effect(() => {
        if (!folderState.tabId) return;

        refreshCountdown = 5;

        const timer = setInterval(async () => {
            refreshCountdown -= 1;
            if (
                refreshCountdown === 0 &&
                folderState.tabId === tabStore.activeTab &&
                folderState.shouldRefresh
            ) {
                await folderQueueQuery.refetch();
                refreshCountdown = 5;
            }
        }, 1000);

        return () => clearInterval(timer);
    });
</script>

<div
    class={cn(
        "absolute z-20 w-fit max-w-[calc(100%-1rem)] rounded-md border",
        "bg-background/70 backdrop-blur-sm shadow-sm opacity-75",
        "transition-opacity hover:opacity-100",
        positionClasses[toolbarPosition],
    )}
>
    <div class="flex w-fit items-center gap-1 p-1">
        {#if folderState.shouldRefresh}
            <Button
                class="text-sm pb-2 text-sky-400"
                onclick={() => {
                    folderState.shouldRefresh = false;
                    refreshCountdown = 5;
                }}
                variant="link"
            >
                {#if !folderQueueQuery.isFetching}
                    Refreshs in {refreshCountdown}s
                {:else}
                    Refreshs in 0s
                {/if}
            </Button>
        {:else}
            <Button
                class="text-sm text-muted-foreground pb-2 hover:text-sky-400"
                onclick={() => {
                    folderState.shouldRefresh = true;
                    refreshCountdown = 5;
                }}
                variant="link"
            >
                Turn on auto-refresh
            </Button>
        {/if}

        {#if !isAllQueuesFolder && onEdit}
            <Button
                variant="ghost"
                onclick={() => {
                    onEdit?.();
                }}
                class="h-8 w-8 p-1 hover:bg-card!"
            >
                <Pencil class="h-4 w-4" />
            </Button>
        {/if}

        <Button
            variant="ghost"
            onclick={() => {
                folderQueueQuery.refetch();
            }}
            class="h-8 w-8 p-1 hover:bg-card!"
        >
            <RefreshCcw class="h-4 w-4" />
        </Button>
    </div>
</div>
