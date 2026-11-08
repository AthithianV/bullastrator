<script lang="ts">
    import { goto } from "$app/navigation";
    import { LogOut } from "@lucide/svelte";
    import QueueSearch from "queue/components/QueueSearch.svelte";
    import WorkspaceSelector from "workspace/components/WorkspaceSelector.svelte";
    import Brand from "./Brand.svelte";
    import ThemeToggler from "$lib/shared/components/actions/ThemeToggler.svelte";
    import UserManagementPanel from "$lib/features/user/components/UserManagementPanel.svelte";
    import Button from "shared/components/ui/button/button.svelte";
    import { useLogout } from "$lib/features/user/hooks/user.hooks";

    const { mutateAsync: logout } = useLogout();
    let isLoggingOut = $state(false);

    async function handleLogout() {
        if (isLoggingOut) return;

        isLoggingOut = true;
        try {
            await logout("");
            await goto("/login");
        } finally {
            isLoggingOut = false;
        }
    }
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
        <Button
            variant="ghost"
            size="icon"
            class="text-muted-foreground hover:bg-red-500/10 hover:text-red-500"
            aria-label="Log out"
            title="Log out"
            disabled={isLoggingOut}
            onclick={handleLogout}
        >
            <LogOut class="size-4" />
        </Button>
    </div>
</div>
