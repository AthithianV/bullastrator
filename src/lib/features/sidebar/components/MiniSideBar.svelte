<script lang="ts">
    import { cn } from "shared/utils";
    import { Button } from "ui/button";
    import { SIDEBAR_MODES } from "../constants/sidebar.contants";
    import { sidebarStore } from "../store/sidebarStore.svelte";
</script>

<div class="flex flex-row items-center justify-between gap-2 h-full py-1 px-2">
    <div class="flex flex-row items-center gap-2">
        {#each SIDEBAR_MODES as { id, icon: Icon, handleClick, shouldShow }}
            {#if shouldShow}
                <Button
                    class={cn(
                        "rounded-md",
                        sidebarStore.mode === id
                            ? "text-primary hover:text-primary dark:bg-slate-800 dark:hover:bg-slate-800! bg-slate-200 hover:bg-slate-200!"
                            : "text-muted-foreground dark:focus:bg-transparent",
                        "cursor-pointer transition-all rounded-sm h-7 w-7 flex-center active:scale-90",
                    )}
                    onclick={() => handleClick()}
                    variant="ghost"
                >
                    <svelte:component
                        this={Icon}
                        style="width:1rem;height:1rem;"
                        stroke-width={2.5}
                    />
                </Button>
            {/if}
        {/each}
    </div>
</div>
