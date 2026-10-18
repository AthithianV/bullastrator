<script lang="ts">
    import { useGetAllQueues } from "queue/hooks/queue.hooks";
    import * as Command from "ui/command";

    import { QueueState } from "queue/store/queueContext.svelte";
    import { queueStore } from "queue/store/queueStore.svelte";
    import { globalStore } from "shared/stores/global.svelte";
    import { useCreateTab } from "tabs/hooks/tab.hooks";
    import Badge from "components/ui/badge/badge.svelte";
    import { Workflow } from "@lucide/svelte";
    import Button from "components/ui/button/button.svelte";

    const query = useGetAllQueues();
    const { mutateAsync: createTab } = useCreateTab();

    const connectionsWithQueues = $derived(query.data ? query.data : []);
    const connectionsForFilter = $derived(
        connectionsWithQueues.filter(
            (connection) => connection.queues.length > 0,
        ),
    );
    let selectedConnectionIds = $state<string[]>([]);
    const visibleConnections = $derived(
        selectedConnectionIds.length === 0
            ? connectionsWithQueues
            : connectionsWithQueues.filter((connection) =>
                  selectedConnectionIds.includes(connection.id),
              ),
    );

    function toggleConnection(connectionId: string) {
        selectedConnectionIds = selectedConnectionIds.includes(connectionId)
            ? selectedConnectionIds.filter((id) => id !== connectionId)
            : [...selectedConnectionIds, connectionId];
    }
</script>

<Command.Dialog
    bind:open={globalStore.shouldShowSearchCommand}
    class="bg-background"
>
    <Command.Input placeholder="Type a command or search..." />
    {#if connectionsForFilter.length > 0}
        <div class="flex flex-wrap gap-2 px-2 py-2">
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
    <Command.List>
        <Command.Empty>No results found.</Command.Empty>
        {#each visibleConnections as connection}
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
                            class="aria-selected:bg-card! aria-selected:font-bold py-3 px-4"
                        >
                            <Workflow />
                            <span>{queue.queueName}</span>

                            <Badge
                                class="ml-2 text-xs opacity-50 rounded-sm"
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
