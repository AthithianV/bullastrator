<script lang="ts">
    import SidebarWrapper from "components/wrapper/SidebarWrapper.svelte";

    import * as Command from "ui/command";
    import SearchResults from "./SearchResults.svelte";
</script>

<SidebarWrapper>
    {#snippet header()}
        <h1>Search</h1>
        <div></div>
    {/snippet}

    {#snippet content()}
        <Command.Root
            class="h-full! max-h-full! border-none bg-transparent z-0 py-2"
            shouldFilter={true}
        >
            <SearchResults />
        </Command.Root>
    {/snippet}
</SidebarWrapper>
