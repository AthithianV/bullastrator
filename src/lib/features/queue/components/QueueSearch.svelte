<script lang="ts">
  import { useGetAllQueues } from "queue/hooks/queue.hooks";
  import * as Command from "ui/command";

  import { QueueState } from "queue/store/queueContext.svelte";
  import { queueStore } from "queue/store/queueStore.svelte";
  import { globalStore } from "shared/stores/global.svelte";
  import { useCreateTab } from "tabs/hooks/tab.hooks";

  const query = useGetAllQueues();
  const { mutateAsync: createTab } = useCreateTab();

  const connectionsWithQueues = $derived(query.data ? query.data : []);
</script>

<Command.Dialog bind:open={globalStore.shouldShowSearchCommand}>
  <Command.Input placeholder="Type a command or search..." />
  <Command.List>
    <Command.Empty>No results found.</Command.Empty>
    {#each connectionsWithQueues as connection}
      <Command.Group heading={connection.connectionName}>
        {#each connection.queues as queue}
          <Command.Item
            value={`${connection.id}-${connection.connectionName}-${queue.queueName}`}
            onSelect={async () => {
              queueStore.setActiveNode(queue.id);
              const result = await createTab({
                id: queue.id,
                data: {
                  title: queue.queueName,
                  connectionId: connection.id,
                  params: QueueState.getDefaultParams(
                    connection.id,
                    queue.queueName,
                  ),
                },
              });
              globalStore.toggleSearchCommand(false);
            }}
          >
            <span>{queue.queueName}</span>

            <span class="ml-2 text-xs text-muted-foreground opacity-50">
              ({connection.connectionName})
            </span>
          </Command.Item>
        {/each}
      </Command.Group>
    {/each}
  </Command.List>
</Command.Dialog>
