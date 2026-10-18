<script lang="ts">
    import * as Resizable from "ui/resizable";
    import * as Scroll from "ui/scroll-area/index";
    import Separator from "ui/separator/separator.svelte";

    import { dndzone } from "svelte-dnd-action";
    import { flip } from "svelte/animate";
    import { untrack } from "svelte";

    import {
        setFolderState,
        useFolderState,
    } from "../store/folder.context.svelte";
    import { Input } from "shared/components/ui/input";
    import { useGetAllQueuesByConnection } from "queue/hooks/queue.hooks";
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
    import FolderToolbar from "./FolderToolbar.svelte";
    import { useFolderShortcuts } from "../hooks/folder.shortcuts.svelte";

    const {
        tabParams,
        tabId,
    }: {
        tabParams: FolderTabParams;
        tabId: string;
    } = $props();

    const initialTabId = untrack(() => tabId);
    const initialTabParams = untrack(() => tabParams);
    setFolderState(initialTabId, initialTabParams);

    const folderState = useFolderState();
    const isAllQueuesFolder = $derived(
        folderState.tabId === folderState.connectionId,
    );
    let searchKeyword = $state("");
    let showSearch = $state(false);
    let searchInput = $state<HTMLInputElement | null>(null);
    let folderContainer = $state<HTMLDivElement | null>(null);

    const queuesQuery = useGetAllQueuesByConnection(folderState.connectionId);
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
        if (folderState.tabId && !isAllQueuesFolder) {
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

    const { handleQueueCardKeydown } = useFolderShortcuts({
        tabId: () => tabId,
        isAllQueuesFolder: () => isAllQueuesFolder,
        folderContainer: () => folderContainer,
        searchInput: () => searchInput,
        onSearch: () => {
            showSearch = !showSearch;
        },
        onEdit: () => {
            queueEditPanel = queueEditPanel === 0 ? 30 : 0;
        },
        onRefresh: () => void folderQueueQuery.refetch(),
    });

    const filteredQueues = $derived(
        (queuesQuery.data ?? []).filter((q) =>
            q.queueName.toLowerCase().includes(searchKeyword.toLowerCase()),
        ),
    );
    const visibleFolderQueues = $derived(
        (folderQueueQuery.data ?? []).filter((q) =>
            q.queueName.toLowerCase().includes(searchKeyword.toLowerCase()),
        ),
    );
    const flipDurationMs = 300;

    function handleDndConsider(e: CustomEvent) {
        if (isAllQueuesFolder) return;
        folderQueueQuery.data = e.detail.items;
    }

    function handleDndFinalize(e: CustomEvent) {
        if (isAllQueuesFolder) return;
        folderQueueQuery.data = e.detail.items;

        reorderFolderQueues({
            folderId: folderState.tabId,
            orderedQueueIds: e.detail.items.map((i: any) => i.id),
        });
    }
</script>

<div
    bind:this={folderContainer}
    class="h-(--tab-content-height) flex flex-col overflow-x-auto no-scrollbar min-w-0"
>
    <Separator />

    <Resizable.PaneGroup direction="horizontal" class="min-w-0">
        <Resizable.Pane class="relative min-w-0 p-5" defaultSize={70}>
            <FolderToolbar
                {tabId}
                {tabParams}
                onEdit={() => {
                    queueEditPanel = queueEditPanel === 0 ? 30 : 0;
                }}
                onSearch={() => {
                    showSearch = !showSearch;
                }}
            />
            {#if showSearch}
                <div
                    class="absolute top-2 left-1/2 z-10 flex w-64 -translate-x-1/2 items-center rounded-md border bg-background px-2"
                >
                    <Search class="h-4 w-4 shrink-0" />
                    <Input
                        bind:ref={searchInput}
                        bind:value={searchKeyword}
                        class="border-none shadow-none dark:bg-transparent"
                        placeholder="Search queues..."
                        type="text"
                    />
                </div>
            {/if}
            <InfiniteScanLoader loading={folderQueueQuery.isFetching} />
            <Scroll.ScrollArea class="h-full @container">
                <div class="flex flex-col gap-4">
                    <div class="flex flex-col gap-2">
                        {#if !folderQueueQuery.data || folderQueueQuery.data?.length === 0}
                            <p
                                class="text-sm text-muted-foreground/50 italic mx-auto"
                            >
                                {isAllQueuesFolder
                                    ? "No queues found."
                                    : "No queues added yet."}
                            </p>
                        {/if}
                        <ul
                            data-folder-queue-grid
                            class="grid @[1600px]:grid-cols-5 @[1200px]:grid-cols-4 @[900px]:grid-cols-3 @[600px]:grid-cols-2 grid-cols-1 gap-2 p-2"
                            use:dndzone={{
                                items: folderQueueQuery.data ?? [],
                                flipDurationMs,
                                dropTargetStyle: { outline: "none" },
                            }}
                            onconsider={handleDndConsider}
                            onfinalize={handleDndFinalize}
                        >
                            {#each visibleFolderQueues as queue (queue.id)}
                                <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
                                <li
                                    data-folder-queue-card
                                    aria-label={queue.queueName}
                                    tabindex="0"
                                    onkeydowncapture={handleQueueCardKeydown}
                                    class="text-sm rounded-md h-full focus:outline-none focus-visible:ring-2 focus-visible:ring-primary"
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

        {#if queueEditPanel > 0 && !isAllQueuesFolder}
            <Resizable.Handle class="bg-background hover:bg-primary" />

            <Resizable.Pane
                class="min-w-0 p-2 ps-0 py-4"
                defaultSize={queueEditPanel}
            >
                <Scroll.ScrollArea class="h-full bg-card rounded-sm">
                    <div class="px-4 flex flex-col gap-2">
                        <div class="sticky top-0 z-100 bg-card">
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
                                        handleTitleChange(
                                            e.currentTarget.value,
                                        )}
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
                                class="flex items-center bg-background border px-2 rounded-[5px] max-lg:hidden"
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

                        <div class="flex flex-col gap-1">
                            {#each filteredQueues as queue}
                                {@const isChecked = queueIdSet.has(queue.id)}
                                <label
                                    class="flex items-center gap-2 p-2 rounded-md hover:bg-background cursor-pointer transition-colors"
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
        {/if}
    </Resizable.PaneGroup>
</div>
