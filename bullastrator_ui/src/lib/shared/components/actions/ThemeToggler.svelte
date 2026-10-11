<script lang="ts">
    import { Button } from "../ui/button";
    import { cn } from "shared/utils";

    import { Moon, Sun } from "@lucide/svelte";
    import { theme } from "shared/stores/theme.svelte";

    function toggleTheme() {
        theme.update((t) => (t === "dark" ? "light" : "dark"));
    }
</script>

<Button
    class={cn(
        "rounded-md",
        "text-muted-foreground dark:hover:bg-transparent hover:bg-transparent dark:focus:bg-transparent bg-transparent",
        "cursor-pointer transition-all rounded-sm h-8 w-8 flex-center active:scale-90",
    )}
    title="Toggle Theme"
    onclick={toggleTheme}
>
    {#if $theme === "dark"}
        <Sun style="width:1.1rem;height:1.1rem;" stroke-width={2.5} />
    {:else}
        <Moon style="width:1.1rem;height:1.1rem;" stroke-width={2.5} />
    {/if}
</Button>
