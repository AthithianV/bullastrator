<script lang="ts">
    import { UserRound, Users } from "@lucide/svelte";
    import * as Tabs from "shared/components/ui/tabs";
    import * as Avatar from "shared/components/ui/avatar";
    import Separator from "shared/components/ui/separator/separator.svelte";
    import Button from "components/ui/button/button.svelte";
    import DialogWrapper from "components/wrapper/DialogWrapper.svelte";

    type AccessTab = "workspaces" | "connections" | "folders" | "queues";

    let open = $state(false);
    let activeTab = $state<AccessTab>("workspaces");

    // The user list will be populated by the user-management API.
    const users: Array<{ id: string; name: string; email: string }> = [];
    const selectedUser = $derived(users[0] ?? null);

    const tabs: Array<{ value: AccessTab; label: string }> = [
        { value: "workspaces", label: "Workspaces" },
        { value: "connections", label: "Connections" },
        { value: "folders", label: "Folders" },
        { value: "queues", label: "Queues" },
    ];
</script>

<DialogWrapper
    title="User management"
    description="Review each user's workspace and resource access."
    contentClass="sm:max-w-[95vw] md:max-w-[90vw] lg:max-w-[80vw] min-h-[80vh]"
>
    {#snippet trigger()}
        <Button
            variant="ghost"
            type="button"
            class="rounded-full p-1.5"
            aria-label="Open user management"
            title="User management"
            onclick={() => (open = true)}
        >
            <Avatar.Root class="size-8">
                <Avatar.Fallback class="bg-background text-primary">
                    <UserRound class="size-4" />
                </Avatar.Fallback>
            </Avatar.Root>
        </Button>
    {/snippet}

    {#snippet children()}
        <div class="flex flex-1 gap-1 h-[90%]">
            <aside class="w-[35%] shrink-0 rounded-lg bg-background">
                <div
                    class="px-4 py-3 text-xs font-semibold uppercase tracking-wide text-muted-foreground"
                >
                    Users
                </div>
                {#if users.length === 0}
                    <div
                        class="px-4 py-8 text-center text-sm text-muted-foreground"
                    >
                        Users will appear here when user management is
                        connected.
                    </div>
                {:else}
                    {#each users as user (user.id)}
                        <button
                            type="button"
                            class="flex w-full items-center gap-3 px-4 py-3 text-left hover:bg-accent"
                        >
                            <Avatar.Root class="size-8">
                                <Avatar.Fallback
                                    >{user.name.slice(0, 1)}</Avatar.Fallback
                                >
                            </Avatar.Root>
                            <span class="min-w-0">
                                <span class="block truncate text-sm font-medium"
                                    >{user.name}</span
                                >
                                <span
                                    class="block truncate text-xs text-muted-foreground"
                                    >{user.email}</span
                                >
                            </span>
                        </button>
                    {/each}
                {/if}
            </aside>

            <main
                class="min-w-0 flex-1 overflow-y-auto bg-background rounded-lg"
            >
                {#if selectedUser}
                    <div class="flex items-center gap-3 px-6 py-5">
                        <Avatar.Root class="size-11">
                            <Avatar.Fallback
                                >{selectedUser.name.slice(
                                    0,
                                    1,
                                )}</Avatar.Fallback
                            >
                        </Avatar.Root>
                        <div>
                            <h2 class="font-semibold">{selectedUser.name}</h2>
                            <p class="text-sm text-muted-foreground">
                                {selectedUser.email}
                            </p>
                        </div>
                    </div>
                    <Separator />
                {:else}
                    <div
                        class="flex flex-col items-center justify-center px-8 text-center text-muted-foreground"
                    >
                        <Users class="mb-3 size-10 opacity-40" />
                        <p class="font-medium">Select a user to view access</p>
                        <p class="mt-1 text-sm">
                            Workspace, connection, folder, and queue permissions
                            will appear here.
                        </p>
                    </div>
                {/if}

                <Tabs.Root bind:value={activeTab} class="gap-0">
                    <Tabs.List
                        class="mx-6 mt-4 grid h-auto w-auto grid-cols-4 rounded-md"
                    >
                        {#each tabs as tab}
                            <Tabs.Trigger
                                value={tab.value}
                                disabled={!selectedUser}
                            >
                                {tab.label}
                            </Tabs.Trigger>
                        {/each}
                    </Tabs.List>

                    {#each tabs as tab}
                        <Tabs.Content value={tab.value} class="px-6 py-5">
                            <div
                                class="rounded-md border border-dashed p-8 text-center text-sm text-muted-foreground"
                            >
                                {selectedUser
                                    ? `No ${tab.label.toLowerCase()} access loaded yet.`
                                    : "Select a user to view access."}
                            </div>
                        </Tabs.Content>
                    {/each}
                </Tabs.Root>
            </main>
        </div>
    {/snippet}
</DialogWrapper>
