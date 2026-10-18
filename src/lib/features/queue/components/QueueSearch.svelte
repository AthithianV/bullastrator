<script lang="ts">
    import { useGetAllQueues } from "queue/hooks/queue.hooks";
    import * as Command from "ui/command";

    import { QueueState } from "queue/store/queueContext.svelte";
    import { queueStore } from "queue/store/queueStore.svelte";
    import { globalStore } from "shared/stores/global.svelte";
    import { useCreateTab } from "tabs/hooks/tab.hooks";
    import Badge from "components/ui/badge/badge.svelte";
    import { Workflow } from "@lucide/svelte";

    const query = useGetAllQueues();
    const { mutateAsync: createTab } = useCreateTab();

    const connectionsWithQueues = $derived(query.data ? query.data : []);
</script>

<Command.Dialog bind:open={globalStore.shouldShowSearchCommand}>
    <Command.Input placeholder="Type a command or search..." />
    <Command.List>
        <Command.Empty>No results found.</Command.Empty>
        {#each connectionsWithQueues as connection}
            {#if connection.queues.length > 0}
                <Command.Group heading={connection.name}>
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
                            class="aria-selected:bg-card! aria-selected:font-bold"
                        >
                            <Workflow />
                            <span>{queue.queueName}</span>

                            <Badge class="ml-2 text-xs opacity-50 rounded-sm">
                                {connection.name}
                            </Badge>
                        </Command.Item>
                    {/each}
                </Command.Group>
            {/if}
        {/each}
    </Command.List>
</Command.Dialog>
