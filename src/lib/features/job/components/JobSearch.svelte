<script lang="ts">
    import { Search, Square, X } from "@lucide/svelte";
    import {
        useAbortJobsFetch,
        useGetJobsInQueues,
    } from "job/hooks/job.hooks.svelte";
    import type { JobFilterType } from "job/interface/jobPayload.types";
    import { filterJobSchema } from "job/schema/jobFilter.schema";
    import { CircleAlert, Funnel, Hash, Type } from "lucide-svelte";
    import { useQueueState } from "queue/store/queueContext.svelte";
    import DateTime from "shared/components/form/DateTime.svelte";
    import { Badge } from "shared/components/ui/badge";
    import Button from "shared/components/ui/button/button.svelte";
    import { Card } from "shared/components/ui/card";
    import Separator from "shared/components/ui/separator/separator.svelte";
    import { cn } from "shared/utils";
    import { defaults, superForm } from "sveltekit-superforms";
    import { zod4 } from "sveltekit-superforms/adapters";
    import * as Dialog from "ui/dialog";
    import { Input } from "ui/input";
    import * as Popover from "ui/popover";
    import * as Select from "ui/select/";

    const filters: {
        id: string;
        label: string;
        icon: any;
        type: "text" | "number" | "range" | "datetime" | "select";
        placeholder?: string;
    }[] = [
        {
            id: "jobId",
            label: "Job ID",
            icon: Hash,
            type: "text",
            placeholder: "e.g. 10255",
        },
        {
            id: "keyword",
            label: "Keyword Data",
            icon: Type,
            type: "text",
            placeholder: "Search JSON content...",
        },
        {
            id: "failedReason",
            label: "Failed Reason",
            icon: CircleAlert,
            type: "text",
            placeholder: "Error message...",
        },
    ];

    const queueJobsQuery = useGetJobsInQueues();
    const abortRefetch = useAbortJobsFetch();

    let open = $state(false);
    let activeType = $state("JobId");

    let queueState = useQueueState();

    let scannedCount = $derived(
        Math.min(
            queueJobsQuery.data?.pages[queueJobsQuery.data?.pages.length - 1]
                .nextCursor ?? 0,
            queueState.jobCounts[queueState.currentStatus],
        ),
    );

    const totalJobsFetched = $derived(
        queueJobsQuery.data?.pages.reduce(
            (acc, page) => acc + page.jobs.length,
            0,
        ) ?? 0,
    );

    // Initialize SuperForm
    const initialData = defaults(zod4(filterJobSchema));

    const { form, errors, enhance, submit } = superForm(initialData, {
        id: `filter-jobs-${queueState.connectionId}-${queueState.queueName}-${queueState.currentStatus}`,
        validators: zod4(filterJobSchema),
        SPA: true, // Client-side validation only
        dataType: "json",
        resetForm: false,
        onUpdate({ form: f }) {
            if (f.valid) {
                const formData = f.data;

                // 1. String Filters)
                const filters: JobFilterType[] = [];

                if (formData.jobId) {
                    filters.push({ type: "JobId", value: formData.jobId });
                }
                if (formData.keyword) {
                    filters.push({ type: "Keyword", value: formData.keyword });
                }
                if (formData.failedReason) {
                    filters.push({
                        type: "FailedReason",
                        value: formData.failedReason,
                    });
                }

                queueState.searchFilters = filters;
                queueState.active.selectedJobIds = [];
                queueState.active.selectedJob = null;
                queueState.active.jobViewSize = 0;
                queueState.searchCursor = 0;
                queueState.searchLimit = 100;

                open = false;
            } else {
                console.log("Validation Errors:", f.errors);
            }
        },
    });

    function clearFilters(e: MouseEvent) {
        e.stopPropagation(); // Prevent the popover from opening
        queueState.searchFilters = [];
        $form = defaults(zod4(filterJobSchema)).data; // Reset form values
        handleApply(); // Trigger a fresh fetch
    }

    $effect(() => {
        // 1. Get the full SuperValidated object
        const defaultState = defaults(zod4(filterJobSchema));

        // 2. Extract the actual form data object to work with
        const nextFormData = defaultState.data;

        // 3. Map your active filters array back to the form object
        if (queueState.searchFilters && queueState.searchFilters.length > 0) {
            for (const f of queueState.searchFilters) {
                switch (f.type) {
                    case "JobId":
                        nextFormData.jobId = f.value;
                        break;
                    case "Keyword":
                        nextFormData.keyword = f.value;
                        break;
                    case "FailedReason":
                        nextFormData.failedReason = f.value;
                        break;
                }
            }
        }

        // 4. Update the superform store with the raw data object
        $form = nextFormData;
    });

    function handleApply() {
        submit();
    }

    function activate(id: string) {
        activeType = id;
    }

    const timeOptions = [
        { label: "Last 1 hour", value: 60 * 60 * 1000 },
        { label: "Last 5 hours", value: 5 * 60 * 60 * 1000 },
        { label: "Last 1 day", value: 24 * 60 * 60 * 1000 },
        { label: "Last 1 week", value: 7 * 24 * 60 * 60 * 1000 },
        // { label: "Custom", value: "custom" },
    ];

    // State for custom dialogs
    let showCustomDialog = $state(false);
    let customTarget = $state<"createdAfter" | "createdBefore" | null>(null);
    let customDate = $state("");
    let customTime = $state("");

    function handleSelectChange(id: string, value: string) {
        if (value === "custom") {
            customTarget = id as "createdAfter" | "createdBefore";
            showCustomDialog = true;
            return;
        }

        // Calculate timestamp: Now - Offset
        const timestamp = Date.now() - Number(value);
        $form[id as keyof typeof $form] = String(timestamp);
    }

    function applyCustomTime() {
        if (customDate && customTime && customTarget) {
            // Create a Date object from the inputs (Local Time)
            const localDateTime = new Date(`${customDate}T${customTime}`);
            const ts = localDateTime.getTime();

            $form[customTarget as keyof typeof $form] = String(ts);
            showCustomDialog = false;

            // Reset inputs for next use
            customDate = "";
            customTime = "";
        }
    }
</script>

<form use:enhance class="flex gap-2">
    <Card
        class="flex flex-row items-center py-1 px-2 gap-2 shadow pb-0 bg-accent border rounded-[5px]"
    >
        <Button
            variant="ghost"
            class="h-2 p-0 w-2 text-red-500 hover:text-red-600 cursor-pointer z-10 mb-0.5"
            onclick={() => {
                abortRefetch();
            }}
            disabled={!queueJobsQuery.isFetching}
        >
            <Square class="h-1 w-1" fill={"currentColor"} />
        </Button>
        <Separator orientation="vertical" />

        <span class="text-xs font-mono text-blue-400 text-nowrap flex-nowrap"
            >{scannedCount} / {queueState.jobCounts[
                queueState.currentStatus
            ]}</span
        >

        {#if queueState.searchFilters.length > 0}
            <Separator orientation="vertical" />
            <div class="flex flex-nowrap items-center gap-2">
                <span class="text-xs uppercase text-gray-400 font-bold"
                    >Displaying:
                </span>
                <span class="text-xs font-mono text-blue-400">
                    {totalJobsFetched}
                </span>
            </div>
        {/if}

        {#if queueState.active.selectedJobIds.length > 0}
            <Separator orientation="vertical" />
            <span
                class="text-xs font-mono bg-primary rounded px-2 py-1 text-black mb-1 font-bold"
            >
                {queueState.active.selectedJobIds.length}
            </span>
        {/if}
    </Card>
    <div
        class="flex items-center bg-accent shadow px-2 rounded-[5px] max-lg:hidden border"
    >
        <Search class="h-4 w-4" />
        <Input
            bind:value={$form.keyword}
            class="bg-transparent dark:bg-transparent border-none shadow-none w-80 py-0"
            placeholder="Search Keyword"
            type="text"
            onkeydown={(e) => e.key === "Enter" && handleApply()}
        />
    </div>

    <Popover.Root bind:open>
        <Popover.Trigger>
            {#snippet child({ props })}
                <Button
                    variant="ghost"
                    {...props}
                    class="gap-2 border py-1 hover:bg-slate-800!"
                >
                    <Funnel class="h-4 w-4" />
                    {#if queueState.searchFilters.length > 0}
                        <Badge
                            class="group relative h-5 min-w-5 overflow-hidden rounded-full px-0 font-mono tabular-nums transition-all hover:bg-red-500"
                            onclick={clearFilters}
                        >
                            <span
                                class="inline-flex items-center group-hover:hidden"
                            >
                                {queueState.searchFilters.length}
                            </span>

                            <span
                                class="hidden items-center group-hover:inline-flex"
                            >
                                <X class="h-1 w-1" />
                            </span>
                        </Badge>
                    {/if}
                </Button>
            {/snippet}
        </Popover.Trigger>

        <Popover.Content class="w-125 p-0" align="end">
            <div class="px-4 py-3 bg-muted/40 border-b">
                <h4 class="font-medium text-sm">Filter Jobs</h4>
                <p class="text-xs text-muted-foreground">
                    Select a category and enter a value.
                </p>
            </div>

            <div class="max-h-100 overflow-y-auto">
                <table class="w-full text-sm">
                    <thead
                        class="text-xs text-muted-foreground bg-muted/20 text-left"
                    >
                        <tr>
                            <th class="px-4 py-2 font-medium w-45">Category</th>
                            <th class="px-4 py-2 font-medium">Value</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each filters as filter}
                            {@const isActive = activeType === filter.id}
                            {@const errorKey =
                                filter.id as keyof typeof $errors}

                            <tr
                                class={cn(
                                    "border-b last:border-0 transition-colors cursor-pointer",
                                    isActive
                                        ? "bg-blue-50/50 dark:bg-blue-900/10"
                                        : "hover:bg-muted/50",
                                )}
                                onclick={() => activate(filter.id)}
                            >
                                <td class="px-4 py-3 align-middle">
                                    <div class="flex items-center gap-3">
                                        <div
                                            class="flex items-center gap-2 text-foreground"
                                        >
                                            <filter.icon
                                                class="h-3.5 w-3.5 opacity-70"
                                            />
                                            <span>{filter.label}</span>
                                        </div>
                                    </div>
                                </td>

                                <td class="px-4 py-2">
                                    <div class="flex flex-col gap-1">
                                        {#if filter.type === "select"}
                                            <Select.Root
                                                type="single"
                                                onValueChange={(value) =>
                                                    handleSelectChange(
                                                        filter.id,
                                                        value,
                                                    )}
                                            >
                                                <Select.Trigger
                                                    class="w-full flex items-center justify-between"
                                                >
                                                    {#if $form[filter.id as keyof typeof $form]}
                                                        <span
                                                            class="text-blue-400"
                                                        >
                                                            {new Date(
                                                                Number(
                                                                    $form[
                                                                        filter.id as keyof typeof $form
                                                                    ],
                                                                ),
                                                            ).toLocaleString()}
                                                        </span>
                                                        <Badge
                                                            class="group relative h-5 min-w-5 overflow-hidden rounded-full px-0 font-mono tabular-nums transition-all"
                                                            onclick={() => {
                                                                $form[
                                                                    filter.id as keyof typeof $form
                                                                ] = "";
                                                            }}
                                                            variant={"outline"}
                                                        >
                                                            <span
                                                                class="items-center group:hover:text-red-500"
                                                            >
                                                                <X
                                                                    class="h-1 w-1"
                                                                />
                                                            </span>
                                                        </Badge>
                                                    {:else}
                                                        Select range
                                                    {/if}</Select.Trigger
                                                >
                                                <Select.Content>
                                                    {#each timeOptions as opt}
                                                        <Select.Item
                                                            value={String(
                                                                opt.value,
                                                            )}
                                                            >{opt.label}</Select.Item
                                                        >
                                                    {/each}
                                                    <Dialog.Root
                                                        bind:open={
                                                            showCustomDialog
                                                        }
                                                    >
                                                        <Dialog.Content
                                                            class="sm:max-w-100"
                                                        >
                                                            <Dialog.Header>
                                                                <Dialog.Title
                                                                    >Pick Custom
                                                                    Time</Dialog.Title
                                                                >
                                                                <Dialog.Description
                                                                >
                                                                    Select a
                                                                    specific
                                                                    date and
                                                                    time to
                                                                    filter jobs
                                                                    for <strong
                                                                        >{customTarget ===
                                                                        "createdAfter"
                                                                            ? "Created After"
                                                                            : "Created Before"}</strong
                                                                    >.
                                                                </Dialog.Description>
                                                            </Dialog.Header>
                                                            <DateTime />
                                                            <Dialog.Footer>
                                                                <Button
                                                                    variant="outline"
                                                                    onclick={() =>
                                                                        (showCustomDialog = false)}
                                                                    >Cancel</Button
                                                                >
                                                                <Button
                                                                    type="submit"
                                                                    onclick={applyCustomTime}
                                                                    >Save
                                                                    Timestamp</Button
                                                                >
                                                            </Dialog.Footer>
                                                        </Dialog.Content>
                                                    </Dialog.Root>
                                                </Select.Content>
                                            </Select.Root>
                                        {:else}
                                            <Input
                                                type={filter.type}
                                                class={cn(
                                                    "h-8 bg-background",
                                                    $errors[errorKey] &&
                                                        "border-destructive focus-visible:ring-destructive",
                                                )}
                                                placeholder={filter.placeholder}
                                                bind:value={
                                                    $form[
                                                        filter.id as keyof typeof $form
                                                    ]
                                                }
                                                onfocus={() =>
                                                    activate(filter.id)}
                                                onkeydown={(e) =>
                                                    e.key === "Enter" &&
                                                    handleApply()}
                                            />
                                            {#if $errors[errorKey]}
                                                <span
                                                    class="text-[10px] text-destructive font-medium"
                                                >
                                                    {$errors[errorKey]}
                                                </span>
                                            {/if}
                                        {/if}
                                    </div>
                                </td>
                            </tr>
                        {/each}
                    </tbody>
                </table>
            </div>

            <div class="p-3 border-t bg-muted/20 flex justify-end gap-2">
                <Button variant="ghost" size="sm" onclick={() => (open = false)}
                    >Cancel</Button
                >
                <Button size="sm" onclick={handleApply}>Apply Filter</Button>
            </div>
        </Popover.Content>
    </Popover.Root>
</form>
