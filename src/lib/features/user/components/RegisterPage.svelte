<script lang="ts">
    import { goto } from "$app/navigation";
    import { Button } from "ui/button";
    import { Input } from "ui/input";
    import { Label } from "ui/label";
    import AuthCard from "./AuthCard.svelte";
    import { useRegister } from "../hooks/user.hooks";
    import { Eye } from "@lucide/svelte";

    let name = $state("");
    let email = $state("");
    let password = $state("");
    let showPassword = $state(false);

    let submitting = $state(false);
    const register = useRegister();

    async function submit(event: SubmitEvent) {
        event.preventDefault();
        submitting = true;
        try {
            await register.mutateAsync({ name, email, password });
            await goto("/");
        } finally {
            submitting = false;
        }
    }
</script>

<AuthCard
    title="Create an account"
    description="Start managing your BullMQ queues."
>
    <form class="space-y-5 py-5" onsubmit={submit}>
        <div class="space-y-2">
            <Label class="text-muted-foreground" for="name">Name</Label>
            <Input
                class="py-5 px-2 focus:border-primary focus:text-primary font-semibold"
                id="name"
                bind:value={name}
                required
                autocomplete="name"
            />
        </div>
        <div class="space-y-2">
            <Label class="text-muted-foreground" for="email">Email</Label>
            <Input
                class="py-5 px-2 focus:border-primary focus:text-primary font-semibold"
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
                    class="relative py-5 px-2 focus:border-primary focus:text-primary"
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
        <div class="flex justify-center items-center">
            <Button type="submit" disabled={submitting}>
                {submitting ? "Creating account..." : "Register"}
            </Button>
        </div>

        <p class="text-center text-sm text-muted-foreground">
            Already have an account?
            <a
                class="text-primary underline-offset-4 hover:underline font-semibold"
                href="/login">Log in</a
            >
        </p>
    </form>
</AuthCard>
