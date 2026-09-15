<script lang="ts">
    import MetricsList from "metrics/components/MetricsList.svelte";
    import QueueList from "queue/components/QueueList.svelte";
    import SearchView from "search/components/SearchView.svelte";
    import ConnectionList from "connection/components/ConnectionList.svelte";
    import WorkerList from "worker/components/WorkerList.svelte";

    import { sidebarStore } from "../store/sidebarStore.svelte";

    import { useCheckHealthForAllConnections } from "connection/hooks/connection.hooks";
    import { connectionStore } from "connection/store/connection.store.svelte";

    const connectionHealthCheckQuery = useCheckHealthForAllConnections();

    $effect(() => {
        for (const connection of connectionHealthCheckQuery.data ?? []) {
            connectionStore.connectionsHealth[connection.id] =
                connection.isActive;
        }
    });
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
