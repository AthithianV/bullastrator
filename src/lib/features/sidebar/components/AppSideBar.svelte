<script lang="ts">
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
</div>
