<script>
    import * as Resizable from "ui/resizable";

    import Card from "ui/card/card.svelte";

    import { globalStore } from "shared/stores/global.svelte";
    import AppSideBar from "sidebar/components/AppSideBar.svelte";
    import TabContainer from "tabs/components/TabContainer.svelte";
    import {
        useGetActiveWorkspace,
        useGetWorkspaces,
    } from "$lib/features/workspace/hooks/workspace.hooks";
    import EmptyWorkspace from "workspace/components/EmptyWorkspace.svelte";

    const workspaces = useGetWorkspaces();
    const activeWorkspace = useGetActiveWorkspace();

    let lastTick = $state(Date.now());

    $effect(() => {
        const interval = setInterval(() => {
            const now = Date.now();
            if (now - lastTick > 10000) {
                console.log(
                    "Bullastrator resumed from sleep. Refreshing connections...",
                );
            }
            lastTick = now;
        }, 2000);

        return () => clearInterval(interval);
    });
</script>

<div class="flex h-(--app-height) min-w-0">
    <div class="p-2 pt-0 min-w-0 w-full">
        <EmptyWorkspace>
            <Resizable.PaneGroup
                direction="horizontal"
                class="gap-0.5 min-w-0 bg-card"
            >
                {#if globalStore.shouldShowSidebar}
                    <Resizable.Pane
                        defaultSize={20}
                        maxSize={50}
                        class="min-w-0 bg-background shadow rounded-lg p-1 border dark:border-black"
                    >
                        <AppSideBar />
                    </Resizable.Pane>
                    <Resizable.Handle class="bg-transparent" />
                {/if}

                <Resizable.Pane class="min-w-0 bg-background shadow rounded-lg">
                    <TabContainer />
                </Resizable.Pane>
            </Resizable.PaneGroup>
        </EmptyWorkspace>
    </div>
</div>
