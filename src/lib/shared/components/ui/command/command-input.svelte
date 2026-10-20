<script lang="ts">
    import { Command as CommandPrimitive } from "bits-ui";
    import { SearchIcon } from "lucide-svelte";
    import { cn } from "shared/utils.js";

    let {
        ref = $bindable(null),
        class: className,
        value = $bindable(""),
        ...restProps
    }: CommandPrimitive.InputProps = $props();
</script>

<div
    class="flex items-center gap-2 bg-card py-1 px-4 rounded-sm mb-2"
    data-slot="command-input-wrapper"
>
    <SearchIcon class="size-4 shrink-0 opacity-50" />
    <CommandPrimitive.Input
        data-slot="command-input"
        class={cn(
            "placeholder:text-muted-foreground flex h-8 w-full rounded-md bg-transparent text-sm outline-hidden disabled:cursor-not-allowed disabled:opacity-50 mb-0! px-0! py-0!",
            className,
        )}
        bind:ref
        {...restProps}
        bind:value
    />
</div>
