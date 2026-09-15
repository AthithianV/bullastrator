<script lang="ts">
    import * as Tabs from "ui/tabs";
    import * as Scroll from "ui/scroll-area/index";
    import Button from "ui/button/button.svelte";
    import { Spinner } from "ui/spinner";

    import { CircleAlert, X } from "@lucide/svelte";

    import JobTimeline from "job/components/JobTimeline.svelte";
    import { cn } from "tailwind-variants";

    import { formatDate } from "helpers/date";
    import DataEditor from "components/editor/Editor.svelte";

    import { useGetJobById, useGetJobLog } from "../hooks/job.hooks.svelte";
    import { useQueueState } from "queue/store/queueContext.svelte";
    import TooltipTrigger from "shared/components/ui/tooltip/tooltip-trigger.svelte";
    import TooltipContent from "shared/components/ui/tooltip/tooltip-content.svelte";
    import Tooltip from "shared/components/ui/tooltip/tooltip.svelte";
    import { extractLink, isLink } from "shared/helpers/linkify";
    import JobError from "./JobError.svelte";
    import JobData from "./JobData.svelte";
    import InfiniteScanLoader from "shared/components/loaders/InfiniteScanLoader.svelte";
    import Card from "$lib/shared/components/ui/card/card.svelte";

    const queueState = useQueueState();
    const selectedJobQuery = useGetJobById();
    const selectedJobLog = useGetJobLog();

    $effect(() => {
        if (selectedJobQuery.data) {
            queueState.active.selectedJobDetails = selectedJobQuery.data;
        }
    });

    $inspect(queueState.active.selectedJobDetails);
</script>

{#if queueState.active.selectedJobDetails}
    <div
        class="h-full flex flex-col overflow-auto min-w-0 select-text relative"
    >
        <InfiniteScanLoader loading={selectedJobQuery.isLoading} />
        <div
            class="px-6 py-2 border-b flex justify-between items-start"
        >
            <div>
                <div class="flex items-center gap-3 mb-1">
                    <h1 class="text-xl font-bold flex items-center gap-2">
                        {#if queueState.active.selectedJobDetails.failedReason}
                            <Tooltip>
                                <TooltipTrigger class="text-red-500">
                                    <CircleAlert class="h-5 w-5" />
                                </TooltipTrigger>
                                <TooltipContent class="p-0">
                                    <div
                                        class={cn(
                                            "mt-3 p-2 rounded",
                                            "text-sm text-wrap text-red-500",
                                            "flex items-center gap-2",
                                            "bg-red-50 dark:bg-red-950/20",
                                            "border border-red-100 dark:border-red-900",
                                            "max-w-[30vw] max-h-500px overflow-auto",
                                        )}
                                    >
                                        <span
                                            class="font-semibold overflow-x-auto"
                                        >
                                            {#each extractLink(queueState.active.selectedJobDetails.failedReason) as part}
                                                {#if isLink(part)}
                                                    <a
                                                        href={part}
                                                        target="_blank"
                                                        rel="noopener noreferrer"
                                                        class="text-blue-800 underline break-all"
                                                    >
                                                        {part}
                                                    </a>
                                                {:else}
                                                    {part}
                                                {/if}
                                            {/each}
                                        </span>
                                    </div>
                                </TooltipContent>
                            </Tooltip>
                        {/if}
                        {queueState.active.selectedJobDetails.name}
                    </h1>
                </div>
                <p class="text-sm text-muted-foreground font-mono">
                    ID: {queueState.active.selectedJobDetails.id} • {formatDate(
                        queueState.active.selectedJobDetails.timestamp,
                    )}
                </p>
            </div>
            <div>
                <Button
                    variant={"ghost"}
                    class="cursor-pointer hover:bg-primary/20 hover:text-red-500"
                    size="lg"
                    onclick={() => {
                        queueState.active.selectedJob = null;
                        queueState.active.jobViewSize = 0;
                    }}
                >
                    <X size={20} />
                </Button>
            </div>
        </div>

        <Tabs.Root
            value={queueState.active.jobDetailsTab ??
                (queueState.currentStatus === "failed" ? "error" : "data")}
            class="flex-1 flex flex-col overflow-hidden min-w-0 gap-0"
        >
            <div class="w-full">
                <Scroll.ScrollArea
                    orientation="horizontal"
                    class="w-full min-w-0 border-b"
                >
                    <div class="w-max px-3">
                        <Tabs.List class="gap-1 bg-transparent">
                            <Tabs.Trigger value="data" class="px-4 active:text-primary"
                                >Job Data</Tabs.Trigger
                            >
                            <Tabs.Trigger value="error" class="px-4"
                                >Error Trace</Tabs.Trigger
                            >
                            <Tabs.Trigger value="options" class="px-4"
                                >Options</Tabs.Trigger
                            >
                            <Tabs.Trigger value="logs" class="px-4"
                                >Logs</Tabs.Trigger
                            >
                            <Tabs.Trigger value="timeline" class="px-4"
                                >Timeline</Tabs.Trigger
                            >
                        </Tabs.List>
                    </div>
                    <Scroll.Scrollbar orientation="horizontal" />
                </Scroll.ScrollArea>
            </div>

            <Scroll.ScrollArea orientation="vertical" class="w-full h-full">
                <div class="h-full w-full bg-background shadow-sm min-w-0">
                    <Tabs.Content value="data" class="mt-0 p-2 min-w-0">
                        <JobData
                            jobData={JSON.stringify(
                                queueState.active.selectedJobDetails.data,
                                null,
                                2,
                            )}
                        />
                    </Tabs.Content>

                    <Tabs.Content value="options" class="mt-0 h-full">
                        <DataEditor
                            class="h-[60vh] px-0"
                            value={JSON.stringify(
                                queueState.active.selectedJobDetails.opts,
                                null,
                                2,
                            )}
                            readOnly={true}
                        />
                    </Tabs.Content>

                    <Tabs.Content value="logs" class="mt-0 h-full">
                        {#if selectedJobLog.data && selectedJobLog.data.length > 0}
                            <div
                                class="space-y-5 font-mono text-xs h-[60vh] overflow-y-auto p-5"
                            >
                                {#each selectedJobLog.data as log, i}
                                    <Card
                                        class="flex flex-row gap-3 p-2 border-gray-700 rounded-sm"
                                    >
                                        <span
                                            class="text-muted-foreground w-6 text-right"
                                            >{i + 1}</span
                                        >
                                        <span>{log}</span>
                                    </Card>
                                {/each}
                            </div>
                        {:else}
                            <div
                                class="text-center py-10 text-muted-foreground text-sm italic"
                            >
                                No logs available for this job.
                            </div>
                        {/if}
                    </Tabs.Content>

                    <Tabs.Content value="timeline" class="mt-0 h-full">
                        <JobTimeline
                            job={queueState.active.selectedJobDetails}
                        />
                    </Tabs.Content>

                    <Tabs.Content value="error" class="mt-0 h-full">
                        <JobError
                            stackTrace={queueState.active.selectedJobDetails
                                ?.stacktrace ?? []}
                            failedReason={queueState.active.selectedJobDetails
                                ?.failedReason}
                        />
                    </Tabs.Content>
                </div>
                <Scroll.Scrollbar orientation="vertical" />
            </Scroll.ScrollArea>
        </Tabs.Root>
    </div>
{/if}
{#if selectedJobQuery.isLoading && !queueState.active.selectedJobDetails}
    <div class="h-full w-full flex justify-center items-center">
        <Spinner class="text-primary h-15 w-15" />
    </div>
{/if}
