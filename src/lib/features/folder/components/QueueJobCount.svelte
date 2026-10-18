<script lang="ts">
    import type { JobStatus } from "job/interface/job.types";
    import type { ReadQueueWithCounts } from "queue/interface/queue.types";
    import { QueueState } from "queue/store/queueContext.svelte";
    import { Button } from "shared/components/ui/button";
    import { Card, CardHeader } from "shared/components/ui/card";
    import { cn } from "shared/utils";
    import {
        useCreateTab,
        useGetTabs,
        useUpdateTab,
    } from "tabs/hooks/tab.hooks";
    import { QUEUE_STATUS_TAB } from "queue/constants/queueConstants";
    import { Play } from "@lucide/svelte";

    let { queue }: { queue: ReadQueueWithCounts } = $props();

    const { mutateAsync: createTab } = useCreateTab();
    const { mutateAsync: updateTab } = useUpdateTab();

    let counts = $derived(queue.counts);
    let total = $derived(counts.total);

    const tabQuery = useGetTabs();

    async function handleClickOnQueue(
        queueId: string,
        queueName: string,
        connectionId: string,
        state: JobStatus,
    ) {
        const tabExists = tabQuery.data?.find((t) => t.id === queueId);

        if (!tabExists) {
            await createTab({
                id: queueId,
                title: queueName,
                connectionId: connectionId,
                params: {
                    ...QueueState.getDefaultParams(connectionId, queueName),
                    state,
                },
            });
        } else {
            await updateTab({
                id: tabExists.id,
                data: {
                    params: {
                        connectionId,
                        type: "QUEUE",
                        queueName: queueName,
                        state,
                    },
                },
            });
        }
    }
</script>

<Card
    class="flex flex-col gap-2 p-4 w-full h-full flex-1 rounded-md border-2 border-card shadow-sm transition-colors bg-card {counts.isPaused &&
        'opacity-70'}"
>
    <CardHeader class="h-fit ps-0"
        ><Button
            variant="ghost"
            onclick={() =>
                handleClickOnQueue(
                    queue.id,
                    queue.queueName,
                    queue.connectionId,
                    "active",
                )}
            class={cn(
                "font-semibold truncate ps-0 text-wrap! text-left",
                "w-fit max-w-full h-fit!",
                "hover:underline hover:text-primary hover:bg-transparent!",
                counts.isPaused ? "text-muted-foreground" : "",
            )}
        >
            {#if counts.isPaused}
                <Play fill={"currentColor"} class="h-3 w-3" />
            {/if}
            {queue.queueName}
        </Button></CardHeader
    >
    <div class="h-5 w-full flex rounded-md overflow-hidden shadow-sm">
        {#if total > 0}
            {#each QUEUE_STATUS_TAB as { value, badgeColor, border, hover }}
                {#if counts[value] > 0}
                    <Button
                        variant="ghost"
                        style="flex: {counts[value]} 1 0%; min-width: 20px;"
                        class={cn(
                            badgeColor,
                            border,
                            hover,
                            "hover:bg-current",
                            "transition-all duration-300",
                            "flex items-center justify-center",
                            "min-w-1",
                            "h-full! p-0!",
                            "rounded-none border-none",
                            "shadow-none ring-offset-0 focus-visible:ring-0",
                        )}
                        onclick={() =>
                            handleClickOnQueue(
                                queue.id,
                                queue.queueName,
                                queue.connectionId,
                                value,
                            )}
                    >
                        {#if (counts[value] / total) * 100 > 0}
                            <span
                                class="text-xs text-white font-bold pointer-events-none"
                            >
                                {counts[value]}
                            </span>
                        {/if}
                    </Button>
                {/if}
            {/each}
        {:else}
            <div
                class="w-full h-full flex items-center justify-center bg-gray-50 italic text-gray-400 text-[10px]"
            >
                No jobs in queue
            </div>
        {/if}
    </div>

    <div class="flex justify-between items-center px-1">
        <div
            class="flex items-center gap-3 text-[12px] font-medium text-muted-foreground"
        >
            <span>
                {total.toLocaleString()}
                {total === 1 ? "Job" : "Jobs"}
            </span>
        </div>
        <span
            class={cn(
                "text-[11px] font-semibold flex justify-center items-center gap-1",
                counts.isPaused ? "text-gray-500" : "text-emerald-500",
            )}
        >
            {#if counts.isPaused}
                <Play fill={"currentColor"} class="h-3 w-3" />
                {"Paused"}
            {/if}
        </span>
    </div>
</Card>
