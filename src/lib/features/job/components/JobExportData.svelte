<script lang="ts">
    import DataEditor from "components/editor/Editor.svelte";
    import * as Dialog from "ui/dialog";

    import { Share } from "@lucide/svelte";
    import InfiniteScanLoader from "shared/components/loaders/InfiniteScanLoader.svelte";
    import { useGetJobData } from "../hooks/job.hooks.svelte";
    import { cn } from "utils";
    import { useQueueState } from "$lib/features/queue/store/queueContext.svelte";
    import Badge from "$lib/shared/components/ui/badge/badge.svelte";
    import TooltipContent from "shared/components/ui/tooltip/tooltip-content.svelte";
    import TooltipTrigger from "shared/components/ui/tooltip/tooltip-trigger.svelte";
    import Tooltip from "shared/components/ui/tooltip/tooltip.svelte";
    import { globalStore } from "shared/stores/global.svelte";

    const jobDataQuery = useGetJobData();
    const queueState = useQueueState();

    async function handleOpenChange(open: boolean) {
        if (open) {
            await jobDataQuery.refetch();
        }
    }
</script>

<Dialog.Root onOpenChange={handleOpenChange}>
    <Tooltip>
        <TooltipTrigger>
            {#snippet child({ props })}
                <Dialog.Trigger
                    {...props}
                    data-queue-shortcut="export"
                    class={cn(
                        "py-2 px-3! cursor-pointer rounded-sm border",
                        "shadow",
                        " hover:bg-card hover:text-primary",
                    )}><Share class="h-4 w-4" /></Dialog.Trigger
                >
            {/snippet}
        </TooltipTrigger>
        <TooltipContent>
            Export
            <kbd
                class="ml-2 rounded border border-border/50 px-1.5 py-0.5 font-mono text-[10px]"
            >
                {globalStore.shortcut("E")}
            </kbd>
        </TooltipContent>
    </Tooltip>
    <Dialog.Content
        class="bg-card! w-[50vw] max-w-[50vw] sm:max-w-[50vw] border-none p-3 gap-1"
    >
        <h3 class="p-3 font-semibold text-lg">
            Job Data - ({queueState.queueName})
            <Badge>{queueState.active.status}</Badge>
        </h3>
        <div class="max-h-[90vh] overflow-y-auto shadow relative">
            <InfiniteScanLoader loading={jobDataQuery.isLoading} />

            <DataEditor
                class="h-[60vh] px-0 rounded-sm"
                value={JSON.stringify(jobDataQuery.data ?? {}, null, 2)}
                readOnly={true}
            />
        </div>
    </Dialog.Content>
</Dialog.Root>
