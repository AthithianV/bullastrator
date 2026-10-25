<script lang="ts">
    import * as Accordion from "ui/accordion";

    import { Folder, FolderOpen, Plus } from "@lucide/svelte";
    import { cn } from "shared/utils";

    import SidebarWrapper from "components/wrapper/SidebarWrapper.svelte";

    import EmptyConnectionSideBar from "connection/components/EmptyConnectionSideBar.svelte";
    import InfiniteScanLoader from "shared/components/loaders/InfiniteScanLoader.svelte";
    import FolderList from "../../folder/components/FolderList.svelte";
    import ConnectionForm from "$lib/features/connection/components/ConnectionForm.svelte";
    import { useGetAllQueues } from "queue/hooks/queue.hooks";
    import { queueStore } from "queue/store/queueStore.svelte";
    import ConnectionCommands from "connection/components/ConnectionCommands.svelte";
    import DialogWrapper from "components/wrapper/DialogWrapper.svelte";
    import CreateEditFolder from "folder/components/CreateEditFolder.svelte";

    const query = useGetAllQueues();
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
                                "sticky top-9 z-10 bg-background",
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
                                onclick={() => {
                                    queueStore.setActiveNode(conn.id);
                                }}
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
                                        <ConnectionCommands
                                            connection={conn}
                                            className="opacity-0 transition-opacity group-hover:opacity-100 group-focus-within:opacity-100"
                                        />
                                    </div>
                                </div>
                            </Accordion.Trigger>
                            <InfiniteScanLoader
                                loading={queueStore.isSyncing &&
                                    queueStore.connectionOnSync === conn.id}
                            />
                        </div>
                        <CreateEditFolder connectionId={conn.id} />
                        <Accordion.Content class={cn("pl-0")}>
                            <FolderList connectionId={conn.id} />
                        </Accordion.Content>
                    </Accordion.Item>
                {/each}
            </Accordion.Root>
        {/if}
    {/snippet}
</SidebarWrapper>
