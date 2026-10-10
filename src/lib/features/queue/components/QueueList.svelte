<script lang="ts">
    import * as Accordion from "ui/accordion";
    import Button from "ui/button/button.svelte";

    import {
        ChevronRight,
        Folder,
        FolderOpen,
        Plus,
        RefreshCcw,
        Workflow,
    } from "@lucide/svelte";
    import { cn } from "shared/utils";

    import SidebarWrapper from "components/wrapper/SidebarWrapper.svelte";
    import { queueStore } from "../store/queueStore.svelte";

    import EmptyConnectionSideBar from "connection/components/EmptyConnectionSideBar.svelte";
    import { connectionStore } from "connection/store/connection.store.svelte";
    import { QueueState } from "queue/store/queueContext.svelte";
    import InfiniteScanLoader from "shared/components/loaders/InfiniteScanLoader.svelte";
    import { useCreateTab } from "tabs/hooks/tab.hooks";
    import FolderList from "../../folder/components/FolderList.svelte";
    import { useCreateFolder } from "../../folder/hooks/folder.hooks.svelte";
    import { useGetAllQueues, useSyncAllQueues } from "../hooks/queue.hooks";
    import DialogWrapper from "$lib/shared/components/wrapper/DialogWrapper.svelte";
    import ConnectionForm from "$lib/features/connection/components/ConnectionForm.svelte";

    const query = useGetAllQueues();
    const { mutateAsync: syncQuery } = useSyncAllQueues();
    const { mutateAsync: createTab } = useCreateTab();
    const { mutateAsync: createFolder } = useCreateFolder();

    async function handleClickOnQueue(
        queueId: string,
        queueName: string,
        connectionId: string,
    ) {
        await createTab({
            id: queueId,
            title: queueName,
            connectionId: connectionId,
            params: QueueState.getDefaultParams(connectionId, queueName),
        });
        queueStore.setActiveNode(queueId);
    }

    async function createNewFolder(e: MouseEvent, connectionId: string) {
        e.stopPropagation();
        const newFolder = await createFolder({
            title: "Untitled",
            connectionId,
        });
        if (newFolder) {
            await createTab({
                id: newFolder.id,
                title: newFolder.title,
                connectionId: connectionId,
                params: {
                    connectionId: connectionId,
                    folderName: newFolder.title,
                    type: "FOLDER",
                    folderId: newFolder.id,
                },
            });
        }
    }

    async function refetch(e: MouseEvent, connectionId: string) {
        e.stopPropagation();
        await syncQuery(connectionId);
    }
</script>

<SidebarWrapper>
    {#snippet header()}
        <h1 class="font-semibold">Queues</h1>
        <div>
            <DialogWrapper
                title="Add new Connection"
                description="Complete to add new Connection"
            >
                {#snippet trigger()}
                    <Plus class="h-4" />
                {/snippet}

                {#snippet children(onComplete)}
                    <ConnectionForm {onComplete} />
                {/snippet}
            </DialogWrapper>
        </div>
    {/snippet}

    {#snippet content()}
        {#if query.isLoading}
            <InfiniteScanLoader />
        {:else if query.isError}
            <div class="flex justify-center items-center h-full py-2">
                <p>Error: {query.error.message}</p>
            </div>
        {:else if query.data && query.data.length === 0}
            <EmptyConnectionSideBar />
        {:else}
            <Accordion.Root class="w-full " type="multiple">
                {#each query.data as conn (conn.id)}
                    <Accordion.Item
                        value={conn.id.toString()}
                        class="border-b-0 last:border-b-0 "
                    >
                        <div
                            class={cn(
                                "sticky top-10 z-10",
                                queueStore.makeActive(conn.id),
                            )}
                        >
                            <Accordion.Trigger
                                class={cn(
                                    "flex-1 flex items-center justify-end flex-row-reverse",
                                    "px-2 py-2",
                                    "hover:no-underline",
                                    "cursor-pointer rounded-[2px]",
                                    "group ",
                                )}
                                onclick={() =>
                                    queueStore.setActiveNode(conn.id)}
                            >
                                <div class="flex-1 flex items-center gap-2">
                                    <Folder
                                        class="w-4 h-4 block group-data-[state=open]:hidden"
                                        fill="currentColor"
                                    />

                                    <FolderOpen
                                        class="w-4 h-4 hidden group-data-[state=open]:block"
                                    />

                                    <div
                                        class="flex-1 flex justify-between items-center"
                                    >
                                        <span class="font-semibold truncate"
                                            >{conn.name}</span
                                        >
                                        <div class="flex gap-2 ms-2">
                                            <Button
                                                variant="ghost"
                                                onclick={(e: MouseEvent) =>
                                                    createNewFolder(e, conn.id)}
                                                disabled={queueStore.isSyncing &&
                                                    queueStore.connectionOnSync ===
                                                        conn.id}
                                                size={"sm"}
                                                class={cn(
                                                    "h-[70%]! p-0.5 w-2 cursor-pointer",
                                                    "hidden group-hover:flex",
                                                    "dark:hover:bg-slate-700! hover:bg-slate-200!",
                                                )}
                                            >
                                                <Plus
                                                    size={5}
                                                    class={cn("w-4 h-4")}
                                                />
                                            </Button>
                                            <Button
                                                variant="ghost"
                                                onclick={(e) =>
                                                    refetch(e, conn.id)}
                                                disabled={queueStore.isSyncing &&
                                                    queueStore.connectionOnSync ===
                                                        conn.id}
                                                size={"sm"}
                                                class={cn(
                                                    "h-[70%]! p-0.5 w-2 cursor-pointer",
                                                    "hidden group-hover:flex",
                                                    "dark:hover:bg-slate-700! hover:bg-slate-200!",
                                                )}
                                            >
                                                <RefreshCcw
                                                    size={8}
                                                    class={cn("w-4 h-4")}
                                                />
                                            </Button>
                                        </div>
                                    </div>
                                </div>
                            </Accordion.Trigger>
                            <InfiniteScanLoader
                                loading={queueStore.isSyncing &&
                                    queueStore.connectionOnSync === conn.id}
                            />
                        </div>
                        <Accordion.Content class={cn("pl-0")}>
                            <Accordion.Root class="w-full" type="single">
                                <Accordion.Item
                                    value={conn.id.toString() + "all"}
                                    class="border-b-0 last:border-b-0"
                                >
                                    <div
                                        role="button"
                                        tabindex="0"
                                        class={cn(
                                            "sticky top-0 z-10 ps-6 flex items-center gap-2 w-full group",
                                            queueStore.makeActive(
                                                conn.id + "all",
                                            ),
                                        )}
                                        onmousedown={(e) => {
                                            e.preventDefault();
                                            queueStore.setActiveNode(
                                                conn.id + "all",
                                            );
                                        }}
                                    >
                                        <Accordion.Trigger
                                            class={cn(
                                                "flex items-center justify-center py-1 p-1 rounded dark:hover:bg-slate-700 bg-transparent!",
                                                "[&[data-state=open]>svg]:rotate-90 transition-transform",
                                            )}
                                        >
                                            <ChevronRight
                                                class="w-4 h-4 block"
                                            />
                                        </Accordion.Trigger>

                                        <div
                                            role="button"
                                            tabindex="0"
                                            class="flex-1 flex justify-between items-center cursor-pointer py-1.5"
                                        >
                                            <div
                                                class="flex gap-2 items-center"
                                            >
                                                <Folder class="w-4 h-4" />
                                                <span
                                                    class="font-semibold truncate"
                                                    >All</span
                                                >
                                            </div>
                                        </div>
                                    </div>
                                    <Accordion.Content class="pl-0">
                                        <ul class="flex flex-col gap-0">
                                            {#if conn.queues && conn.queues.length > 0}
                                                {#each conn.queues as queue (queue.queueName)}
                                                    <li>
                                                        <Button
                                                            class={cn(
                                                                "flex gap-2 items-center",
                                                                "has-[>svg]:ps-12 py-2 h-fit bg-transparent text-foreground w-full text-start justify-start",
                                                                "cursor-pointer  rounded-[2px]",
                                                                queueStore.makeActive(
                                                                    queue.id,
                                                                ),
                                                            )}
                                                            onclick={() =>
                                                                handleClickOnQueue(
                                                                    queue.id,
                                                                    queue.queueName,
                                                                    conn.id,
                                                                )}
                                                        >
                                                            <Workflow
                                                                class={cn(
                                                                    "w-4 h-4",
                                                                )}
                                                            />
                                                            <span
                                                                >{queue.queueName}</span
                                                            >
                                                        </Button>
                                                    </li>
                                                {/each}
                                            {:else}
                                                <div
                                                    class="text-xs text-center font-semibold text-muted-forground p-2"
                                                >
                                                    No queues found
                                                </div>
                                            {/if}
                                        </ul>
                                    </Accordion.Content>
                                </Accordion.Item>
                                <FolderList connectionId={conn.id} />
                            </Accordion.Root>
                        </Accordion.Content>
                    </Accordion.Item>
                {/each}
            </Accordion.Root>
        {/if}
    {/snippet}
</SidebarWrapper>
