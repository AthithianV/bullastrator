<script lang="ts">
    import * as Resizable from "ui/resizable";
    import * as Scroll from "ui/scroll-area/index";
    import Separator from "ui/separator/separator.svelte";

    import { dndzone } from "svelte-dnd-action";
    import { flip } from "svelte/animate";

    import {
        setFolderState,
        useFolderState,
    } from "../store/folder.context.svelte";
    import { Input } from "shared/components/ui/input";
    import { useGetAllQueuesByConnection } from "queue/hooks/queue.hooks";
    import type { ReadConnection } from "connection/interface/connection.types";
    import Checkbox from "shared/components/ui/checkbox/checkbox.svelte";
    import { cn } from "shared/utils";
    import {
        useGetFolderById,
        useGetQueuesOfFolder,
        useReorderFolderQueues,
        useToggleQueueInFolder,
        useUpdateFolder,
    } from "../hooks/folder.hooks.svelte";
    import QueueJobCount from "./QueueJobCount.svelte";
    import { Button } from "shared/components/ui/button";
    import { Pencil, RefreshCcw, Search, X } from "@lucide/svelte";
    import { useUpdateTab } from "tabs/hooks/tab.hooks";
    import type { FolderTabParams } from "../interface/folder.types";
    import InfiniteScanLoader from "shared/components/loaders/InfiniteScanLoader.svelte";
    import { tabStore } from "$lib/features/tabs/store/tabStore.svelte";

    const {
        tabParams,
        tabId,
    }: {
        tabParams: FolderTabParams;
        tabId: string;
        connection?: ReadConnection;
    } = $props<{
        tabParams: FolderTabParams;
        tabId: string;
        connection?: ReadConnection;
    }>();

    setFolderState(tabId, tabParams);

    const folderState = useFolderState();
    let searchKeyword = $state("");

    const queuesQuery = useGetAllQueuesByConnection(
        folderState.connectionId,
        false,
    );
    const folderQueueQuery = useGetQueuesOfFolder();
    const folderQuery = useGetFolderById();

    const { mutateAsync: updateFolder } = useUpdateFolder();
    const { mutateAsync: updateTab } = useUpdateTab();
    const { mutateAsync: reorderFolderQueues } = useReorderFolderQueues();

    let folderNameInput = $state<HTMLInputElement | null>(null);

    let queueIdSet = $derived(
        new Set((folderQueueQuery.data ?? []).map((q) => q.id)),
    );

    let queueEditPanel = $state(0);

    const { mutateAsync: toggleQueue } = useToggleQueueInFolder();

    const handleTitleChange = async (newTitle: string) => {
        if (folderState.tabId) {
            await updateFolder({ id: folderState.tabId, title: newTitle });
            await updateTab({
                id: folderState.tabId,
                data: { title: newTitle },
            });
        }
    };

    $effect(() => {
        if (folderQuery.data) {
            folderState.folderName = folderQuery.data.title;
        }
    });

    const filteredQueues = $derived(
        (queuesQuery.data ?? []).filter((q) =>
            q.queueName.toLowerCase().includes(searchKeyword.toLowerCase()),
        ),
    );

    const flipDurationMs = 300;

    function handleDndConsider(e: CustomEvent) {
        folderQueueQuery.data = e.detail.items;
    }

    function handleDndFinalize(e: CustomEvent) {
        folderQueueQuery.data = e.detail.items;

        reorderFolderQueues({
            folderId: folderState.tabId,
            orderedQueueIds: e.detail.items.map((i: any) => i.id),
        });
    }

    $effect(() => {
        if (queueEditPanel < 5 && folderQueueQuery.data?.length === 0) {
            queueEditPanel = 30;
        }

        if (queueEditPanel > 0 && folderNameInput) {
            const timer = setTimeout(() => {
                folderNameInput?.focus();
                folderNameInput?.select();
            }, 50);

            return () => clearTimeout(timer);
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
    class="h-(--tab-content-height) flex flex-col overflow-x-auto no-scrollbar min-w-0"
>
    <Separator />

    <Resizable.PaneGroup direction="horizontal" class="min-w-0">
        <Resizable.Pane class="relative min-w-0 p-5" defaultSize={70}>
            <InfiniteScanLoader loading={folderQueueQuery.isFetching} />
            <Scroll.ScrollArea class="h-full @container">
                <div class="flex flex-col gap-4">
                    <div
                        class="flex items-center justify-between bg-background sticky top-0 py-2 border-b"
                    >
                        <div class="flex gap-2 justify-end items-end w-full">
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

                            {#if queueEditPanel < 5}
                                <Button
                                    variant="ghost"
                                    onclick={() => {
                                        if (queueEditPanel === 0)
                                            queueEditPanel = 30;
                                    }}
                                    class="border shadow"
                                >
                                    <Pencil />
                                </Button>
                            {/if}

                            <Button
                                variant="ghost"
                                onclick={() => {
                                    folderQueueQuery.refetch();
                                }}
                                class="border shadow"
                            >
                                <RefreshCcw />
                            </Button>
                        </div>
                    </div>

                    <div class="flex flex-col gap-2">
                        {#if !folderQueueQuery.data || folderQueueQuery.data?.length === 0}
                            <p
                                class="text-sm text-muted-foreground/50 italic mx-auto"
                            >
                                No queues added yet.
                            </p>
                        {/if}
                        <ul
                            class="grid @[1600px]:grid-cols-5 @[1200px]:grid-cols-4 @[900px]:grid-cols-3 @[600px]:grid-cols-2 grid-cols-1 gap-2"
                            use:dndzone={{
                                items: folderQueueQuery.data ?? [],
                                flipDurationMs,
                                dropTargetStyle: { outline: "none" },
                            }}
                            onconsider={handleDndConsider}
                            onfinalize={handleDndFinalize}
                        >
                            {#each folderQueueQuery.data ?? [] as queue (queue.id)}
                                <li
                                    class="text-sm rounded-md h-full"
                                    animate:flip={{ duration: flipDurationMs }}
                                >
                                    <QueueJobCount {queue} />
                                </li>
                            {/each}
                        </ul>
                    </div>
                </div>
                <Scroll.Scrollbar orientation="vertical" />
            </Scroll.ScrollArea>
        </Resizable.Pane>

        <Resizable.Handle />

        <Resizable.Pane class="min-w-0" defaultSize={queueEditPanel}>
            <Scroll.ScrollArea class="h-full">
                <div class="px-4 flex flex-col gap-2">
                    <div class="sticky top-0 z-100 bg-background">
                        <div class="py-2 flex items-center justify-between">
                            <Input
                                bind:ref={folderNameInput}
                                bind:value={folderState.folderName}
                                class={cn(
                                    "bg-transparent dark:bg-transparent",
                                    "border-none shadow-none px-0 text-2xl! h-10!",
                                    "font-semibold focus-visible:ring-0 rounded-none text-wrap",
                                )}
                                placeholder="Folder Name"
                                oninput={(e) =>
                                    handleTitleChange(e.currentTarget.value)}
                            />
                            <Button
                                variant="ghost"
                                onclick={() => {
                                    queueEditPanel = 0;
                                }}
                                class="hover:text-red-500"
                            >
                                <X class="w-5 h-5" />
                            </Button>
                        </div>
                        <div
                            class="flex items-center bg-background dark:bg-input/30 border px-2 rounded-[5px] max-lg:hidden"
                        >
                            <Search class="h-4 w-4" />
                            <Input
                                bind:value={searchKeyword}
                                class="dark:bg-transparent border-none shadow-none w-80 py-0"
                                placeholder="Search..."
                                type="text"
                            />
                        </div>
                    </div>

                    <div class="flex flex-col gap-2">
                        {#each filteredQueues as queue}
                            {@const isChecked = queueIdSet.has(queue.id)}
                            <label
                                class="flex items-center gap-2 p-2 rounded-md hover:bg-muted/50 cursor-pointer transition-colors"
                            >
                                <Checkbox
                                    checked={isChecked}
                                    onCheckedChange={() =>
                                        toggleQueue({
                                            queueId: queue.id,
                                            folderId: folderState.tabId,
                                        })}
                                    class="w-4 h-4 rounded border-gray-300 text-primary focus:ring-primary"
                                />
                                <span class="text-sm font-medium"
                                    >{queue.queueName}</span
                                >
                            </label>
                        {/each}
                    </div>
                </div>
                <Scroll.Scrollbar orientation="vertical" />
            </Scroll.ScrollArea>
        </Resizable.Pane>
    </Resizable.PaneGroup>
</div>
