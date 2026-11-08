<script lang="ts">
    import { Boxes, LoaderCircle } from "@lucide/svelte";
    import * as Card from "ui/card";
    import { Button } from "ui/button";
    import { useSetActiveWorkspace } from "workspace/hooks/workspace.hooks";
    import type { ReadWorkspaceModel } from "../interface/workspace.types";
    import WorkspaceForm from "./WorkspaceForm.svelte";

    let {
        workspaces = [],
    }: {
        workspaces?: ReadWorkspaceModel[];
    } = $props();

    const { mutateAsync: setActiveWorkspace, isPending } =
        useSetActiveWorkspace();

    const hasWorkspaces = $derived(workspaces.length > 0);

    async function selectWorkspace(workspaceId: string) {
        await setActiveWorkspace(workspaceId);
    }
</script>

<div class="min-h-full w-full overflow-auto bg-background rounded-lg">
    <div
        class="mx-auto flex min-h-full w-full max-w-3xl flex-col justify-center px-6 py-12 sm:px-10"
    >
        <div class="mb-8 text-center">
            <div
                class="mx-auto mb-5 flex size-12 items-center justify-center rounded-2xl bg-primary/10 text-primary ring-1 ring-primary/20"
            >
                <Boxes class="size-6" strokeWidth={1.8} />
            </div>
            <p
                class="mb-2 text-xs font-semibold uppercase tracking-[0.2em] text-primary"
            >
                {hasWorkspaces ? "Welcome back" : "Get started"}
            </p>
            <h1 class="text-3xl font-semibold tracking-tight sm:text-4xl">
                {hasWorkspaces
                    ? "Choose a workspace"
                    : "Create your first workspace"}
            </h1>
            <p
                class="mx-auto mt-3 max-w-xl text-sm leading-6 text-muted-foreground sm:text-base"
            >
                {hasWorkspaces
                    ? "Select a workspace to continue where you left off."
                    : "A workspace keeps your connections, queues, jobs, and tabs organized in one focused place."}
            </p>
        </div>

        {#if hasWorkspaces}
            <Card.Root class="overflow-hidden border shadow-sm">
                <Card.Header class="border-b bg-muted/20 px-6 py-5">
                    <Card.Title class="text-base">Your workspaces</Card.Title>
                    <Card.Description>
                        Choose one to open the Bullastrator workspace.
                    </Card.Description>
                </Card.Header>
                <Card.Content class="grid gap-2 p-3 sm:grid-cols-2">
                    {#each workspaces as workspace (workspace.id)}
                        <Button
                            variant="ghost"
                            class="h-auto min-h-18 justify-start gap-3 rounded-lg border px-4 py-3 text-left hover:border-primary/40 hover:bg-primary/5"
                            disabled={isPending}
                            onclick={() => selectWorkspace(workspace.id)}
                        >
                            <span
                                class="flex size-10 shrink-0 items-center justify-center rounded-xl text-base font-semibold text-white shadow-sm"
                                style:background-color={workspace.color ??
                                    "#64748b"}
                            >
                                {workspace.name.substring(0, 1).toUpperCase()}
                            </span>
                            <span class="min-w-0 flex-1">
                                <span class="block truncate font-medium"
                                    >{workspace.name}</span
                                >
                                <span
                                    class="mt-1 block text-xs uppercase tracking-wide text-muted-foreground"
                                >
                                    {workspace.role}
                                </span>
                            </span>
                            {#if isPending}
                                <LoaderCircle
                                    class="size-4 shrink-0 animate-spin"
                                />
                            {/if}
                        </Button>
                    {/each}
                </Card.Content>
            </Card.Root>
        {:else}
            <div class="rounded-2xl">
                <WorkspaceForm onComplete={() => {}} />
            </div>
            <p class="mt-5 text-center text-xs text-muted-foreground">
                You can change the workspace name and appearance later from
                workspace settings.
            </p>
        {/if}
    </div>
</div>
