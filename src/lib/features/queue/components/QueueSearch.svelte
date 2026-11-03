<script lang="ts">
    import * as Command from "ui/command";
    import { globalStore } from "shared/stores/global.svelte";
    import SearchResults from "search/components/SearchResults.svelte";
    import Button from "components/ui/button/button.svelte";
    import { Search } from "@lucide/svelte";
</script>

<div
    class="flex-1 px-3 flex gap-2 text-sm items-center justify-center font-semibold"
>
    <Button
        variant="ghost"
        class="bg-background/40 hover:bg-background! h-8 gap-2 px-3 w-[25%] flex justify-between"
        onclick={() => globalStore.toggleSearchCommand(true)}
    >
        <div
            class="flex justify-start items-center gap-2 text-muted-foreground"
        >
            <Search Searchclass="h-4 w-4" />
            <span>Search</span>
        </div>

        <kbd class="text-xs opacity-60">{globalStore.shortcut("P")}</kbd>
    </Button>
</div>

<Command.Dialog
    bind:open={globalStore.shouldShowSearchCommand}
    showCloseButton={false}
    class="bg-background h-full! p-5 px-3"
>
    <SearchResults closeOnSelect />
</Command.Dialog>
