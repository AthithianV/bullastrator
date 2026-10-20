<script lang="ts">
    import * as Accordion from "ui/accordion";

    import {
        ChevronRight,
        Ellipsis,
        Folder,
        Pencil,
        Trash2,
    } from "@lucide/svelte";
    import * as DropdownMenu from "ui/dropdown-menu";
    import { cn } from "shared/utils";

    import { useDeleteFolder } from "../../folder/hooks/folder.hooks.svelte";
    import { queueStore } from "queue/store/queueStore.svelte";
    import { useCreateTab } from "tabs/hooks/tab.hooks";
    import type { ReadFolderWithQueuesModel } from "folder/interface/folder.types";
    import { explorerStore } from "$lib/features/explorer/store/explorer.svelte";

    const {
        folder,
    }: {
        folder: ReadFolderWithQueuesModel;
    } = $props();

    const { mutateAsync: createTab } = useCreateTab();
    const { mutateAsync: deleteFolder } = useDeleteFolder();
    const isAllQueuesFolder = $derived(folder.id === folder.connectionId);
    const dropdownId = $derived(`folder-${folder.id}`);

    function handleDropdownChange(open: boolean) {
        if (open) {
            explorerStore.openDropdownId = dropdownId;
        } else if (explorerStore.openDropdownId === dropdownId) {
            explorerStore.openDropdownId = null;
        }
    }

    async function openFolder(
        e: MouseEvent | Event,
        folderId: string,
        title: string,
    ) {
        e.stopPropagation();
        await createTab({
            id: folderId,
            title: title,
            connectionId: folder.connectionId,
            params: {
                connectionId: folder.connectionId,
                folderName: title,
                type: "FOLDER",
                folderId: folderId,
            },
        });
    }
</script>

<div
    role="button"
    tabindex="0"
    class={cn(
        "ps-6 flex items-center gap-1 w-full group transition-colors",
        queueStore.makeActive(`${folder.id}-${folder.title}`),
    )}
    onmousedown={(e) => {
        e.preventDefault();
        queueStore.setActiveNode(`${folder.id}-${folder.title}`);
        openFolder(e, folder.id, folder.title);
    }}
>
    <Accordion.Trigger
        class={cn(
            "flex items-center justify-center py-1 p-1 rounded hover:bg-slate-200 dark:hover:bg-slate-700",
            "[&[data-state=open]>svg]:rotate-90 transition-transform",
        )}
    >
        <ChevronRight class="w-4 h-4 block" />
    </Accordion.Trigger>

    <div
        class="flex-1 flex justify-between items-center cursor-pointer py-1.5 px-2"
    >
        <div class="flex gap-2 items-center">
            <Folder class="w-4 h-4" />
            <span class="font-semibold truncate">{folder.title}</span>
        </div>
        {#if !isAllQueuesFolder}
            <DropdownMenu.Root
                open={explorerStore.openDropdownId === dropdownId}
                onOpenChange={handleDropdownChange}
            >
                <DropdownMenu.Trigger
                    class="flex items-center justify-center rounded-sm hover:bg-slate-200 dark:hover:bg-slate-700 h-5 w-5 p-1"
                    onpointerdown={(e) => e.stopPropagation()}
                    onmousedown={(e) => e.stopPropagation()}
                    onclick={(e) => e.stopPropagation()}
                >
                    <Ellipsis class="h-4 w-4" />
                </DropdownMenu.Trigger>
                <DropdownMenu.Content align="end">
                    <DropdownMenu.Item
                        onclick={(e) => {
                            explorerStore.renameFolderId = folder.id;
                            explorerStore.createFolderForConnectionId = null;
                        }}
                    >
                        <Pencil />
                        Rename
                    </DropdownMenu.Item>
                    <DropdownMenu.Item
                        variant="destructive"
                        onclick={(e) => {
                            deleteFolder(folder.id);
                        }}
                    >
                        <Trash2 />
                        Delete Folder
                    </DropdownMenu.Item>
                </DropdownMenu.Content>
            </DropdownMenu.Root>
        {/if}
    </div>
</div>
