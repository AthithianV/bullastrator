<script lang="ts">
    import { Plus } from "@lucide/svelte";
    import DialogWrapper from "components/wrapper/DialogWrapper.svelte";
    import { cn } from "shared/utils";
    import * as Select from "ui/select";
    import Skeleton from "ui/skeleton/skeleton.svelte";
    import {
        useGetActiveWorkspace,
        useGetWorkspaces,
        useSetActiveWorkspace,
    } from "workspace/hooks/workspace.hooks";
    import WorkspaceForm from "./WorkspaceForm.svelte";

    const workspaces = useGetWorkspaces();
    const defaultWorkspace = useGetActiveWorkspace();
    const setActiveWorkspaceQuery = useSetActiveWorkspace();

    let selectedId = $state<string>("");

    $effect(() => {
        if (defaultWorkspace.data && !selectedId) {
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

    function handleSelect(value: string | undefined) {
        if (value) {
            selectedId = value;
            setActiveWorkspaceQuery.mutateAsync(value);
        }
    }
</script>

<div class="flex flex-col gap-2 w-full max-w-fit py-0 mx-2">
    {#if isLoading}
        <div
            class="flex h-10 items-center px-3 py-2 border rounded-md animate-pulse bg-muted/50"
        >
            <Skeleton class="h-10 w-10" />
        </div>
    {:else}
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
                            >{selectedWorkspace.name}</span
                        >
                    {:else}
                        <span class="text-muted-foreground"
                            >Select Workspace</span
                        >
                    {/if}
                </div>
            </Select.Trigger>

            <Select.Content class="max-h-[50vh] overflow-auto">
                {#each allWorkspaces as ws}
                    <Select.Item
                        value={ws.id.toString()}
                        label={ws.name}
                        class="cursor-pointer py-2 px-4"
                    >
                        <div class="flex items-center gap-2 w-full py-1 px-4">
                            <div
                                class="h-3 w-3 rounded-full shrink-0"
                                style:background-color={ws.color ?? "#94a3b8"}
                            ></div>
                            <span class="flex-1 truncate">{ws.name}</span>
                            {#if ws.isDefault}
                                <span
                                    class="text-[10px] uppercase tracking-wider font-bold text-primary opacity-70"
                                >
                                    Def
                                </span>
                            {/if}
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
                                "w-full bg-transparent h-5!",
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
</div>
