<script lang="ts">
    import { explorerStore } from "$lib/features/explorer/store/explorer.svelte";
    import { Folder } from "@lucide/svelte";
    import Input from "components/ui/input/input.svelte";
    import {
        useCreateFolder,
        useUpdateFolder,
    } from "folder/hooks/folder.hooks.svelte";
    import { cn } from "tailwind-variants";

    const {
        connectionId,
        folderId,
        folderName,
    }: {
        connectionId: string;
        folderId?: string;
        folderName?: string;
    } = $props();

    let newFolderName = $state("");
    let folderError = $state<string | null>(null);

    const { mutateAsync: createFolder } = useCreateFolder();
    const { mutateAsync: updateFolder } = useUpdateFolder();

    let editorElement = $state<HTMLDivElement | null>(null);
    let inputElement = $state<HTMLInputElement | null>(null);

    $effect(() => {
        if (folderName) newFolderName = folderName;
        if (editorElement && inputElement) {
            inputElement.focus();
        }
    });

    function closeEditor() {
        explorerStore.createFolderForConnectionId = null;
        explorerStore.renameFolderId = null;
        folderError = null;
    }

    $effect(() => {
        if (!editorElement) return;

        const handleOutsidePointerDown = (event: PointerEvent) => {
            if (!editorElement?.contains(event.target as Node)) {
                closeEditor();
            }
        };

        document.addEventListener("pointerdown", handleOutsidePointerDown);
        return () =>
            document.removeEventListener(
                "pointerdown",
                handleOutsidePointerDown,
            );
    });

    async function createNewFolder(e: Event, connectionId: string) {
        e.stopPropagation();

        if (e instanceof KeyboardEvent && e.key === "Enter") {
            if (newFolderName === "" || !newFolderName) {
                folderError = "Folder Name Required";
                return;
            }

            if (folderId) {
                await updateFolder({
                    id: folderId,
                    title: newFolderName,
                });
            } else {
                await createFolder({
                    title: newFolderName || "Untitled",
                    connectionId,
                });
            }
            explorerStore.createFolderForConnectionId = null;
            newFolderName = "";
            explorerStore.renameFolderId = null;
        }
    }
</script>

{#if explorerStore.createFolderForConnectionId === connectionId || folderId}
    <div
        bind:this={editorElement}
        class={cn(
            "flex-1 flex justify-between items-center cursor-pointer ps-16 border  rounded-sm",
            folderError ? "border-red-500" : "border-primary",
        )}
    >
        <div class="flex gap-2 items-center">
            <Folder class="w-4 h-4" />
            <Input
                bind:ref={inputElement}
                bind:value={newFolderName}
                class="w-full border-none outline-0 px-0 bg-transparent! py-1!"
                onkeydown={(e) => createNewFolder(e, connectionId)}
            />
        </div>
    </div>
{/if}
