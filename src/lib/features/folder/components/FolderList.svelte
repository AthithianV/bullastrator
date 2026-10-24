<script lang="ts">
    import Button from "ui/button/button.svelte";
    import * as Accordion from "ui/accordion";

    import { Workflow } from "@lucide/svelte";
    import { cn } from "shared/utils";
    import { dndzone } from "svelte-dnd-action";

    import {
        useGetAllFolders,
        useReorderFolders,
    } from "../../folder/hooks/folder.hooks.svelte";
    import { queueStore } from "queue/store/queueStore.svelte";
    import { useCreateTab } from "tabs/hooks/tab.hooks";
    import { QueueState } from "queue/store/queueContext.svelte";
    import FolderCommands from "./FolderCommands.svelte";
    import CreateEditFolder from "./CreateEditFolder.svelte";
    import { explorerStore } from "$lib/features/explorer/store/explorer.svelte";

    const { connectionId } = $props();
    const folderQuery = useGetAllFolders(() => connectionId);
    const { mutateAsync: createTab } = useCreateTab();
    const { mutateAsync: reorderFolders } = useReorderFolders();

    const folders = $derived(folderQuery.data ?? []);
    const defaultFolder = $derived(
        folders.find((folder) => folder.id === connectionId),
    );
    const reorderableFolders = $derived(
        folders.filter((folder) => folder.id !== connectionId),
    );

    function handleDndConsider(event: CustomEvent) {
        const reorderedFolders = event.detail.items;
        folderQuery.data = defaultFolder
            ? [defaultFolder, ...reorderedFolders]
            : reorderedFolders;
    }

    function handleDndFinalize(event: CustomEvent) {
        handleDndConsider(event);
        void reorderFolders({
            connectionId,
            orderedFolderIds: event.detail.items.map(
                (folder: { id: string }) => folder.id,
            ),
        });
    }

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
</script>

{#snippet folderItem(folder: (typeof folders)[number])}
        <Accordion.Item
            value={folder.id.toString()}
            class="border-b-0 last:border-b-0"
        >
            {#if explorerStore.renameFolderId === folder.id}
                <CreateEditFolder
                    connectionId={folder.connectionId}
                    folderId={folder.id}
                    folderName={folder.title}
                />
            {:else}
                <FolderCommands {folder} />
            {/if}
            <Accordion.Content class="pl-0">
                <ul class="flex flex-col gap-0">
                    {#if folder.queues && folder.queues.length > 0}
                        {#each folder.queues as queue (queue)}
                            <li
                                class="w-full flex justify-between items-center"
                            >
                                <Button
                                    class={cn(
                                        "flex gap-2 items-center",
                                        "has-[>svg]:ps-12 py-2 h-fit bg-transparent text-foreground w-full text-start justify-start",
                                        "cursor-pointer  rounded-[2px]",
                                        queueStore.makeActive(queue.id),
                                    )}
                                    onclick={() =>
                                        handleClickOnQueue(
                                            queue.id,
                                            queue.queueName,
                                            queue.connectionId,
                                        )}
                                >
                                    <Workflow class={cn("w-4 h-4")} />
                                    <span>{queue.queueName}</span>
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
{/snippet}

<Accordion.Root class="w-full" type="multiple">
    {#if defaultFolder}
        {@render folderItem(defaultFolder)}
    {/if}
    <div
        use:dndzone={{
            items: reorderableFolders,
            dropTargetStyle: { outline: "none" },
        }}
        onconsider={handleDndConsider}
        onfinalize={handleDndFinalize}
    >
        {#each reorderableFolders as folder (folder.id)}
            {@render folderItem(folder)}
        {/each}
    </div>
</Accordion.Root>
