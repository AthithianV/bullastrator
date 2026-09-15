<script lang="ts">
    import { Folder, LoaderCircle, Workflow, X } from "@lucide/svelte";
    import { cn } from "shared/utils";
    import * as Avatar from "ui/avatar";
    import Button from "ui/button/button.svelte";
    import * as Scroll from "ui/scroll-area/index";
    import Separator from "ui/separator/separator.svelte";
    import * as Tabs from "ui/tabs/index";

    import { dndzone } from "svelte-dnd-action";

    import EmptyTabPage from "./EmptyTabPage.svelte";
    import TabError from "./TabError.svelte";

    import {
        useDeleteTab,
        useGetActiveTabId,
        useGetTabs,
        useReorderTabs,
        useSetActiveTab,
    } from "../hooks/tab.hooks";

    import QueueTab from "queue/components/QueueTab.svelte";
    import { tick, untrack } from "svelte";
    import FolderTab from "../../folder/components/FolderTab.svelte";
    import { tabStore } from "../store/tabStore.svelte";

    const tabQuery = useGetTabs();
    const activeTabQuery = useGetActiveTabId();
    const { mutateAsync: closeTab } = useDeleteTab();
    const { mutateAsync: setActiveTabMutateAsync } = useSetActiveTab();
    const { mutateAsync: reorderTabs } = useReorderTabs();

    $effect(() => {
        tabStore.setActiveTab(activeTabQuery.data ?? null);
    });

    $effect(() => {
        // We want to trigger this when activeTab changes OR when a new tab is added
        const activeId = tabStore.activeTab;
        const tabCount = tabQuery.data?.length;

        if (activeId && tabCount) {
            untrack(() => {
                tick().then(() => {
                    const el = document.querySelector(
                        `[data-tab-id="${activeId}"]`,
                    );
                    if (el) {
                        el.scrollIntoView({
                            behavior: "smooth",
                            block: "nearest",
                        });
                    }
                });
            });
        }
    });

    const handleCloseTab = async (e: MouseEvent, tabId: string) => {
        e.stopPropagation();
        e.preventDefault();

        if (tabStore.activeTab === tabId && tabQuery.data) {
            const newActiveTabId = tabStore.moveToNextTabOnClose(
                tabId,
                tabQuery.data,
            );
            await setActiveTabMutateAsync(newActiveTabId ?? null);
        }

        closeTab(tabId);
    };

    const flipDurationMs = 300;

    function handleDndConsider(e: CustomEvent) {
        tabQuery.data = e.detail.items;
    }

    function handleDndFinalize(e: CustomEvent) {
        tabQuery.data = e.detail.items;

        reorderTabs(e.detail.items.map((i: any) => i.id));
    }

    const avatarStyle = (color?: string) =>
        `background-color: ${color || "var(--primary)"}`;
</script>

<div class="w-full h-(--app-height) overflow-x-auto flex flex-col min-w-0">
    {#if tabQuery.isLoading}
        <div
            class="h-full flex-1 flex flex-col items-center justify-center text-muted-foreground gap-2"
        >
            <LoaderCircle class="size-8 animate-spin" />
            <p class="text-sm">Restoring workspace...</p>
        </div>
    {:else if tabQuery.isError}
        <TabError
            refetch={() => tabQuery.refetch()}
            message={tabQuery.error.message}
        />
    {:else if !tabQuery.data || tabQuery.data.length === 0}
        <EmptyTabPage />
    {:else}
        <Tabs.Root
            value={tabStore.activeTab ?? ""}
            onValueChange={(val) => {
                tabStore.setActiveTab(val);
                setActiveTabMutateAsync(val);
            }}
            class="h-full flex flex-col gap-0 min-w-0 p-1"
        >
            <Scroll.ScrollArea
                class="w-full whitespace-nowrap"
                orientation="horizontal"
                type="always"
            >
                <Tabs.List
                    class={"bg-transparent h-(--tab-list-height) justify-start gap-1 min-w-0 flex-none"}
                >
                    <div
                        class="flex flex-row items-center gap-1 p-1 pr-12 overflow-x-auto no-scrollbar scroll-smooth w-full"
                        style="scrollbar-width: none;"
                        use:dndzone={{
                            items: tabQuery.data ?? [],
                            flipDurationMs,
                            dropTargetStyle: { outline: "none" },
                        }}
                        onconsider={handleDndConsider}
                        onfinalize={handleDndFinalize}
                    >
                        {#each tabQuery.data ?? [] as tab (tab.id + JSON.stringify(tab.params))}
                            <Tabs.Trigger
                                value={tab.id}
                                class={cn(
                                    "shrink-0",
                                    "data-[state=active]:bg-accent",
                                    "dark:data-[state=active]:text-accent-foreground",
                                    "dark:data-[state=active]:bg-accent-foreground/10",
                                    "h-8 rounded-sm px-3 py-1",
                                    "flex items-center cursor-pointer",
                                )}
                                data-tab-id={tab.id}
                                onchangecapture={() =>
                                    tabStore.setActiveTab(tab.id)}
                            >
                                <Avatar.Root
                                    class="flex items-center justify-center"
                                >
                                    <Avatar.Fallback
                                        class={cn(
                                            "flex items-center justify-center",
                                            "h-3/4 w-3/4",
                                            "rounded-full p-2",
                                            "text-[10px] text-white font-semibold italics",
                                        )}
                                        style={avatarStyle(
                                            tab.connection?.color,
                                        )}
                                    >
                                        {tab.connection?.label}
                                    </Avatar.Fallback>
                                </Avatar.Root>
                                {#if tab.params.type === "QUEUE"}
                                    <Workflow />
                                {/if}
                                {#if tab.params.type === "FOLDER"}
                                    <Folder fill="currentColor" />
                                {/if}

                                {tab.title}

                                <Button
                                    variant="ghost"
                                    size={"sm"}
                                    class="w-2 h-2 px-0 relative top-px text-0.5 cursor-pointer p-2 rounded hover:bg-slate-200 dark:hover:bg-slate-800 hover:text-red-400 text-muted-foreground/50"
                                    onclick={(e) => {
                                      e.stopPropagation();
                                      handleCloseTab(e, tab.id);
                                    }}
                                >
                                    <X size={8} class="w-2 h-2" />
                                </Button>
                            </Tabs.Trigger>
                        {/each}
                    </div>
                </Tabs.List>

                <Scroll.Scrollbar orientation="horizontal" />
            </Scroll.ScrollArea>
            <Separator />
            <div class="h-(--tab-content-height) overflow-auto min-w-0">
                {#each tabQuery.data as tab}
                    <Tabs.Content value={tab.id} class="h-full m-0 p-0 min-w-0">
                        {#key tab.id}
                            {#if tab.params.type === "QUEUE"}
                                <QueueTab
                                    tabParams={tab.params}
                                    tabId={tab.id}
                                />
                            {:else if tab.params.type === "FOLDER"}
                                <FolderTab
                                    tabParams={tab.params}
                                    tabId={tab.id}
                                    connection={tab.connection}
                                />
                            {/if}
                        {/key}
                    </Tabs.Content>
                {/each}
            </div>
        </Tabs.Root>
    {/if}
</div>
