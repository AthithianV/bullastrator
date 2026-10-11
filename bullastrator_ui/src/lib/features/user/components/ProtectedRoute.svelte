<script lang="ts">
    import { goto } from "$app/navigation";
    import { onMount } from "svelte";
    import { PUBLIC_IS_WEB } from "$env/static/public";
    import { useCheckSession } from "../hooks/user.hooks";

    let { children } = $props<{ children: import("svelte").Snippet }>();
    let checking = $state(true);
    const checkSession = useCheckSession();

    onMount(async () => {
        if (![true, "true", "1"].includes(PUBLIC_IS_WEB)) {
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
