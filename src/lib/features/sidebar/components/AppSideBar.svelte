<script lang="ts">
    import MetricsList from "metrics/components/MetricsList.svelte";
    import QueueList from "queue/components/QueueList.svelte";
    import SearchView from "search/components/SearchView.svelte";
    import ConnectionList from "connection/components/ConnectionList.svelte";
    import WorkerList from "worker/components/WorkerList.svelte";

    import { sidebarStore } from "../store/sidebarStore.svelte";
</script>

<div class="h-full text-foreground/90">
    {#if sidebarStore.mode === "CONNECTION"}
        <ConnectionList />
    {:else if sidebarStore.mode === "QUEUE"}
        <QueueList />
    {:else if sidebarStore.mode === "METRICS"}
        <MetricsList />
    {:else if sidebarStore.mode === "SEARCH"}
        <SearchView />
    {:else if sidebarStore.mode === "WORKER"}
        <WorkerList />
    {/if}
</div>
