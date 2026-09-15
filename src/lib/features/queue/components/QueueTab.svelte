<script lang="ts">
    import * as Resizable from "ui/resizable";
    import * as Scroll from "ui/scroll-area/index";

    import JobList from "job/components/JobList.svelte";
    import JobView from "job/components/JobView.svelte";

    import type { QueueTabParams } from "queue/interface/queue.types";
    import {
        setQueueState,
        useQueueState,
    } from "queue/store/queueContext.svelte";
    import { untrack } from "svelte";
    import { useGetTabById, useUpdateTab } from "tabs/hooks/tab.hooks";
    import { tabStore } from "tabs/store/tabStore.svelte";
    import Separator from "ui/separator/separator.svelte";
    import { cn } from "utils";
    import QueueStatusbar from "./QueueStatusbar.svelte";
    import QueueToolbar from "./QueueToolbar.svelte";

    const { tabParams, tabId }: { tabParams: QueueTabParams; tabId: string } =
        $props<{
            tabParams: QueueTabParams;
            tabId: string;
        }>();

    setQueueState(tabId, tabParams);

    let queueState = useQueueState();
    const { mutateAsync: updateTab } = useUpdateTab();
    const tabQuery = useGetTabById(tabId);

    $effect(() => {
        const externalParams = tabQuery.data?.params;

        if (externalParams) {
            if (externalParams.type === "QUEUE" && externalParams.state) {
                queueState.currentStatus = externalParams.state;
            }
        }
    });

    $effect(() => {
        const currentTabId = tabId;

        const handler = setTimeout(() => {
            const dataToSave = untrack(() => queueState.toJSON());

            if (tabStore.activeTab === currentTabId) {
                updateTab({
                    id: tabId,
                    data: { params: dataToSave },
                });
            }
        }, 500);

        return () => clearTimeout(handler);
    });
    let isHandlerActive = $state(false);
</script>

<div
    class="h-(--tab-content-height) flex flex-col overflow-x-auto no-scrollbar min-w-0"
>
    <QueueStatusbar />
    <Separator />

    <QueueToolbar />
    <Separator />

    <Resizable.PaneGroup
        direction="horizontal"
        onLayoutChange={(sizes) => {
            queueState.active.jobViewSize = sizes[1];
        }}
        class="min-w-0"
    >
        <Resizable.Pane class="relative min-w-0">
            <Scroll.ScrollArea>
                <JobList />
                <Scroll.Scrollbar orientation="horizontal" />
            </Scroll.ScrollArea>
        </Resizable.Pane>



        {#if queueState.active.jobViewSize > 0}
            <Resizable.Handle
                class={cn(
                    "rounded-full",
                    isHandlerActive && "border-primary border-2",
                )}
                onmousedown={() => (isHandlerActive = true)}
                onmouseup={() => (isHandlerActive = false)}
            />
        <Resizable.Pane
            defaultSize={queueState.active.jobViewSize}
            class="min-w-0"
        >
            <JobView />
        </Resizable.Pane>
        {/if}
    </Resizable.PaneGroup>
</div>
