<script lang="ts">
    import type { QueueAction } from "$lib/features/queue/interface/queue.types";
        import {
        FileDown,
        FileUp,
        ListPlus,
        Plus,
        RotateCcw,
        Trash,
        Funnel,
        Search,
        RefreshCcw,
        ArrowBigUpDash,
        Shredder,
    } from "@lucide/svelte";

    const {
        action,
        size = 12,
        className = "",
    } = $props<{ action: QueueAction; size?: number; className?: string }>();

    const icons: Record<string, typeof FileDown> = {
       add: Plus,
        "add-multiple": ListPlus,
        "import-jobs": FileUp,
        "export-jobs": FileDown,
        "retry-failed": RotateCcw,
        "promote-delayed": ArrowBigUpDash,
        delete: Trash,
        filter: Funnel,
        refresh: RefreshCcw,
        clear: Shredder,
        search: Search,
    };

    const Icon = $derived(icons[action] ?? FileDown);
</script>

<Icon {size} class={className} />
