<script lang="ts">
    import * as Avatar from "shared/components/ui/avatar";
    import { Alert, AlertDescription, AlertTitle } from "ui/alert";
    import { Button } from "ui/button";
    import { Skeleton } from "ui/skeleton";

    import DialogWrapper from "components/wrapper/DialogWrapper.svelte";
    import SidebarWrapper from "components/wrapper/SidebarWrapper.svelte";
    import ConnectionForm from "./ConnectionForm.svelte";

    import { CircleAlert, Link2, Plus, Trash } from "@lucide/svelte";

    import EmptyConnectionSideBar from "connection/components/EmptyConnectionSideBar.svelte";
    import { cn } from "shared/utils";
    import {
        useDeleteConnection,
        useGetConnections,
    } from "../hooks/connection.hooks";

    const connectionsQuery = useGetConnections();
    const { mutateAsync: deleteConnection } = useDeleteConnection();

    const avatarStyle = (color?: string) =>
        `background-color: ${color || "var(--primary)"}`;
</script>

<SidebarWrapper>
    {#snippet header()}
        <h1>Connections</h1>

        <DialogWrapper
            title="Add new Connection"
            description="Complete to add new Connection"
        >
            {#snippet trigger()}
                <Plus class="h-4" />
            {/snippet}

            {#snippet children(onComplete)}
                <ConnectionForm {onComplete} />
            {/snippet}
        </DialogWrapper>
    {/snippet}

    {#snippet content()}
        {#if connectionsQuery.isLoading}
            <Skeleton class="h-10 w-full" />
        {:else if connectionsQuery.isError}
            <Alert variant="destructive">
                <CircleAlert class="h-4 w-4" />
                <AlertTitle>Error</AlertTitle>
                <AlertDescription>
                    Failed to load connections: {connectionsQuery.error.message}
                </AlertDescription>
            </Alert>
        {:else if connectionsQuery.data && connectionsQuery.data.length > 0}
            <ul>
                {#each connectionsQuery.data as connection}
                    <li class="group relative list-none">
                        <DialogWrapper
                            title="Edit Connection"
                            description="Update your Redis configuration"
                            triggerClass="w-full text-left transition-all"
                        >
                            {#snippet trigger()}
                                <div
                                    class={cn(
                                        "flex items-center gap-3 rounded-md border border-transparent",
                                        " transition-colors cursor-pointer px-3",
                                    )}
                                >
                                    <Avatar.Root class="h-10 w-10 rounded-lg">
                                        <Avatar.Fallback
                                            class="rounded-lg text-white font-bold text-xs"
                                            style={avatarStyle(
                                                connection.color,
                                            )}
                                        >
                                            {connection.label
                                                ?.substring(0, 2)
                                                .toUpperCase() || "RD"}
                                        </Avatar.Fallback>
                                    </Avatar.Root>

                                    <div
                                        class="flex-1 min-w-0 flex flex-col gap-0.5"
                                    >
                                        <div class="flex items-center gap-2">
                                            <span
                                                class="font-medium truncate text-sm text-foreground"
                                            >
                                                {connection.name}
                                            </span>
                                            <!-- <Badge
                        variant="outline"
                        class="text-[10px] px-1.5 py-0 h-4 font-mono text-black"
                        style={avatarStyle(connection.color)}
                      >
                        {connection.host}:{connection.port}
                      </Badge> -->
                                        </div>
                                        <div
                                            class="flex items-center gap-1 text-muted-foreground"
                                        >
                                            <Link2 size={12} />
                                            <span class="text-xs truncate"
                                                >BullMQ / Redis</span
                                            >
                                        </div>
                                    </div>

                                    <div
                                        class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity"
                                    >
                                        <Button
                                            variant="ghost"
                                            size="icon"
                                            class="h-8 w-8 text-destructive hover:text-destructive"
                                            onclick={(e) => {
                                                e.preventDefault();
                                                e.stopPropagation();
                                                e.stopImmediatePropagation();
                                                deleteConnection(connection.id);
                                            }}
                                        >
                                            <Trash size={14} />
                                        </Button>
                                    </div>
                                </div>
                            {/snippet}

                            {#snippet children(onComplete)}
                                <ConnectionForm
                                    {onComplete}
                                    connectionId={connection.id}
                                />
                            {/snippet}
                        </DialogWrapper>
                    </li>
                {/each}
            </ul>
        {:else}
            <EmptyConnectionSideBar />
        {/if}
    {/snippet}
</SidebarWrapper>
