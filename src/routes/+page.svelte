<script lang="ts">
    import { onMount } from "svelte";
    import * as Resizable from "ui/resizable";
    import { PUBLIC_IS_WEB } from "$env/static/public";

    import TitleBar from "titlebar/components/TitleBar.svelte";
    import Card from "ui/card/card.svelte";

    import UpdateToaster from "$lib/shared/components/updater/UpdateToaster.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import Zoom from "settings/components/Zoom.svelte";
    import { globalStore } from "shared/stores/global.svelte";
    import AppSideBar from "sidebar/components/AppSideBar.svelte";
    import TabContainer from "tabs/components/TabContainer.svelte";
    import WebTitlebar from "$lib/features/titlebar/components/WebTitlebar.svelte";

    onMount(async () => {
        !PUBLIC_IS_WEB && (await invoke("show_main_window"));
    });

    onMount(() => {
        window.addEventListener("keydown", globalStore.handleKeyDown);
        return () =>
            window.removeEventListener("keydown", globalStore.handleKeyDown);
    });

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

<main class="w-full fixed top-0 bg-card select-none min-w-0">
    <Zoom />
    {#if !PUBLIC_IS_WEB}
        <TitleBar />
        <UpdateToaster />
    {:else}
        <WebTitlebar />
    {/if}
    <div class="flex h-(--app-height) min-w-0">
        <div class="w-full p-2 pt-0 min-w-0">
            <Card
                class="h-full rounded-none shadow-none py-0 min-w-0 border-none bg-background"
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
        </div>
    </div>
</main>
