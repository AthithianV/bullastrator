<script lang="ts">
    import MiniSideBar from "$lib/features/sidebar/components/MiniSideBar.svelte";
    import ScrollArea from "components/ui/scroll-area/scroll-area.svelte";
    import Separator from "components/ui/separator/separator.svelte";
    import { Scrollbar } from "components/ui/scroll-area";
    import { SIDEBAR_MODES } from "sidebar/constants/sidebar.contants";
    import { sidebarStore } from "sidebar/store/sidebarStore.svelte";

    const activeMode = $derived(
        SIDEBAR_MODES.find(
            (mode) => mode.id === sidebarStore.mode && mode.shouldShow,
        ),
    );
</script>

<div class="w-full h-full">
    {#if activeMode?.component}
        {@const ActiveComponent = activeMode.component}
        <ActiveComponent />
    {/if}
    <div
        class="flex flex-col gap-1 items-center justify-between p-0 sticky bottom-0 right-0 z-50 font-semibold bg-background"
    >
        <Separator />
        <MiniSideBar />
    </div>
</div>
