<script lang="ts">
    import QueueSearch from "queue/components/QueueSearch.svelte";
    import WorkspaceSelector from "workspace/components/WorkspaceSelector.svelte";
    import Brand from "./Brand.svelte";
    import ThemeToggler from "$lib/shared/components/actions/ThemeToggler.svelte";
    import { globalStore } from "shared/stores/global.svelte";
    import { Search } from "@lucide/svelte";
    import { Button } from "shared/components/ui/button";
</script>

<div class="w-full h-(--titlebar-height) flex justify-between items-center">
    <div class="flex items-center">
        <Brand />
        <WorkspaceSelector />
    </div>
    <QueueSearch />
    <div
        class="flex-1 px-3 flex gap-2 text-sm items-center justify-center font-semibold"
    >
        <Button
            variant="ghost"
            class="h-8 gap-2 px-3"
            onclick={() => globalStore.toggleSearchCommand(true)}
        >
            <Search class="h-4 w-4" />
            <span>Search</span>
            <kbd class="text-xs opacity-60">{globalStore.shortcut("P")}</kbd>
        </Button>
    </div>
    <div class="flex items-center h-full gap-1 px-2">
        <ThemeToggler />
    </div>
</div>
