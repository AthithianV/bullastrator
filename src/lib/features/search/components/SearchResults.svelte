<script lang="ts">
    import { Folder, Workflow } from "@lucide/svelte";
    import * as Command from "ui/command";
    import Button from "components/ui/button/button.svelte";
    import Badge from "components/ui/badge/badge.svelte";
    import { useGetAllFoldersByConnections } from "folder/hooks/folder.hooks.svelte";
    import { useGetAllQueues } from "queue/hooks/queue.hooks";
    import { QueueState } from "queue/store/queueContext.svelte";
    import { queueStore } from "queue/store/queueStore.svelte";
    import { globalStore } from "shared/stores/global.svelte";
    import { useCreateTab } from "tabs/hooks/tab.hooks";

    const { closeOnSelect = false }: { closeOnSelect?: boolean } = $props();

    const query = useGetAllQueues();
    const { mutateAsync: createTab } = useCreateTab();
    const connections = $derived(query.data ?? []);
    const folderQuery = useGetAllFoldersByConnections(() =>
        connections.map((connection) => connection.id),
    );
    const folders = $derived(folderQuery.data ?? []);

    let selectedConnectionIds = $state<string[]>([]);
    const visibleConnections = $derived(
        selectedConnectionIds.length === 0
            ? connections
            : connections.filter((connection) =>
                  selectedConnectionIds.includes(connection.id),
              ),
    );

    function toggleConnection(connectionId: string) {
        selectedConnectionIds = selectedConnectionIds.includes(connectionId)
            ? selectedConnectionIds.filter((id) => id !== connectionId)
            : [...selectedConnectionIds, connectionId];
    }

    function closeAfterSelect() {
        if (closeOnSelect) {
            globalStore.toggleSearchCommand(false);
        }
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
        closeAfterSelect();
    }

    async function openQueue(
        connectionId: string,
        queue: (typeof connections)[number]["queues"][number],
    ) {
        queueStore.setActiveNode(queue.id);
        await createTab({
            id: queue.id,
            title: queue.queueName,
            connectionId,
            params: QueueState.getDefaultParams(connectionId, queue.queueName),
        });
        closeAfterSelect();
    }
</script>

<Command.Input placeholder="Type a command or search..." class="px-4 py-2 mb-2 w-full" />

{#if connections.length > 0}
    <div class="flex flex-wrap gap-2 py-2">
        {#each connections as connection}
            {@const selected = selectedConnectionIds.includes(connection.id)}
            <Button
                type="button"
                aria-pressed={selected}
                onclick={() => toggleConnection(connection.id)}
                class={`rounded-sm border px-4! py-1! text-xs h-7 transition-colors ${
                    selected
                        ? "border-primary bg-primary text-primary-foreground"
                        : "hover:bg-accent text-accent bg-transparent"
                }`}
                style="border-color: {connection.color ?? undefined}; color: {selected
                    ? connection.color
                        ? 'white'
                        : undefined
                    : (connection.color ?? undefined)}; background-color: {selected
                    ? (connection.color ?? undefined)
                    : undefined}"
            >
                {connection.name}
            </Button>
        {/each}
    </div>
{/if}

<Command.List class="max-h-none flex-1 overflow-y-auto">
    <Command.Empty>No results found.</Command.Empty>
    {#each visibleConnections as connection}
        {@const connectionFolders = folders.filter(
            (folder) => folder.connectionId === connection.id,
        )}
        {#if connection.queues.length > 0 || connectionFolders.length > 0}
            <Command.Group heading={connection.name}>
                {#each connectionFolders as folder}
                    <Command.Item
                        value={`${connection.name} ${folder.title}`}
                        onSelect={() => openFolder(folder)}
                        class="aria-selected:bg-card! aria-selected:font-bold py-3 px-4"
                    >
                        <Folder fill="currentColor" />
                        <span>{folder.title}</span>
                        <Badge
                            class="ml-2 text-[10px] font-bold opacity-80 rounded-sm text-white"
                            style="background-color: {connection.color};"
                        >
                            {connection.name}
                        </Badge>
                    </Command.Item>
                {/each}
                {#each connection.queues as queue}
                    <Command.Item
                        value={`${connection.name} ${queue.queueName}`}
                        onSelect={() => openQueue(connection.id, queue)}
                        class="aria-selected:bg-card! aria-selected:font-bold py-3 px-4"
                    >
                        <Workflow />
                        <span>{queue.queueName}</span>
                        <Badge
                            class="ml-2 text-[10px] font-bold opacity-80 rounded-sm text-white"
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
