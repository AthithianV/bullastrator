<script lang="ts">
    import SidebarWrapper from "components/wrapper/SidebarWrapper.svelte";

    import { useGetAllQueues } from "queue/hooks/queue.hooks";
    import * as Command from "ui/command";
    import * as Avatar from "shared/components/ui/avatar";

    import { Search } from "@lucide/svelte";
    import { QueueState } from "queue/store/queueContext.svelte";
    import { queueStore } from "queue/store/queueStore.svelte";
    import { Input } from "shared/components/ui/input";
    import { globalStore } from "shared/stores/global.svelte";
    import { useCreateTab } from "tabs/hooks/tab.hooks";
    import { cn } from "tailwind-variants";
    import Badge from "$lib/shared/components/ui/badge/badge.svelte";

    let keyword = $state("");
    const query = useGetAllQueues();
    const { mutateAsync: createTab } = useCreateTab();

    const avatarStyle = (color?: string) =>
        `background-color: ${color || "var(--primary)"}`;

    const connectionsWithQueues = $derived(query.data ? query.data : []);
</script>

<SidebarWrapper>
    {#snippet header()}
        <h1>Search</h1>
        <div></div>
    {/snippet}

    {#snippet content()}
        <div class="p-2 sticky top-8 bg-background z-10">
            <div class="flex flex-1 gap-1 items-center px-2 rounded-md">
                <Search size={20} />
                <Input
                    class="border-none bg-transparent! outline-none px-1"
                    bind:value={keyword}
                />
            </div>
        </div>
        <Command.Root
            class="h-full! max-h-full! border-none bg-transparent z-0"
            shouldFilter={true}
        >
            <Command.Input
                placeholder="Type search keyword..."
                class="h-0 w-full bg-transparent outline-none hidden border-none!"
                bind:value={keyword}
            />

            <Command.List class="flex-1 overflow-y-auto max-h-full">
                <Command.Empty>No results found.</Command.Empty>
                {#each connectionsWithQueues as connection}
                    <Command.Group heading={connection.name}>
                        {#each connection.queues as queue}
                            <Command.Item
                                value={`${connection.id}-${connection.name}-${queue.queueName}`}
                                class={cn(
                                    "hover:bg-card! text-base hover:text-current! py-2",
                                )}
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
                            >
                                <Avatar.Root class="h-10 w-10 rounded-lg">
                                    <Avatar.Fallback
                                        class="rounded-lg text-white font-bold text-xs"
                                        style={avatarStyle(connection.color)}
                                    >
                                        {connection.name
                                            ?.substring(0, 2)
                                            .toUpperCase()}
                                    </Avatar.Fallback>
                                </Avatar.Root>

                                <div class="flex flex-col gap-2">
                                    <span>{queue.queueName}</span>

                                    <Badge
                                        style={avatarStyle(connection.color)}
                                        class="text-xs opacity-80 text-white"
                                    >
                                        {connection.name}
                                    </Badge>
                                </div>
                            </Command.Item>
                        {/each}
                    </Command.Group>
                {/each}
            </Command.List>
        </Command.Root>
    {/snippet}
</SidebarWrapper>
