<script lang="ts">
    import { CircleX } from "@lucide/svelte";

    const { message, refetch } = $props();
</script>

<div class="flex-1 flex flex-col items-center justify-center text-destructive gap-4">
    <CircleX class="size-10" />
    <div class="text-center">
        <h3 class="font-bold text-lg">Failed to load tabs</h3>
        <p class="text-sm text-muted-foreground">{message}</p>
    </div>
    <button
        class="px-4 py-2 bg-secondary rounded-md text-sm hover:bg-secondary/80 transition"
        onclick={() => refetch()}
    >
        Try Again
    </button>
</div>
