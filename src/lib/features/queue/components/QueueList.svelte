<script lang="ts">
    import * as Accordion from "ui/accordion";
    import Button from "ui/button/button.svelte";

    import { Folder, FolderOpen, Plus, RefreshCcw } from "@lucide/svelte";
    import { cn } from "shared/utils";

    import SidebarWrapper from "components/wrapper/SidebarWrapper.svelte";
    import { queueStore } from "../store/queueStore.svelte";

    import EmptyConnectionSideBar from "connection/components/EmptyConnectionSideBar.svelte";
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
                                    "group",
                                )}
                                onclick={() =>
                                    queueStore.setActiveNode(conn.id)}
                            >
                                <div class="flex-1 flex items-center gap-2">
                                    <span
                                        class="h-2 w-2 rounded-full"
                                        style="background-color: {conn.color};"
                                    ></span>
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
                            <FolderList connectionId={conn.id} />
                        </Accordion.Content>
                    </Accordion.Item>
                {/each}
            </Accordion.Root>
        {/if}
    {/snippet}
</SidebarWrapper>
