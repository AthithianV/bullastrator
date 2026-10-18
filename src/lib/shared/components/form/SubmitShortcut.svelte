<script lang="ts">
    import { globalStore } from "shared/stores/global.svelte";
</script>

<kbd
    class="ml-2 rounded border border-border/50 px-1.5 py-0.5 font-mono text-[10px]"
>
    {globalStore.shortcut("Enter")}
</kbd>
