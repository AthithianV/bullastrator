<script lang="ts">
    import MiniSideBar from "$lib/features/sidebar/components/MiniSideBar.svelte";
    import { Scrollbar } from "../ui/scroll-area";
    import ScrollArea from "../ui/scroll-area/scroll-area.svelte";
    import Separator from "../ui/separator/separator.svelte";

    const { header, content } = $props();
</script>

<div class="w-full h-full">
    <ScrollArea class="w-full h-full flex" orientation="vertical">
        <h1
            class="flex items-center justify-between p-2 sticky top-0 right-0 h-(--tab-list-height) z-100 font-semibold bg-background"
        >
            {@render header()}
        </h1>
        <Separator />

        {@render content()}

        <Scrollbar orientation="vertical" />
    </ScrollArea>
    <div
        class="flex flex-col gap-1 items-center justify-between p-0 sticky bottom-0 right-0 z-50 font-semibold bg-background"
    >
        <Separator />
        <MiniSideBar />
    </div>
</div>
