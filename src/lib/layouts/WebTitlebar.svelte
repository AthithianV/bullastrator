<script lang="ts">
    import QueueSearch from "queue/components/QueueSearch.svelte";
    import WorkspaceSelector from "workspace/components/WorkspaceSelector.svelte";
    import Brand from "./Brand.svelte";
    import ThemeToggler from "$lib/shared/components/actions/ThemeToggler.svelte";
    import UserManagementPanel from "$lib/features/user/components/UserManagementPanel.svelte";
</script>

<div class="w-full h-(--titlebar-height) flex justify-between items-center">
    <div class="flex items-center">
        <Brand />
        <WorkspaceSelector />
    </div>
    <QueueSearch />
    <div class="flex items-center h-full gap-1 px-2">
        <ThemeToggler />
        <UserManagementPanel />
    </div>
</div>
