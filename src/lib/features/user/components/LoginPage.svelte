<script lang="ts">
    import { goto } from "$app/navigation";
    import { onMount } from "svelte";
    import { Button } from "ui/button";
    import { Input } from "ui/input";
    import { Label } from "ui/label";
    import AuthCard from "./AuthCard.svelte";
    import { useCheckSession, useLogin } from "../hooks/user.hooks";
    import { Eye } from "@lucide/svelte";
    import SubmitShortcut from "shared/components/form/SubmitShortcut.svelte";
    import { submitOnModEnter } from "shared/helpers/formShortcuts";

    let email = $state("");
    let password = $state("");
    let showPassword = $state(false);
    let submitting = $state(false);
    let checkingSession = $state(true);
    const login = useLogin();
    const checkSession = useCheckSession();

    onMount(async () => {
        if (await checkSession()) {
            await goto("/");
            return;
        }
        checkingSession = false;
    });

    async function submit(event: SubmitEvent) {
        event.preventDefault();
        submitting = true;
        try {
            await login.mutateAsync({ email, password });
            await goto("/");
        } finally {
            submitting = false;
        }
    }
</script>

{#if !checkingSession}
<AuthCard
    title="Welcome back"
    description="Log in to continue to Bullastrator."
>
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <form class="space-y-6 pt-10" onsubmit={submit} onkeydown={submitOnModEnter}>
        <div class="space-y-2">
            <Label class="text-muted-foreground" for="email">Email</Label>
            <Input
                class="py-6 px-2 focus:border-primary focus:text-primary font-semibold text-lg!"
                id="email"
                type="email"
                bind:value={email}
                required
                autocomplete="email"
            />
        </div>
        <div class="space-y-2">
            <Label class="text-muted-foreground" for="password">Password</Label>
            <div class="relative">
                <Input
                    id="password"
                    type={showPassword ? "text" : "password"}
                    class="relative py-6 px-2 focus:border-primary focus:text-primary font-semibold text-lg!"
                    bind:value={password}
                    required
                    autocomplete="current-password"
                />
                <Button
                    variant="ghost"
                    class="hover:bg-transparent absolute right-0 top-1 text-muted-foreground"
                    onclick={() => (showPassword = !showPassword)}
                    ><Eye /></Button
                >
            </div>
        </div>
        <div class="flex justify-center items-center pt-5">
            <Button class="font-semibold" type="submit" disabled={submitting}>
                {submitting ? "Logging in..." : "Log in"}
                <SubmitShortcut />
            </Button>
        </div>

        <p class="text-center text-sm text-muted-foreground">
            Need an account?
            <a
                class="text-primary underline-offset-4 hover:underline"
                href="/register">Register</a
            >
        </p>
    </form>
</AuthCard>
{:else}
    <div
        class="flex min-h-screen items-center justify-center text-sm text-muted-foreground"
    >
        Checking your session...
    </div>
{/if}
