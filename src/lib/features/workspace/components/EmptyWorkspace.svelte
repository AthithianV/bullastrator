<script lang="ts">
    import {
        Boxes,
        Check,
        Layers3,
        LoaderCircle,
        Plus,
        ShieldCheck,
    } from "@lucide/svelte";
    import { PUBLIC_IS_WEB } from "$env/static/public";
    import * as Card from "ui/card";
    import { Button } from "ui/button";
    import { useSetActiveWorkspace } from "workspace/hooks/workspace.hooks";
    import type { ReadWorkspaceModel } from "../interface/workspace.types";
    import WorkspaceForm from "./WorkspaceForm.svelte";

    let { workspaces = [] } = $props<{
        workspaces?: ReadWorkspaceModel[];
    }>();

    const { mutateAsync: setActiveWorkspace, isPending } =
        useSetActiveWorkspace();

    const hasWorkspaces = $derived(workspaces.length > 0);

    async function selectWorkspace(workspaceId: string) {
        await setActiveWorkspace(workspaceId);
    }
</script>

<div class="flex min-h-full items-center justify-center p-6">
    <Card.Root class="w-full max-w-4xl overflow-hidden border shadow-sm">
        <div class="grid md:grid-cols-[1fr_1.1fr]">
            <div
                class="flex flex-col justify-center gap-6 bg-primary/10 p-8 md:p-10"
            >
                <div
                    class="flex size-14 items-center justify-center rounded-2xl bg-primary text-primary-foreground shadow-sm"
                >
                    <Boxes size={28} strokeWidth={1.8} />
                </div>

                <div class="space-y-3">
                    <p
                        class="text-sm font-semibold uppercase tracking-[0.18em] text-primary"
                    >
                        {hasWorkspaces
                            ? "Choose a workspace"
                            : "Your workspace"}
                    </p>
                    <h1 class="text-3xl font-bold tracking-tight">
                        {hasWorkspaces
                            ? "Where should we take you?"
                            : "A home for your queues."}
                    </h1>
                    <p class="text-sm leading-6 text-muted-foreground">
                        {hasWorkspaces
                            ? "Select a workspace to continue organizing your connections, queues, jobs, and tabs."
                            : "Create or select a workspace to organize connections, queues, jobs, and tabs in one place."}
                    </p>
                </div>

                <div class="space-y-3 text-sm text-muted-foreground">
                    <div class="flex items-center gap-3">
                        <Check class="size-4 text-primary" />
                        <span>Keep related Redis connections together</span>
                    </div>
                    <div class="flex items-center gap-3">
                        <Layers3 class="size-4 text-primary" />
                        <span>Switch between projects quickly</span>
                    </div>
                    {#if PUBLIC_IS_WEB}
                        <div class="flex items-center gap-3">
                            <ShieldCheck class="size-4 text-primary" />
                            <span>Collaborate with workspace members</span>
                        </div>
                    {/if}
                </div>
            </div>

            <div class="p-8 md:p-10">
                {#if hasWorkspaces}
                    <Card.Header class="px-0">
                        <Card.Title class="text-xl">Your workspaces</Card.Title>
                        <Card.Description>
                            Choose one to set it as your active workspace.
                        </Card.Description>
                    </Card.Header>
                    <Card.Content class="space-y-3 px-0 pt-6">
                        {#each workspaces as workspace (workspace.id)}
                            <Button
                                variant="outline"
                                class="h-auto w-full justify-start gap-3 px-4 py-3 text-left"
                                disabled={isPending}
                                onclick={() => selectWorkspace(workspace.id)}
                            >
                                <span
                                    class="flex size-9 shrink-0 items-center justify-center rounded-lg text-base"
                                    style:background-color={workspace.color ??
                                        "#94a3b8"}
                                >
                                    {workspace.name.substring(0, 1)}
                                </span>
                                <span class="min-w-0 flex-1">
                                    <span class="block truncate font-medium">
                                        {workspace.name}
                                    </span>
                                    <span
                                        class="block text-xs text-muted-foreground"
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
                {:else}
                    <Card.Header class="px-0">
                        <Card.Title class="flex items-center gap-2 text-xl">
                            <Plus class="size-5 text-primary" />
                            Create a workspace
                        </Card.Title>
                        <Card.Description>
                            Give your workspace a name and choose an optional
                            icon and color.
                        </Card.Description>
                    </Card.Header>
                    <Card.Content class="px-0 pt-6">
                        <WorkspaceForm />
                    </Card.Content>
                {/if}
            </div>
        </div>
    </Card.Root>
</div>
