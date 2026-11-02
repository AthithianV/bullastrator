<script lang="ts">
    import { Pencil, Plus, Trash2 } from "@lucide/svelte";
    import Button from "ui/button/button.svelte";
    import DialogWrapper from "components/wrapper/DialogWrapper.svelte";
    import { confirmAction } from "shared/helpers/confirmAction";
    import { cn } from "shared/utils";
    import * as Select from "ui/select";
    import Skeleton from "ui/skeleton/skeleton.svelte";
    import {
        useDeleteWorkspace,
        useGetActiveWorkspace,
        useGetWorkspaces,
        useSetActiveWorkspace,
    } from "workspace/hooks/workspace.hooks";
    import type { ReadWorkspaceModel } from "../interface/workspace.types";
    import WorkspaceForm from "./WorkspaceForm.svelte";

    const workspaces = useGetWorkspaces();
    const defaultWorkspace = useGetActiveWorkspace();
    const setActiveWorkspaceQuery = useSetActiveWorkspace();
    const { mutateAsync: deleteWorkspace } = useDeleteWorkspace();

    let selectedId = $state<string>("");
    let editingWorkspace = $state<ReadWorkspaceModel | null>(null);
    let editDialogOpen = $state(false);
    let editVersion = $state(0);

    $effect(() => {
        if (
            defaultWorkspace.data &&
            (!selectedId || !allWorkspaces.some((ws) => ws.id === selectedId))
        ) {
            selectedId = defaultWorkspace.data.id.toString();
        }
    });

    const allWorkspaces = $derived(workspaces.data ?? []);
    const isLoading = $derived(
        workspaces.isLoading || defaultWorkspace.isLoading,
    );

    const selectedWorkspace = $derived(
        allWorkspaces.find((w) => w.id.toString() === selectedId),
    );
    const orderedWorkspaces = $derived([
        ...allWorkspaces.filter((ws) => ws.id.toString() === selectedId),
        ...allWorkspaces.filter((ws) => ws.id.toString() !== selectedId),
    ]);

    function handleSelect(value: string | undefined) {
        if (value) {
            selectedId = value;
            setActiveWorkspaceQuery.mutateAsync(value);
        }
    }

    function startEditing(workspace: ReadWorkspaceModel, event: Event) {
        event.preventDefault();
        event.stopPropagation();
        editingWorkspace = workspace;
        editVersion += 1;
        editDialogOpen = true;
    }

    async function removeWorkspace(
        workspace: ReadWorkspaceModel,
        event: Event,
    ) {
        event.preventDefault();
        event.stopPropagation();

        const confirmed = await confirmAction(
            `Are you sure you want to delete the workspace "${workspace.name}"? This action cannot be undone.`,
            { title: "Delete workspace", kind: "warning" },
        );

        if (!confirmed) return;

        await deleteWorkspace(workspace.id);
    }
</script>

<div class="flex flex-col gap-2 w-full max-w-fit py-0 mx-2">
    {#if !isLoading}
        <Select.Root
            type="single"
            bind:value={selectedId}
            onValueChange={handleSelect}
        >
            <Select.Trigger
                class="w-full justify-between hover:bg-accent transition-colors border-none py-1 px-4 h-8!"
            >
                <div class="flex items-center gap-2 truncate">
                    {#if selectedWorkspace}
                        <div
                            class="h-3 w-3 rounded-full shrink-0"
                            style:background-color={selectedWorkspace.color ??
                                "#94a3b8"}
                        ></div>
                        <span class="truncate font-medium"
                            >{selectedWorkspace.name.slice(0, 20)}</span
                        >
                    {:else}
                        <span class="text-muted-foreground"
                            >Select Workspace</span
                        >
                    {/if}
                </div>
            </Select.Trigger>

            <Select.Content class="max-h-[50vh] min-w-[15vw] overflow-auto">
                {#each orderedWorkspaces as ws, index}
                    {#if index === 1 && selectedWorkspace}
                        <Select.Separator class="my-1" />
                    {/if}
                    <Select.Item
                        value={ws.id.toString()}
                        label={ws.name}
                        class="group cursor-pointer flex items-center justify-between gap-10"
                    >
                        <div
                            class="flex items-center justify-between gap-2 w-full"
                        >
                            <div
                                class="h-3 w-3 rounded-full shrink-0"
                                style:background-color={ws.color ?? "#94a3b8"}
                            ></div>
                            <span class="flex-1 truncate"
                                >{ws.name.slice(0, 20)}</span
                            >
                            <div
                                class="ms-auto flex items-center gap-1 opacity-0 transition-opacity group-hover:opacity-100 group-focus-within:opacity-100"
                            >
                                <Button
                                    type="button"
                                    size="icon-sm"
                                    variant="ghost"
                                    class="h-7 w-7 hover:text-blue-400"
                                    aria-label={`Edit ${ws.name}`}
                                    title={`Edit ${ws.name}`}
                                    onpointerdown={(event) =>
                                        startEditing(ws, event)}
                                    onclick={(event) => event.stopPropagation()}
                                >
                                    <Pencil class="h-3.5 w-3.5 text-current" />
                                </Button>
                                <Button
                                    type="button"
                                    size="icon-sm"
                                    variant="ghost"
                                    class="h-7 w-7 text-destructive hover:text-red-400"
                                    aria-label={`Delete ${ws.name}`}
                                    title={`Delete ${ws.name}`}
                                    onpointerdown={(event) =>
                                        removeWorkspace(ws, event)}
                                    onclick={(event) => event.stopPropagation()}
                                >
                                    <Trash2 class="h-3.5 w-3.5 text-current" />
                                </Button>
                            </div>
                        </div>
                    </Select.Item>
                {/each}
                <DialogWrapper
                    title="Create Workspace"
                    description="Organize your connections into a new workspace."
                    triggerClass="w-full"
                >
                    {#snippet trigger()}
                        <div
                            class={cn(
                                "text-center",
                                "py-2 flex gap-2 items-center justify-center rounded-sm text-sm",
                                "w-full bg-transparent h-5! hover:font-bold",
                            )}
                        >
                            <Plus class="h-4 w-4" />
                            <span>Add Workspace</span>
                        </div>
                    {/snippet}

                    {#snippet children(onComplete)}
                        <WorkspaceForm {onComplete} />
                    {/snippet}
                </DialogWrapper>
            </Select.Content>
        </Select.Root>
    {/if}

    {#if editingWorkspace}
        {@const workspace = editingWorkspace}
        <DialogWrapper
            bind:open={editDialogOpen}
            showTrigger={false}
            title="Edit Workspace"
            description="Update your workspace details."
        >
            {#snippet children(onComplete)}
                {#key `${workspace.id}-${editVersion}`}
                    <WorkspaceForm
                        data={workspace}
                        onComplete={() => {
                            onComplete();
                            editingWorkspace = null;
                        }}
                    />
                {/key}
            {/snippet}
        </DialogWrapper>
    {/if}
</div>
