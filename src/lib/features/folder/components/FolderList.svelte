<script lang="ts">
    import Button from "ui/button/button.svelte";
    import * as Accordion from "ui/accordion";

    import {
        ChevronDown,
        ChevronRight,
        Eye,
        Folder,
        FolderOpen,
        Trash2,
        Workflow,
    } from "@lucide/svelte";
    import { cn } from "shared/utils";

    import {
        useDeleteFolder,
        useGetAllFolders,
    } from "../../folder/hooks/folder.hooks.svelte";
    import { queueStore } from "queue/store/queueStore.svelte";
    import { useCreateTab } from "tabs/hooks/tab.hooks";
    import { QueueState } from "queue/store/queueContext.svelte";

    const { connectionId } = $props();
    const folderQuery = useGetAllFolders(connectionId);
    const { mutateAsync: createTab } = useCreateTab();
    const { mutateAsync: deleteFolder } = useDeleteFolder();

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

    async function openFolder(
        e: MouseEvent | Event,
        folderId: string,
        title: string,
    ) {
        e.stopPropagation();
        await createTab({
            id: folderId,
            data: {
                title: title,
                connectionId: connectionId,
                params: {
                    connectionId: connectionId,
                    folderName: title,
                    type: "FOLDER",
                    folderId: folderId,
                },
            },
        });
    }
</script>

<Accordion.Root class="w-full" type="multiple">
    {#each folderQuery.data ?? [] as folder (folder.id)}
        <Accordion.Item
            value={folder.id.toString()}
            class="border-b-0 last:border-b-0"
        >
            <div
                role="button"
                tabindex="0"
                class={cn(
                    "ps-6 flex items-center gap-2 w-full group transition-colors",
                    queueStore.makeActive(folder.id),
                )}
                onmousedown={(e) => {
                    e.preventDefault();
                    queueStore.setActiveNode(folder.id);
                    openFolder(e, folder.id, folder.title);
                }}
            >
                <Accordion.Trigger
                    class={cn(
                        "flex items-center justify-center py-1 p-1 rounded hover:bg-slate-200 dark:hover:bg-slate-700",
                        "[&[data-state=open]>svg]:rotate-90 transition-transform",
                    )}
                >
                    <ChevronRight class="w-4 h-4 block" />
                </Accordion.Trigger>

                <div
                    class="flex-1 flex justify-between items-center cursor-pointer py-1"
                >
                    <div class="flex gap-2 items-center">
                        <Folder class="w-4 h-4" />
                        <span class="font-semibold truncate"
                            >{folder.title}</span
                        >
                    </div>

                    <div class="px-2 opacity-0 group-hover:opacity-100 gap-2">
                        <Button
                            variant="ghost"
                            class="h-6 w-6 p-1 text-red-500 hover:text-red-600 hover:p-1 rounded hover:bg-slate-200 dark:hover:bg-slate-700"
                            onclick={(e) => {
                                e.stopPropagation(); // Prevents openFolder from firing
                                deleteFolder(folder.id);
                            }}
                        >
                            <Trash2 class="w-4 h-4" />
                        </Button>
                    </div>
                </div>
            </div>
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
                                    <span
                                        >{queue.displayName ??
                                            queue.queueName}</span
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
    {/each}
</Accordion.Root>
