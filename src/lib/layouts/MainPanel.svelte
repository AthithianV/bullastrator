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
    import EmptyWorkspace from "$lib/features/workspace/components/EmptyWorkspace.svelte";

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
    <div class="w-full p-2 pt-0 min-w-0">
        {#if workspaces.isLoading || activeWorkspace.isLoading}
            <div
                class="h-full w-full bg-background rounded-lg flex justify-center items-center"
            >
                <h1>Loading...</h1>
            </div>
        {:else if workspaces.isError}
            <div
                class="h-full w-full bg-background rounded-lg flex justify-center items-center"
            >
                <h1>Error Occurred</h1>
                <p>{workspaces.error}</p>
            </div>
        {:else if workspaces.data && workspaces.data.length > 0 && activeWorkspace.data && !activeWorkspace.isError}
            <Card
                class="h-full rounded-none shadow-none py-0 min-w-0 border-none"
            >
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

                    <Resizable.Pane
                        class="min-w-0 bg-background shadow rounded-lg"
                    >
                        <TabContainer />
                    </Resizable.Pane>
                </Resizable.PaneGroup>
            </Card>
        {:else}
            <div
                class="h-full w-full bg-background rounded-lg flex justify-center items-center"
            >
                <EmptyWorkspace workspaces={workspaces.data ?? []} />
            </div>
        {/if}
    </div>
</div>
