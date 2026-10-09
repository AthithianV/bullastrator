<script lang="ts">
    import QueueSearch from "queue/components/QueueSearch.svelte";
    import WorkspaceSelector from "workspace/components/WorkspaceSelector.svelte";
    import Brand from "./Brand.svelte";
    import ThemeToggler from "$lib/shared/components/actions/ThemeToggler.svelte";
</script>

<div class="w-full h-(--titlebar-height) flex justify-between items-center">
    <div class="flex items-center">
        <Brand />
        <WorkspaceSelector />
    </div>
    <QueueSearch />
    <div
        role="button"
        tabindex="0"
        class="flex-1 px-3 flex gap-2 text-sm items-center justify-center font-semibold"
    ></div>
    <div class="flex items-center h-full gap-1 px-2">
        <ThemeToggler />
    </div>
</div>
