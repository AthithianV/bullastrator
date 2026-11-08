<script lang="ts">
    import { UserRound, Users, X } from "@lucide/svelte";
    import * as Avatar from "shared/components/ui/avatar";
    import * as Tabs from "shared/components/ui/tabs";
    import * as Resizable from "shared/components/ui/resizable";
    import Separator from "shared/components/ui/separator/separator.svelte";
    import Button from "components/ui/button/button.svelte";
    import DialogWrapper from "components/wrapper/DialogWrapper.svelte";
    import { useGetAllUsers } from "../hooks/user.hooks";
    import {
        useCreateUserRole,
        useDeleteUserRole,
        useGetRoleOptions,
        useGetUserRoles,
    } from "../hooks/user.roles.hooks";
    import type { RoleEntityOption } from "../interface";

    type AccessTab = "WORKSPACE" | "CONNECTION" | "FOLDER" | "QUEUE";
    const tabs: Array<{ value: AccessTab; label: string }> = [
        { value: "WORKSPACE", label: "Workspaces" },
        { value: "CONNECTION", label: "Connections" },
        { value: "FOLDER", label: "Folders" },
        { value: "QUEUE", label: "Queues" },
    ];
    const roleChoices: Record<AccessTab, string[]> = {
        WORKSPACE: ["VIEWER", "EDITOR", "ADMIN", "OWNER"],
        CONNECTION: ["VIEWER", "EDITOR"],
        FOLDER: ["VIEWER", "EDITOR"],
        QUEUE: ["VIEWER", "EDITOR"],
    };
    let open = $state(false),
        activeTab = $state<AccessTab>("WORKSPACE"),
        selectedUserId = $state<string | null>(null);
    let input = $state(""),
        pendingEntity = $state<RoleEntityOption | null>(null),
        pendingRole = $state("VIEWER");

    const usersQuery = useGetAllUsers(() => open);
    const users = $derived(usersQuery.data ?? []);
    const rolesQuery = useGetUserRoles(
        () => selectedUserId,
        () => open,
    );
    const optionsQuery = useGetRoleOptions(
        () => activeTab,
        () => open,
    );
    const createRole = useCreateUserRole(() => selectedUserId);
    const deleteRole = useDeleteUserRole(() => selectedUserId);

    $effect(() => {
        if (
            users.length > 0 &&
            (!selectedUserId ||
                !users.some((user) => user.id === selectedUserId))
        )
            selectedUserId = users[0].id;
        if (users.length === 0) selectedUserId = null;
    });
    const matchingOptions = $derived(
        (optionsQuery.data ?? []).filter((option) =>
            option.name.toLowerCase().includes(input.toLowerCase()),
        ),
    );
    const selectedUser = $derived(
        users.find((user) => user.id === selectedUserId) ?? null,
    );
    const visibleRoles = $derived(
        (rolesQuery.data ?? []).filter((role) => role.entityType === activeTab),
    );
    function chooseEntity(entity: RoleEntityOption) {
        pendingEntity = entity;
        input = entity.name;
    }
    function assignRole() {
        if (selectedUserId && pendingEntity)
            createRole.mutate({
                userId: selectedUserId,
                entityType: activeTab,
                entityId: pendingEntity.id,
                role: pendingRole,
            });
        pendingEntity = null;
        input = "";
    }
</script>

<DialogWrapper
    bind:open
    title="User management"
    description="Manage scoped roles for every user"
    contentClass="sm:max-w-[95vw] md:max-w-[90vw] lg:max-w-[80vw] h-[90vh] p-4!"
    bodyClass="flex flex-1 min-h-0 flex-col overflow-hidden bg-card"
>
    {#snippet trigger()}<Button
            variant="ghost"
            type="button"
            class="rounded-full p-1.5"
            aria-label="Open user management"
            title="User management"
            onclick={() => (open = true)}
            ><Avatar.Root class="size-8"
                ><Avatar.Fallback class="bg-background text-primary"
                    ><UserRound class="size-4" /></Avatar.Fallback
                ></Avatar.Root
            ></Button
        >{/snippet}
    {#snippet children()}
        <Resizable.PaneGroup
            direction="horizontal"
            class="min-h-0 flex-1 gap-0.5 border-none"
        >
            <Resizable.Pane
                defaultSize={25}
                minSize={20}
                maxSize={50}
                class="min-w-0 overflow-y-auto rounded-lg bg-background"
                ><div
                    class="px-4 py-3 text-xs font-semibold uppercase tracking-wide text-muted-foreground"
                >
                    Users
                </div>
                {#each users as user (user.id)}<button
                        type="button"
                        class="flex w-full items-center gap-3 px-4 py-3 text-left hover:bg-accent"
                        class:bg-accent={selectedUserId === user.id}
                        onclick={() => (selectedUserId = user.id)}
                        ><Avatar.Root class="size-8"
                            ><Avatar.Fallback
                                >{user.name.slice(0, 1)}</Avatar.Fallback
                            ></Avatar.Root
                        ><span class="min-w-0"
                            ><span class="block truncate text-sm font-medium"
                                >{user.name}</span
                            ><span
                                class="block truncate text-xs text-muted-foreground"
                                >{user.email}</span
                            ></span
                        ></button
                    >{/each}</Resizable.Pane
            >
            <Resizable.Handle class="bg-card" />
            <Resizable.Pane
                defaultSize={65}
                minSize={50}
                class="min-w-0 overflow-y-auto rounded-lg bg-background"
            >
                {#if selectedUser}<div
                        class="flex items-center gap-3 px-6 py-5"
                    >
                        <Avatar.Root class="size-11"
                            ><Avatar.Fallback
                                >{selectedUser.name.slice(
                                    0,
                                    1,
                                )}</Avatar.Fallback
                            ></Avatar.Root
                        >
                        <div>
                            <h2 class="font-semibold">{selectedUser.name}</h2>
                            <p class="text-sm text-muted-foreground">
                                {selectedUser.email}
                            </p>
                        </div>
                    </div>
                    <Separator /><Tabs.Root bind:value={activeTab} class="gap-0"
                        ><Tabs.List
                            class="mx-6 mt-4 grid h-auto w-auto grid-cols-4 rounded-md"
                            >{#each tabs as tab}<Tabs.Trigger value={tab.value}
                                    >{tab.label}</Tabs.Trigger
                                >{/each}</Tabs.List
                        >{#each tabs as tab}<Tabs.Content
                                value={tab.value}
                                class="px-6 py-5"
                                ><div class="relative flex gap-2">
                                    <input
                                        class="h-9 min-w-0 flex-1 rounded-md border bg-background px-3 text-sm outline-none focus:ring-2 focus:ring-ring"
                                        placeholder={`Add ${tab.label.slice(0, -1).toLowerCase()}...`}
                                        bind:value={input}
                                        onfocus={() => (pendingEntity = null)}
                                    /><select
                                        class="h-9 rounded-md border bg-background px-2 text-sm"
                                        bind:value={pendingRole}
                                        >{#each roleChoices[tab.value] as role}<option
                                                value={role}>{role}</option
                                            >{/each}</select
                                    ><Button
                                        size="sm"
                                        disabled={!pendingEntity ||
                                            createRole.isPending}
                                        onclick={assignRole}>Add</Button
                                    >{#if input && !pendingEntity && matchingOptions.length > 0}<div
                                            class="absolute left-0 top-10 z-10 max-h-48 w-[calc(100%-9rem)] overflow-y-auto rounded-md border bg-popover p-1 shadow-md"
                                        >
                                            {#each matchingOptions as option}<button
                                                    type="button"
                                                    class="block w-full rounded px-2 py-1.5 text-left text-sm hover:bg-accent"
                                                    onclick={() =>
                                                        chooseEntity(option)}
                                                    >{option.name}</button
                                                >{/each}
                                        </div>{/if}
                                </div>
                                <div class="mt-4 flex flex-wrap gap-2">
                                    {#each visibleRoles as role (role.id)}<span
                                            class="inline-flex items-center gap-1 rounded-full border bg-muted px-3 py-1 text-xs"
                                            ><span class="font-medium"
                                                >{role.role}</span
                                            ><span class="text-muted-foreground"
                                                >· {role.entityName}</span
                                            ><button
                                                type="button"
                                                aria-label={`Remove ${role.entityName} role`}
                                                onclick={() =>
                                                    deleteRole.mutate(role.id)}
                                                ><X class="size-3" /></button
                                            ></span
                                        >{:else}<p
                                            class="text-sm text-muted-foreground"
                                        >
                                            No {tab.label.toLowerCase()} roles assigned.
                                        </p>{/each}
                                </div></Tabs.Content
                            >{/each}</Tabs.Root
                    >{:else}<div
                        class="flex flex-1 flex-col items-center justify-center px-8 text-center text-muted-foreground"
                    >
                        <Users class="mb-3 size-10 opacity-40" />
                        <p class="font-medium">
                            Select a user to manage access
                        </p>
                    </div>{/if}
            </Resizable.Pane>
        </Resizable.PaneGroup>
    {/snippet}
</DialogWrapper>
