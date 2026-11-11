<script lang="ts">
    import { goto } from "$app/navigation";
    import { onMount } from "svelte";
    import { useCheckSession } from "../hooks/user.hooks";
    import { runtime } from "shared/runtime";

    let { children } = $props();
    let checking = $state(true);
    const checkSession = useCheckSession();

    onMount(async () => {
        if (!runtime.isWeb) {
            checking = false;
            return;
        }

        if (!(await checkSession())) {
            await goto(
                `/login?redirect=${encodeURIComponent(location.pathname)}`,
            );
            return;
        }
        checking = false;
    });
</script>

{#if !checking}
    {@render children()}
{:else}
    <div
        class="flex min-h-screen items-center justify-center text-sm text-muted-foreground"
    >
        Checking your session...
    </div>
{/if}
