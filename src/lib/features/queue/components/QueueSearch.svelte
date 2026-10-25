<script lang="ts">
    import * as Command from "ui/command";
    import { globalStore } from "shared/stores/global.svelte";
    import SearchResults from "search/components/SearchResults.svelte";
</script>

<Command.Dialog
    bind:open={globalStore.shouldShowSearchCommand}
    showCloseButton={false}
    class="bg-background h-full! p-5 px-3"
>
    <SearchResults closeOnSelect />
</Command.Dialog>
