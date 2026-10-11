<script lang="ts">
    import { Inbox, Plus } from "@lucide/svelte";
    import DialogWrapper from "shared/components/wrapper/DialogWrapper.svelte";
    import ConnectionForm from "./ConnectionForm.svelte";
</script>

<div class="flex h-full w-full flex-col items-center p-8 text-center">
    <div class="relative mb-4 flex items-center justify-center">
        <div
            class="absolute h-16 w-16 animate-pulse rounded-full bg-primary/10 blur-2xl"
        ></div>

        <Inbox />
    </div>

    <h3 class="text-sm font-semibold text-foreground">No Connections Found</h3>
    <p class="mt-1 text-xs text-muted-foreground max-w-45 mb-5">
        Connect to a Redis instance to start managing your queues.
    </p>

    <DialogWrapper
        title="Add new Connection"
        description="Complete to add new Connection"
    >
        {#snippet trigger()}
            <div
                class="text-primary rounded-sm text-sm font-semibold flex gap-2 items-center justify-center px-2"
            >
                <Plus class="h-4 w-4" />

                <span> Add Connection </span>
            </div>
        {/snippet}

        {#snippet children(onComplete)}
            <ConnectionForm {onComplete} />
        {/snippet}
    </DialogWrapper>
</div>
