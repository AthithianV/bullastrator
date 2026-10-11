<script lang="ts">
    import TooltipContent from "shared/components/ui/tooltip/tooltip-content.svelte";
    import TooltipTrigger from "shared/components/ui/tooltip/tooltip-trigger.svelte";
    import Tooltip from "shared/components/ui/tooltip/tooltip.svelte";
    import * as Alert from "ui/alert";

    import { CircleAlert, Hash, Repeat, Timer, Type } from "@lucide/svelte";

    import {
        createVirtualizer,
        type SvelteVirtualizer,
    } from "@tanstack/svelte-virtual";
    import { formatDate, formatDuration, timeAgo } from "helpers/date";
    import { useGetJobsInQueues } from "job/hooks/job.hooks.svelte";
    import type { Job } from "job/interface/job.types";
    import { useQueueState } from "queue/store/queueContext.svelte";
    import CopyButton from "shared/components/actions/CopyButton.svelte";
    import InfiniteScanLoader from "shared/components/loaders/InfiniteScanLoader.svelte";
    import { untrack, type Component } from "svelte";
    import { get, type Readable } from "svelte/store";
    import { cn } from "tailwind-variants";
    import Separator from "$lib/shared/components/ui/separator/separator.svelte";

    let queueState = useQueueState();

    const queueJobsQuery = useGetJobsInQueues();

    let jobData = $state<Job[]>([]);

    $effect(() => {
        if (!queueJobsQuery.isFetching) {
            jobData =
                queueJobsQuery.data?.pages.flatMap((page) => page.jobs) ?? [];
        }
    });

    $effect(() => {
        if (queueState.currentStatus) queueState.active.selectedJob = null;
        queueState.active.selectedJobIds = [];
    });

    interface HeaderItem {
        label?: string;
        icon?: Component;
        class?: string;
    }

    const headers: HeaderItem[] = [
        { class: "w-10", label: "No." },
        { label: "ID", icon: Hash },
        { label: "Job Name", icon: Type },
        { label: "Added at", icon: Timer },
        { label: "Delayed", icon: Timer },
        { label: "Waited For", icon: Timer },
        { label: "Exection Time", icon: Timer },
        { label: "Attempts", icon: Repeat },
    ];

    let focusIndex = $state(-1);
    let anchorIndex = $state(-1);

    function handleKeydown(e: KeyboardEvent) {
        const target = e.target as HTMLElement;
        if (
            target.tagName === "INPUT" ||
            target.tagName === "TEXTAREA" ||
            target.isContentEditable
        )
            return;

        if (jobData.length === 0) {
            return;
        }

        if (e.key === "Enter") {
            queueState.active.jobViewSize = 50;
        }

        if (e.key === "ArrowDown" || e.key === "ArrowUp") {
            e.preventDefault();
            const jobs = jobData;
            const direction = e.key === "ArrowDown" ? 1 : -1;
            let newIndex = focusIndex + direction;

            if (newIndex < 0) newIndex = 0;
            if (newIndex >= jobs.length) newIndex = jobs.length - 1;

            const job = jobs[newIndex];
            document
                .getElementById(`job-row-${job.id}`)
                ?.scrollIntoView({ block: "nearest" });

            if (e.shiftKey) {
                if (anchorIndex === -1) anchorIndex = focusIndex;
                const start = Math.min(anchorIndex, newIndex);
                const end = Math.max(anchorIndex, newIndex);
                queueState.active.selectedJobIds = jobs
                    .slice(start, end + 1)
                    .map((j: Job) => String(j.id));
                queueState.active.selectedJob = job.id;
                focusIndex = newIndex;
            } else {
                queueState.active.selectedJob = job.id;
                queueState.active.selectedJobIds = [String(job.id)];
                focusIndex = newIndex;
                anchorIndex = newIndex;
            }
        }
    }

    function handleJobClick(job: Job, index: number, e: MouseEvent) {
        if (e.shiftKey && anchorIndex !== -1) {
            const jobs = jobData;
            if (!jobs) return;
            const start = Math.min(anchorIndex, index);
            const end = Math.max(anchorIndex, index);
            queueState.active.selectedJobIds = jobs
                .slice(start, end + 1)
                .map((j: Job) => String(j.id));
            queueState.active.selectedJob = job.id;
            focusIndex = index;
        } else if (e.ctrlKey || e.metaKey) {
            const id = String(job.id);
            if (queueState.active.selectedJobIds.includes(id)) {
                queueState.active.selectedJobIds =
                    queueState.active.selectedJobIds.filter(
                        (i: string) => i !== id,
                    );
            } else {
                queueState.active.selectedJobIds = [
                    ...queueState.active.selectedJobIds,
                    id,
                ];
            }
            queueState.active.selectedJob = job.id;
            focusIndex = index;
            anchorIndex = index;
        } else {
            queueState.active.selectedJob = job.id;
            queueState.active.selectedJobIds = [String(job.id)];
            focusIndex = index;
            anchorIndex = index;
        }
    }

    const gridClass =
        "grid grid-cols-[4em_8em_2fr_1fr_1fr_1fr_1fr] gap-2 items-center px-4";

    let parentRef: HTMLDivElement | null = $state(null);
    let rowVirtualizer = $state<Readable<
        SvelteVirtualizer<HTMLDivElement, Element>
    > | null>(null);

    // 1. Create & Resize (Setup)
    $effect(() => {
        if (!parentRef) return;

        if (!rowVirtualizer) {
            rowVirtualizer = createVirtualizer({
                count: queueJobsQuery.hasNextPage
                    ? jobData.length + 1
                    : jobData.length,
                getScrollElement: () => parentRef,
                estimateSize: () => 36,
                overscan: 10,
            });
        }

        const ro = new ResizeObserver(() => {
            const instance = get(rowVirtualizer!);
            instance.measure();
        });

        ro.observe(parentRef);
        return () => ro.disconnect();
    });

    $effect(() => {
        const count = queueJobsQuery.hasNextPage
            ? jobData.length + 1
            : jobData.length;

        if (rowVirtualizer) {
            const instance = get(rowVirtualizer);
            untrack(() => {
                instance.setOptions({
                    count,
                    getScrollElement: () => parentRef!,
                    estimateSize: () => 36,
                    overscan: 10,
                });
                // Crucial: After updating options, we must notify the virtualizer
                // to check if it needs to render more items now.
                instance.measure();
            });
        }
    });

    // 3. Infinite Scroll (Reading state)
    $effect(() => {
        if (
            !$rowVirtualizer ||
            !queueJobsQuery.hasNextPage ||
            queueJobsQuery.isFetchingNextPage
        )
            return;

        // Use $ here because we WANT to react to scroll changes
        const virtualItems = $rowVirtualizer.getVirtualItems();
        if (virtualItems.length === 0) return;

        const lastItem = virtualItems[virtualItems.length - 1];
        if (lastItem.index >= jobData.length - 1) {
            queueJobsQuery.fetchNextPage();
        }
    });

    const getJobMetrics = (job: Job) => {
        const now = Date.now();

        const processDuration =
            job.finishedOn && job.processedOn
                ? job.finishedOn - job.processedOn
                : null;

        const waitedFor = job.processedOn
            ? job.processedOn - job.timestamp
            : now - job.timestamp;

        return {
            processDuration: formatDuration(processDuration),
            waitedFor: formatDuration(waitedFor),
            // Raw values are often useful for charts/sorting in Svelte components
            raw: { processDuration, waitedFor },
        };
    };

    const isDataFetching = $derived(queueJobsQuery.isFetching);
    const isMutationActive = $derived(!!queueState.actionInProgress);
    const loaderColor = $derived(
        isDataFetching && !isMutationActive
            ? "bg-primary"
            : queueState.actionInProgress === "DELETE"
              ? "bg-red-500"
              : queueState.actionInProgress === "PROMOTE"
                ? "bg-violet-500"
                : queueState.actionInProgress === "RETRY"
                  ? "bg-yellow-500"
                  : "bg-primary",
    );
</script>

<svelte:window onkeydown={handleKeydown} />

<div
    class="h-(--queue-table) overflow-auto w-full min-h-0 scrollbar-gutter-stable"
    bind:this={parentRef}
>
    <InfiniteScanLoader
        color={loaderColor}
        loading={queueJobsQuery.isFetching || !!queueState.actionInProgress}
    />
    <div class="w-full text-sm">
        <div class="sticky top-0 z-30">
            <div
                class={cn(
                    "shadow-b-sm h-12 font-medium text-muted-foreground bg-background",
                    "min-w-200",
                    gridClass,
                )}
            >
                {#each headers as header}
                    {#if (header.label === "Delayed" && queueState.currentStatus === "delayed") || header.label === "Waited For" || (header.label === "Exection Time" && queueState.currentStatus === "completed") || header.label === "Attempts" || header.label === "No." || header.label === "ID" || header.label === "Job Name" || header.label === "Added at" || header.label === "Added at"}
                        <div
                            class={cn(
                                "flex items-center gap-2 text-base font-semibold",
                                header.class,
                            )}
                        >
                            {#if header.icon}
                                <header.icon size={12} class={"text-sky-500"} />
                            {/if}
                            {header.label ?? ""}
                        </div>
                    {/if}
                {/each}
            </div>
            <Separator />
        </div>

        <div class="text-foreground/70">
            {#if queueJobsQuery.error}
                <div
                    class="flex flex-col items-center justify-center h-24 border-b"
                >
                    <Alert.Root variant="destructive">
                        <Alert.Title
                            class="flex items-center gap-1 justify-center"
                        >
                            <CircleAlert class="h-4 w-4" />
                            Unable to load jobs.
                        </Alert.Title>
                        <Alert.Description class="text-center justify-center">
                            {queueJobsQuery.error}
                        </Alert.Description>
                    </Alert.Root>
                </div>
            {:else if jobData.length === 0}
                <div class="flex items-center justify-center h-24">
                    <div>No Job Found</div>
                </div>
            {:else if $rowVirtualizer}
                <div
                    class="relative w-full"
                    style:height={`${$rowVirtualizer?.getTotalSize()}px`}
                >
                    {#each $rowVirtualizer?.getVirtualItems() as virtualRow}
                        {@const index = virtualRow.index}
                        {@const job = jobData[index]}

                        {#if job}
                            <div
                                class="absolute top-0 left-0 w-full py-2"
                                style:transform={`translateY(${virtualRow.start}px)`}
                            >
                                <div
                                    id="job-row-{job.id}"
                                    class={cn(
                                        "h-9 border-b-2 cursor-pointer hover:bg-muted/50",
                                        gridClass,
                                        queueState.active.selectedJobIds.includes(
                                            String(job.id),
                                        ) &&
                                            "bg-accent-foreground/10 text-accent-foreground hover:bg-accent-foreground/10",
                                        queueState.active.selectedJob ===
                                            job.id &&
                                            "border-2 border-accent-foreground/50",
                                    )}
                                    onclick={(e) =>
                                        handleJobClick(job, index, e)}
                                    ondblclick={() => {
                                        queueState.active.jobViewSize = 50;
                                    }}
                                    role="button"
                                    tabindex="0"
                                    onkeydown={(e) => {
                                        if (e.key === "Enter")
                                            handleJobClick(
                                                job,
                                                index,
                                                e as any,
                                            );
                                    }}
                                >
                                    <div
                                        class="font-mono text-base p-2 truncate"
                                    >
                                        {queueState.active.currentPage *
                                            queueState.itemsPerPage +
                                            index +
                                            1}
                                    </div>
                                    <div
                                        class="font-mono text-base p-2 truncate"
                                    >
                                        <Tooltip delayDuration={300}>
                                            <TooltipTrigger>
                                                {job.id}
                                            </TooltipTrigger>
                                            <TooltipContent
                                                class="px-3 text-base"
                                            >
                                                {job.id}
                                                <CopyButton value={job.id} />
                                            </TooltipContent>
                                        </Tooltip>
                                    </div>
                                    <div
                                        class="font-medium truncate text-base p-2"
                                    >
                                        <Tooltip delayDuration={300}>
                                            <TooltipTrigger>
                                                {job.name}
                                            </TooltipTrigger>
                                            <TooltipContent
                                                class="px-3 text-base"
                                            >
                                                {job.name}
                                                <CopyButton value={job.name} />
                                            </TooltipContent>
                                        </Tooltip>
                                    </div>
                                    <div
                                        class="font-mono truncate text-base p-2"
                                    >
                                        <Tooltip delayDuration={300}>
                                            <TooltipTrigger>
                                                {timeAgo(job.timestamp)}
                                            </TooltipTrigger>
                                            <TooltipContent
                                                class="p-2 text-base"
                                            >
                                                {formatDate(
                                                    job.timestamp,
                                                    "DD MMM YYYY, HH:mm:ss",
                                                )}
                                            </TooltipContent>
                                        </Tooltip>
                                    </div>
                                    {#if queueState.currentStatus === "delayed"}
                                        <div
                                            class="font-mono truncate text-base p-2"
                                        >
                                            {formatDuration(job.delay)}
                                        </div>
                                    {/if}
                                    <div
                                        class="font-mono truncate text-base p-2"
                                    >
                                        {getJobMetrics(job).waitedFor}
                                    </div>
                                    {#if queueState.currentStatus === "completed"}
                                        <div
                                            class="font-mono truncate text-base p-2"
                                        >
                                            {getJobMetrics(job).processDuration}
                                        </div>
                                    {/if}

                                    <div
                                        class="font-mono truncate text-base p-2"
                                    >
                                        {job.attemptsMade}
                                    </div>
                                </div>
                            </div>
                        {:else}
                            <div></div>
                        {/if}
                    {/each}
                </div>
            {/if}
        </div>
    </div>
</div>
