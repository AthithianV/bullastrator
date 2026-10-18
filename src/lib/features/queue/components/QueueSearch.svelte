<script lang="ts">
    import { useGetAllQueues } from "queue/hooks/queue.hooks";
    import * as Command from "ui/command";

    import { QueueState } from "queue/store/queueContext.svelte";
    import { queueStore } from "queue/store/queueStore.svelte";
    import { globalStore } from "shared/stores/global.svelte";
    import { useCreateTab } from "tabs/hooks/tab.hooks";
    import { useGetAllFoldersByConnections } from "folder/hooks/folder.hooks.svelte";
    import Badge from "components/ui/badge/badge.svelte";
    import { Folder, Workflow } from "@lucide/svelte";
    import Button from "components/ui/button/button.svelte";

    const query = useGetAllQueues();
    const { mutateAsync: createTab } = useCreateTab();

    const connectionsWithQueues = $derived(query.data ? query.data : []);
    const folderQuery = useGetAllFoldersByConnections(() =>
        connectionsWithQueues.map((connection) => connection.id),
    );
    const folders = $derived(folderQuery.data ?? []);
    const connectionsForFilter = $derived(connectionsWithQueues);
    let selectedConnectionIds = $state<string[]>([]);
    const visibleConnections = $derived(
        selectedConnectionIds.length === 0
            ? connectionsWithQueues
            : connectionsWithQueues.filter((connection) =>
                  selectedConnectionIds.includes(connection.id),
              ),
    );
    const visibleFolders = $derived(
        selectedConnectionIds.length === 0
            ? folders
            : folders.filter((folder) =>
                  selectedConnectionIds.includes(folder.connectionId),
              ),
    );

    function toggleConnection(connectionId: string) {
        selectedConnectionIds = selectedConnectionIds.includes(connectionId)
            ? selectedConnectionIds.filter((id) => id !== connectionId)
            : [...selectedConnectionIds, connectionId];
    }

    async function openFolder(folder: (typeof folders)[number]) {
        await createTab({
            id: folder.id,
            title: folder.title,
            connectionId: folder.connectionId,
            params: {
                connectionId: folder.connectionId,
                folderName: folder.title,
                type: "FOLDER",
                folderId: folder.id,
            },
        });
        globalStore.toggleSearchCommand(false);
    }
</script>

<Command.Dialog
    bind:open={globalStore.shouldShowSearchCommand}
    class="bg-background h-full! p-5 px-3"
>
    <Command.Input
        placeholder="Type a command or search..."
        class={"bg-card px-4 py-2 mb-2"}
    />
    {#if connectionsForFilter.length > 0}
        <div class="flex flex-wrap gap-2 px-4 py-2">
            {#each connectionsForFilter as connection}
                {@const selected = selectedConnectionIds.includes(
                    connection.id,
                )}
                <Button
                    type="button"
                    aria-pressed={selected}
                    onclick={() => toggleConnection(connection.id)}
                    class={`rounded-sm border px-4! py-1! text-xs h-7 transition-colors ${
                        selected
                            ? "border-primary bg-primary text-primary-foreground"
                            : "hover:bg-accent text-accent bg-transparent"
                    }`}
                    style="border-color: {connection.color ??
                        undefined}; color: {selected
                        ? connection.color
                            ? 'white'
                            : undefined
                        : (connection.color ??
                          undefined)}; background-color: {selected
                        ? (connection.color ?? undefined)
                        : undefined}"
                >
                    {connection.name}
                </Button>
            {/each}
        </div>
    {/if}
    <Command.List class="max-h-none flex-1">
        <Command.Empty>No results found.</Command.Empty>
        {#each visibleConnections as connection}
            {@const connectionFolders = visibleFolders.filter(
                (folder) => folder.connectionId === connection.id,
            )}
            {#if connection.queues.length > 0 || connectionFolders.length > 0}
                <Command.Group>
                    {#each connectionFolders as folder}
                        <Command.Item
                            value={`${connection.id}-${connection.name}-${folder.title}`}
                            onSelect={() => openFolder(folder)}
                            class="aria-selected:bg-card! aria-selected:font-bold py-3 px-4"
                        >
                            <Folder fill={"currentColor"} />
                            <span>{folder.title}</span>

                            <Badge
                                class="ml-2 text-[10px] opacity-50 rounded-sm font-bold"
                                style="background-color: {connection.color};"
                            >
                                {connection.name}
                            </Badge>
                        </Command.Item>
                    {/each}
                    {#each connection.queues as queue}
                        <Command.Item
                            value={`${connection.id}-${connection.name}-${queue.queueName}`}
                            onSelect={async () => {
                                queueStore.setActiveNode(queue.id);
                                await createTab({
                                    id: queue.id,
                                    title: queue.queueName,
                                    connectionId: connection.id,
                                    params: QueueState.getDefaultParams(
                                        connection.id,
                                        queue.queueName,
                                    ),
                                });
                                globalStore.toggleSearchCommand(false);
                            }}
                            class="aria-selected:bg-card! aria-selected:font-bold py-3 px-4"
                        >
                            <Workflow />
                            <span>{queue.queueName}</span>

                            <Badge
                                class="ml-2 text-[10px] font-bold opacity-50 rounded-sm"
                                style="background-color: {connection.color};"
                            >
                                {connection.name}
                            </Badge>
                        </Command.Item>
                    {/each}
                </Command.Group>
            {/if}
        {/each}
    </Command.List>
</Command.Dialog>
