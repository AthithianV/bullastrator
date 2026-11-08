<script lang="ts">
    import { onMount } from "svelte";

    import UpdateToaster from "$lib/shared/components/updater/UpdateToaster.svelte";
    import Zoom from "settings/components/Zoom.svelte";
    import { globalStore } from "shared/stores/global.svelte";
    import ProtectedRoute from "$lib/features/user/components/ProtectedRoute.svelte";
    import TitleBar from "$lib/layouts/TitleBar.svelte";
    import WebTitlebar from "$lib/layouts/WebTitlebar.svelte";
    import MainPanel from "$lib/layouts/MainPanel.svelte";
    import { invokeWrapper } from "$lib/shared/helpers/invokeWrapper";
    import { useGetActiveWorkspace } from "workspace/hooks/workspace.hooks";
    import EmptyWorkspace from "workspace/components/EmptyWorkspace.svelte";

    const activeWorkspace = useGetActiveWorkspace();

    onMount(async () => {
        await invokeWrapper("show_main_window");
    });

    onMount(() => {
        window.addEventListener("keydown", globalStore.handleKeyDown);
        return () =>
            window.removeEventListener("keydown", globalStore.handleKeyDown);
    });
</script>

<ProtectedRoute>
    <Zoom />
    <div class="w-screen h-screen p-2 min-w-0">
        {#if activeWorkspace.isLoading}
            <div
                class="h-full w-full bg-background rounded-lg flex justify-center items-center"
            >
                <h1>Loading...</h1>
            </div>
        {:else if activeWorkspace.isLoading}
            <div
                class="h-full w-full bg-background rounded-lg flex justify-center items-center"
            >
                <h1>Error Occurred</h1>
                <p>{activeWorkspace.error}</p>
            </div>
        {:else if activeWorkspace.data && !activeWorkspace.isError}
            {#if !globalStore.isWeb}
                <TitleBar />
                <UpdateToaster />
            {:else}
                <WebTitlebar />
            {/if}
            <MainPanel />
        {:else}
            <div
                class="h-full w-full bg-background rounded-lg flex justify-center items-center"
            >
                <EmptyWorkspace />
            </div>
        {/if}
    </div>
</ProtectedRoute>
