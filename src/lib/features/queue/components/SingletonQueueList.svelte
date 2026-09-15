<script lang="ts">
    import { Inbox, RefreshCcw, Search, Workflow } from "@lucide/svelte";
    import type { ConnectionWithQueue } from "queue/interface/queue.types";
    import { queueStore } from "queue/store/queueStore.svelte";
    import InfiniteScanLoader from "shared/components/loaders/InfiniteScanLoader.svelte";
    import { Button } from "shared/components/ui/button";
    import Input from "shared/components/ui/input/input.svelte";
    import Separator from "shared/components/ui/separator/separator.svelte";
    import { cn } from "shared/utils";

    let {
        queuesWithConnection,
        handleClickOnQueue,
        refetch,
    }: {
        queuesWithConnection: ConnectionWithQueue[];
        handleClickOnQueue: (
            queueId: string,
            queueName: string,
            connectionId: string,
        ) => void;
        refetch: (e: MouseEvent, connectionId: string) => void;
    } = $props<{
        queuesWithConnection: ConnectionWithQueue[];
        handleClickOnQueue: (
            queueId: string,
            queueName: string,
            connectionId: string,
        ) => void;
        refetch: (e: MouseEvent, connectionId: string) => void;
    }>();

    let keyword = $state("");
    let queues = $derived(
        queuesWithConnection.length === 0
            ? []
            : queuesWithConnection[0].queues.filter((q) =>
                  q.queueName.toLowerCase().includes(keyword.toLowerCase()),
              ),
    );
</script>

{#if queuesWithConnection && queuesWithConnection.length > 0}
    <div class="p-2 flex items-center gap-2 sticky top-8">
        <div
            class="flex flex-1 gap-1 items-center border px-2 rounded-md bg-accent"
        >
            <Search size={20} />
            <Input
                class="border-none bg-transparent! outline-none px-1"
                bind:value={keyword}
            />
        </div>
        <Button
            variant="ghost"
            onclick={(e) => refetch(e, queuesWithConnection[0].id)}
            disabled={queueStore.isSyncing &&
                queueStore.connectionOnSync === queuesWithConnection[0].id}
            size={"sm"}
            class={cn("h-8 w-8 py-2 px-2! cursor-pointer", "bg-accent")}
        >
            <RefreshCcw size={8} class={cn("w-4 h-4")} />
        </Button>
    </div>
    <Separator />
    <div class="relative">
        <InfiniteScanLoader
            loading={queueStore.isSyncing &&
                queueStore.connectionOnSync === queuesWithConnection[0].id}
        />
    </div>
{/if}
<ul class="flex flex-col gap-0">
    {#if queues && queues.length > 0}
        {#each queues as queue (queue.queueName)}
            <li>
                <Button
                    class={cn(
                        "flex gap-2 items-center",
                        "h-fit bg-transparent text-foreground w-full text-start justify-start",
                        "cursor-pointer rounded-[5px] truncate flex-1",
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
                    <span class="truncate">{queue.queueName}</span>
                </Button>
            </li>
        {/each}
    {:else}
        <div
            class="flex h-full w-full flex-col items-center justify-center p-8 text-center"
        >
            <div class="relative mb-4 flex items-center justify-center">
                <div
                    class="absolute h-16 w-16 animate-pulse rounded-full bg-primary/10 blur-2xl"
                ></div>

                <Inbox />
            </div>

            <h3 class="text-sm font-semibold text-foreground">
                No Queues Found
            </h3>
            <p class="mt-1 text-xs text-muted-foreground max-w-45 mb-5">
                Sync Redis instance to start managing your queues.
            </p>
        </div>
    {/if}
</ul>
