<script lang="ts">
    import { Ellipsis, Pencil, Plus, RefreshCcw, Trash2 } from "@lucide/svelte";
    import ConnectionForm from "./ConnectionForm.svelte";
    import DialogWrapper from "shared/components/wrapper/DialogWrapper.svelte";
    import * as DropdownMenu from "ui/dropdown-menu";
    import { useSyncAllQueues } from "queue/hooks/queue.hooks";
    import { useDeleteConnection } from "connection/hooks/connection.hooks";
    import type { ConnectionWithQueue } from "queue/interface/queue.types";
    import { explorerStore } from "$lib/features/explorer/store/explorer.svelte";

    const { mutateAsync: syncQuery } = useSyncAllQueues();
    const { mutateAsync: deleteConnection } = useDeleteConnection();

    const { connection }: { connection: ConnectionWithQueue } = $props();
    const dropdownId = $derived(`connection-${connection.id}`);
    let updateDialogOpen = $state(false);

    function handleDropdownChange(open: boolean) {
        if (open) {
            explorerStore.openDropdownId = dropdownId;
        } else if (explorerStore.openDropdownId === dropdownId) {
            explorerStore.openDropdownId = null;
        }
    }

    async function refetch(e: Event, connectionId: string) {
        e.stopPropagation();
        await syncQuery(connectionId);
    }

    function stop(event: Event) {
        event.stopPropagation();
    }
</script>

<div class="flex gap-2 ms-2">
    <DropdownMenu.Root
        open={explorerStore.openDropdownId === dropdownId}
        onOpenChange={handleDropdownChange}
    >
        <DropdownMenu.Trigger
            class="h-5 w-5 p-1 flex items-center justify-center rounded-sm hover:bg-slate-200 dark:hover:bg-slate-700"
            onpointerdown={(e) => e.stopPropagation()}
            onmousedown={(e) => e.stopPropagation()}
            onclick={stop}
        >
            <Ellipsis class="h-4 w-4" />
        </DropdownMenu.Trigger>
        <DropdownMenu.Content align="end">
            <DropdownMenu.Item
                onclick={(e) =>
                    (explorerStore.createFolderForConnectionId = connection.id)}
            >
                <Plus class="h-4 w-4" />
                Add New Folder
            </DropdownMenu.Item>
            <DropdownMenu.Item
                onclick={(e) => {
                    e.stopPropagation();
                    explorerStore.openDropdownId = null;
                    updateDialogOpen = true;
                }}
            >
                <Pencil class="h-4 w-4" />
                Update Connection
            </DropdownMenu.Item>
            <DropdownMenu.Item onclick={(e) => refetch(e, connection.id)}>
                <RefreshCcw class="h-4 w-4" />
                Refresh Connection
            </DropdownMenu.Item>
            <DropdownMenu.Item
                variant="destructive"
                onclick={() => deleteConnection(connection.id)}
            >
                <Trash2 class="h-4 w-4" />
                Delete Connection
            </DropdownMenu.Item>
        </DropdownMenu.Content>
    </DropdownMenu.Root>

    <DialogWrapper
        bind:open={updateDialogOpen}
        showTrigger={false}
        title="Update Connection"
        description="Update the Redis connection details."
    >
        <ConnectionForm
            connectionId={connection.id}
            onComplete={() => (updateDialogOpen = false)}
        />
    </DialogWrapper>
</div>
