<script lang="ts">
    import {
        Toaster as Sonner,
        type ToasterProps as SonnerProps,
    } from "svelte-sonner";
    import { mode } from "mode-watcher";

    let { ...restProps }: SonnerProps = $props();
</script>

<Sonner
    theme={mode.current}
    class="toaster group"
    style="--normal-bg: var(--color-card); --normal-text: var(--color-card-foreground); --normal-border: var(--color-border);"
    {...restProps}
/>

<style>
    :global([data-sonner-toast][data-type="success"] [data-icon]) {
        color: #22c55e;
    }

    :global([data-sonner-toast][data-type="error"] [data-icon]) {
        color: #ef4444;
    }

    :global([data-sonner-toast][data-type="warning"] [data-icon]) {
        color: #eab308;
    }

    :global([data-sonner-toast][data-type="info"] [data-icon]) {
        color: #3b82f6;
    }
</style>
