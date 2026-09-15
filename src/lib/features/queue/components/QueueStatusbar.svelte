<script lang="ts">
    import * as Tabs from "ui/tabs";
    import * as Scroll from "ui/scroll-area/index";
    import { useGetJobCountInQueue } from "job/hooks/job.hooks.svelte";
    import type { JobStatus } from "job/interface/job.types";
    import { cn } from "tailwind-variants";
    import JobStatusIcon from "components/icons/QueueStatusIcon.svelte";
    import { QUEUE_STATUS_TAB } from "../constants/queueConstants";
    import { useQueueState } from "queue/store/queueContext.svelte";

    let queueState = useQueueState();
    const queueJobsQuery = useGetJobCountInQueue();

    $effect(() => {
        if (queueJobsQuery.data) {
            queueState.jobCounts = queueJobsQuery.data;
        }
    });
</script>

<Scroll.ScrollArea class="bg-muted/10 w-full">
    <div class="p-2 py-1 h-(--queue-tab-height)">
        <Tabs.Root bind:value={queueState.currentStatus}>
            <Tabs.List
                class="w-full justify-start h-auto p-0 bg-transparent gap-2"
            >
                {#each QUEUE_STATUS_TAB as tab}
                    <Tabs.Trigger
                        value={tab.value}
                        class={cn(
                            "px-5",
                            tab.color,
                            tab.border,
                            tab.background,
                        )}
                    >
                        <JobStatusIcon
                            status={tab.value as JobStatus}
                            size={12}
                            className={tab.color}
                        />

                        <span class={tab.color}>{tab.label}</span>

                        <span
                            class={cn(
                                " text-black",
                                "text-[10px]",
                                "font-bold px-1.5 rounded-sm min-w-5",
                                "text-center",
                                tab.badgeColor,
                                "relative top-px",
                            )}
                        >
                            {queueState.jobCounts?.[tab.countKey] ?? 0}
                        </span>
                    </Tabs.Trigger>
                {/each}
            </Tabs.List>
        </Tabs.Root>
    </div>
    <Scroll.Scrollbar orientation="horizontal" />
</Scroll.ScrollArea>
