<script lang="ts">
    import JobExportData from "$lib/features/job/components/JobExportData.svelte";
    import {
        ArrowBigUpDash,
        Asterisk,
        Pause,
        Play,
        RefreshCcw,
        RotateCcw,
        Shredder,
        Trash2,
        Zap,
    } from "@lucide/svelte";
    import JobActionButton from "job/components/JobActionButton.svelte";
    import JobForm from "job/components/JobForm.svelte";
    import JobSearch from "job/components/JobSearch.svelte";
    import {
        useDeleteJobs,
        useGetJobsInQueues,
        usePromoteJobs,
        useRefetchJobsinQueue,
        useRetryJob,
    } from "job/hooks/job.hooks.svelte";
    import {
        useGetQueuesDetails,
        usePauseQueue,
    } from "queue/hooks/queue.hooks";
    import { useQueueState } from "queue/store/queueContext.svelte";
    import Spinner from "shared/components/ui/spinner/spinner.svelte";
    import { cn } from "tailwind-variants";
    import * as ButtonGroup from "ui/button-group/index";
    import Button from "ui/button/button.svelte";
    import * as DropdownMenu from "ui/dropdown-menu/index";
    import * as Scroll from "ui/scroll-area/index";
    import TooltipContent from "ui/tooltip/tooltip-content.svelte";
    import TooltipTrigger from "ui/tooltip/tooltip-trigger.svelte";
    import Tooltip from "ui/tooltip/tooltip.svelte";

    let queueState = useQueueState();

    const { mutateAsync: retryJob, isPending: isRetrying } = useRetryJob();
    const { mutateAsync: promoteJobs, isPending: isPromoting } =
        usePromoteJobs();
    const { mutateAsync: deleteJobs, isPending: isDeleting } = useDeleteJobs();
    const refetchJobQuery = useRefetchJobsinQueue();
    const jobsQuery = useGetJobsInQueues();

    const queueDetailsQuery = useGetQueuesDetails();
    const pauseQueueQuery = usePauseQueue();

    let allJobInSelectedState = $derived(
        queueState.jobCounts?.[
            queueState.currentStatus as keyof typeof queueState.jobCounts
        ] ?? 0,
    );

    let hasSelection = $derived(queueState.active.selectedJobIds.length > 0);
    let hasJobsInState = $derived(allJobInSelectedState > 0);
</script>

<Scroll.ScrollArea class="space-y-4 bg-muted/10 p-1 flex justify-between">
    <div
        class="flex justify-between items-center h-(--queue-toolbar-height) min-w-[50vw]"
    >
        <!-- LEFT ACTIONS: ADD, PROMOTE, RETRY, DELETE -->
        <div class="flex gap-1">
            <JobForm />

            {#if queueState.currentStatus === "delayed"}
                <JobActionButton
                    icon={ArrowBigUpDash}
                    isLoading={isPromoting}
                    isDisabled={!hasSelection}
                    isDropdownDisabled={!hasJobsInState}
                    onclick={() => promoteJobs(false)}
                    title="Promote selected jobs"
                >
                    {#snippet menuContent()}
                        <DropdownMenu.Item
                            onclick={() => promoteJobs(false)}
                            disabled={!hasSelection}
                        >
                            <ArrowBigUpDash class="mr-2 h-4 w-4" />
                            <span>Promote Selected</span>
                        </DropdownMenu.Item>
                        <DropdownMenu.Separator />
                        <DropdownMenu.Item
                            onclick={() => promoteJobs(true)}
                            disabled={!hasJobsInState}
                        >
                            <Asterisk class="mr-2 h-4 w-4" />
                            <span>Promote All</span>
                        </DropdownMenu.Item>
                    {/snippet}
                </JobActionButton>
            {/if}

            {#if queueState.currentStatus === "failed" || queueState.currentStatus === "completed"}
                <JobActionButton
                    icon={RotateCcw}
                    isLoading={isRetrying}
                    isDisabled={!hasSelection}
                    isDropdownDisabled={!hasJobsInState}
                    onclick={() =>
                        retryJob({
                            strategy: "ToBack",
                            isAll: false,
                            retryJobStatus: queueState.currentStatus,
                        })}
                    title="Retry selected jobs"
                >
                    {#snippet menuContent()}
                        <DropdownMenu.Item
                            onclick={() =>
                                retryJob({
                                    strategy: "ToBack",
                                    isAll: false,
                                    retryJobStatus: queueState.currentStatus,
                                })}
                            class="flex justify-between items-center"
                            disabled={!hasSelection}
                        >
                            <div class="flex items-center gap-2">
                                <RotateCcw class="h-4 w-4" />
                                <span>Retry Selected</span>
                            </div>
                            <span class="text-xs opacity-70 ml-2"
                                >RPUSH ({queueState.active.selectedJobIds
                                    .length})</span
                            >
                        </DropdownMenu.Item>

                        <DropdownMenu.Separator />

                        <DropdownMenu.Item
                            onclick={() =>
                                retryJob({
                                    strategy: "ToFront",
                                    isAll: false,
                                    retryJobStatus: queueState.currentStatus,
                                })}
                            disabled={!hasSelection}
                            class="text-orange-500 hover:text-orange-600 hover:bg-orange-50 flex justify-between items-center"
                        >
                            <div class="flex items-center gap-2">
                                <Zap class="h-4 w-4" fill={"currentColor"} />
                                <span class="font-medium"
                                    >Quick Retry Selected</span
                                >
                            </div>
                            <span class="text-xs opacity-70 ml-2"
                                >LPUSH ({queueState.active.selectedJobIds
                                    .length})</span
                            >
                        </DropdownMenu.Item>

                        <DropdownMenu.Separator />

                        <DropdownMenu.Item
                            onclick={() =>
                                retryJob({
                                    strategy: "ToBack",
                                    isAll: true,
                                    retryJobStatus: queueState.currentStatus,
                                })}
                            disabled={!hasJobsInState}
                            class="flex justify-between items-center"
                        >
                            <div class="flex items-center gap-2">
                                <Asterisk
                                    class="mr-2 h-4 w-4 fill-orange-500/20"
                                />
                                <span class="font-medium">Retry All</span>
                            </div>
                            <span class="text-xs opacity-70 ml-2"
                                >LPUSH ({allJobInSelectedState})</span
                            >
                        </DropdownMenu.Item>

                        <DropdownMenu.Separator />

                        <DropdownMenu.Item
                            onclick={() =>
                                retryJob({
                                    strategy: "ToFront",
                                    isAll: true,
                                    retryJobStatus: queueState.currentStatus,
                                })}
                            class="flex justify-between items-center text-orange-500 hover:text-orange-600 hover:bg-slate-800"
                            disabled={!hasJobsInState}
                        >
                            <div class="flex items-center gap-2">
                                <Asterisk
                                    class="mr-2 h-4 w-4 fill-orange-500/20"
                                />
                                <span class="font-medium">Quick Retry All</span>
                            </div>
                            <span class="text-xs opacity-70 ml-2"
                                >RPUSH ({allJobInSelectedState})</span
                            >
                        </DropdownMenu.Item>
                    {/snippet}
                </JobActionButton>
            {/if}

            {#if queueState.currentStatus !== "active"}
                <JobActionButton
                    icon={Trash2}
                    isLoading={isDeleting}
                    isDisabled={!hasSelection}
                    isDropdownDisabled={!hasJobsInState}
                    onclick={() => deleteJobs(false)}
                    title="Delete selected jobs"
                    class="text-red-500 hover:text-red-600"
                >
                    {#snippet menuContent()}
                        <DropdownMenu.Item
                            onclick={() => deleteJobs(false)}
                            disabled={!hasSelection}
                        >
                            <Trash2 class="h-4 w-4 text-red-500" />
                            <span
                                >Delete Selected ({queueState.active
                                    .selectedJobIds.length})</span
                            >
                        </DropdownMenu.Item>
                        <DropdownMenu.Separator />
                        <DropdownMenu.Item onclick={() => deleteJobs(true)}>
                            <Shredder class="text-red-500 h-4 w-4" />
                            <span>Delete All ({allJobInSelectedState})</span>
                        </DropdownMenu.Item>
                    {/snippet}
                </JobActionButton>
            {/if}
        </div>

        <!-- RIGHT ACTIONS: SEARCH & FILTER -->
        <div class="flex gap-1 ps-2">
            <JobExportData />
            <JobSearch />
            <ButtonGroup.Root class="gap-1">
                <Tooltip>
                    <TooltipTrigger>
                        <Button
                            variant="ghost"
                            class={cn(
                                "py-2 px-3! cursor-pointer rounded-sm",
                                "bg-accent hover:text-primary shadow border hover:bg-slate-800!",
                            )}
                            disabled={jobsQuery.isFetching}
                            onclick={refetchJobQuery}
                        >
                            <RefreshCcw class={cn("h-4 w-4")} />
                        </Button>
                    </TooltipTrigger>
                    <TooltipContent>Refresh</TooltipContent>
                </Tooltip>
                <Tooltip>
                    <TooltipTrigger>
                        <Button
                            variant="ghost"
                            class={cn(
                                " py-2 px-3! cursor-pointer rounded-sm",
                                "bg-accent hover:text-primary shadow border hover:bg-slate-800!",
                            )}
                            size="default"
                            disabled={queueDetailsQuery.isFetching ||
                                !queueDetailsQuery.data ||
                                pauseQueueQuery.isPending}
                            onclick={() =>
                                queueDetailsQuery.data &&
                                pauseQueueQuery.mutateAsync(
                                    !queueDetailsQuery.data.isPaused,
                                )}
                        >
                            {#if pauseQueueQuery.isPending || queueDetailsQuery.isFetching}
                                <Spinner />
                            {:else if queueDetailsQuery.data?.isPaused}
                                <Play fill="currentColor" />
                            {:else}
                                <Pause fill="currentColor" />
                            {/if}
                        </Button>
                    </TooltipTrigger>
                    <TooltipContent>Pause/Resume Queue Globally</TooltipContent>
                </Tooltip>
            </ButtonGroup.Root>
        </div>
    </div>
    <Scroll.Scrollbar orientation="horizontal" />
</Scroll.ScrollArea>
