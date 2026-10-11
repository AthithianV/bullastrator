<script lang="ts">
    import { onMount } from "svelte";
    import { PUBLIC_IS_WEB } from "$env/static/public";

    import UpdateToaster from "$lib/shared/components/updater/UpdateToaster.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import Zoom from "settings/components/Zoom.svelte";
    import { globalStore } from "shared/stores/global.svelte";
    import ProtectedRoute from "$lib/features/user/components/ProtectedRoute.svelte";
    import TitleBar from "$lib/layouts/TitleBar.svelte";
    import WebTitlebar from "$lib/layouts/WebTitlebar.svelte";
    import MainPanel from "$lib/layouts/MainPanel.svelte";

    onMount(async () => {
        !PUBLIC_IS_WEB && (await invoke("show_main_window"));
    });

    onMount(() => {
        window.addEventListener("keydown", globalStore.handleKeyDown);
        return () =>
            window.removeEventListener("keydown", globalStore.handleKeyDown);
    });
</script>

<ProtectedRoute>
    <Zoom />
    {#if !PUBLIC_IS_WEB}
        <TitleBar />
        <UpdateToaster />
    {:else}
        <WebTitlebar />
    {/if}
    <MainPanel />
</ProtectedRoute>
