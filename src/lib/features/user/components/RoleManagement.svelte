<script lang="ts">
    import { Search, Trash } from "@lucide/svelte";
    import * as Tabs from "shared/components/ui/tabs";
    import {
        useCreateUserRole,
        useDeleteUserRole,
        useGetRoleOptions,
        useGetUserRoles,
        useUpdateUserRole,
    } from "../hooks/user.roles.hooks";
    import type { RoleEntityOption, UserRole } from "../interface";
    import Button from "components/ui/button/button.svelte";
    import Input from "components/ui/input/input.svelte";
    import Checkbox from "components/ui/checkbox/checkbox.svelte";
    import * as Select from "components/ui/select";
    import * as Resizable from "components/ui/resizable";

    type AccessTab = "WORKSPACE" | "CONNECTION" | "FOLDER" | "QUEUE";

    const tabs: Array<{ value: AccessTab; label: string }> = [
        { value: "WORKSPACE", label: "Workspaces" },
        { value: "CONNECTION", label: "Connections" },
        { value: "FOLDER", label: "Folders" },
        { value: "QUEUE", label: "Queues" },
    ];

    const roleOptions = [
        { value: "VIEWER", label: "Viewer" },
        { value: "EDITOR", label: "Editor" },
        { value: "ADMIN", label: "Administrator" },
        { value: "OWNER", label: "Owner" },
    ];

    let { selectedUserId }: { selectedUserId: string } = $props();

    let activeTab = $state<AccessTab>("WORKSPACE");
    let input = $state("");

    const rolesQuery = useGetUserRoles(() => selectedUserId);
    const optionsQuery = useGetRoleOptions(() => activeTab);
    const createRole = useCreateUserRole(() => selectedUserId);
    const deleteRole = useDeleteUserRole(() => selectedUserId);
    const updateRole = useUpdateUserRole(() => selectedUserId);

    const matchingOptions = $derived(
        (optionsQuery.data ?? []).filter((option) =>
            option.name.toLowerCase().includes(input.trim().toLowerCase()),
        ),
    );

    const visibleRoles = $derived(
        (rolesQuery.data ?? []).filter((role) => role.entityType === activeTab),
    );

    function roleForEntity(entityId: string) {
        return visibleRoles.find((role) => role.entityId === entityId);
    }

    function isAssigned(entityId: string) {
        return Boolean(roleForEntity(entityId));
    }

    function toggleAssignment(entity: RoleEntityOption) {
        const role = roleForEntity(entity.id);

        if (role) {
            deleteRole.mutate(role.id);
            return;
        }

        createRole.mutate({
            userId: selectedUserId,
            entityType: activeTab,
            entityId: entity.id,
            role: "VIEWER",
        });
    }

    function changeRole(role: UserRole, value: string) {
        if (value === role.role) return;
        updateRole.mutate({ id: role.id, role: value });
    }
</script>

<Tabs.Root
    bind:value={activeTab}
    class="flex h-full min-h-0 flex-1 flex-col gap-0"
>
    <Tabs.List class="mx-6 mt-4 grid h-auto w-auto grid-cols-4 rounded-md">
        {#each tabs as tab}
            <Tabs.Trigger value={tab.value}>{tab.label}</Tabs.Trigger>
        {/each}
    </Tabs.List>

    {#each tabs as tab}
        <Tabs.Content
            value={tab.value}
            class="flex h-full min-h-0 flex-1 flex-col overflow-hidden px-6 py-5"
        >
            <Resizable.PaneGroup
                direction="horizontal"
                class="h-full min-h-0 flex-1 w-full gap-1"
            >
                <Resizable.Pane
                    defaultSize={50}
                    minSize={50}
                    class="min-h-0 flex-1 min-w-0 overflow-y-auto flex  flex-col gap-1!"
                >
                    {#if visibleRoles.length === 0}
                        <div
                            class="rounded-lg border border-dashed px-4 py-12 text-center text-sm text-muted-foreground"
                        >
                            Select a {tab.label.toLowerCase().slice(0, -1)} to assign
                            access.
                        </div>
                    {:else}
                        {#each visibleRoles as role (role.id)}
                            <div
                                class="flex min-h-16 items-center gap-3 border bg-card rounded-lg px-3 py-2.5 sm:px-4"
                            >
                                <div
                                    class="flex size-9 shrink-0 items-center justify-center rounded-full bg-primary/10 text-sm font-semibold text-primary ring-1 ring-primary/15"
                                >
                                    {role.entityName
                                        .substring(0, 1)
                                        .toUpperCase()}
                                </div>
                                <div class="min-w-0 flex-1">
                                    <p class="truncate text-sm font-medium">
                                        {role.entityName}
                                    </p>
                                    <p
                                        class="mt-0.5 text-xs text-muted-foreground"
                                    >
                                        {tab.label.slice(0, -1)} access
                                    </p>
                                </div>
                                <Select.Root
                                    type="single"
                                    value={role.role}
                                    onValueChange={(value) =>
                                        changeRole(role, value)}
                                >
                                    <Select.Trigger
                                        class="h-8 w-32 bg-background text-xs"
                                    >
                                        {roleOptions.find(
                                            (option) =>
                                                option.value === role.role,
                                        )?.label ?? role.role}
                                    </Select.Trigger>
                                    <Select.Content>
                                        {#each roleOptions as option}
                                            <Select.Item value={option.value}
                                                >{option.label}</Select.Item
                                            >
                                        {/each}
                                    </Select.Content>
                                </Select.Root>
                                <Button
                                    type="button"
                                    variant="ghost"
                                    size="icon"
                                    class="size-8 shrink-0 text-muted-foreground hover:bg-red-500/10 hover:text-red-400"
                                    aria-label={`Remove ${role.entityName} role`}
                                    onclick={() => deleteRole.mutate(role.id)}
                                >
                                    <Trash class="size-4" />
                                </Button>
                            </div>
                        {/each}
                    {/if}
                </Resizable.Pane>

                <Resizable.Handle class="bg-transparent" />

                <Resizable.Pane
                    defaultSize={30}
                    minSize={30}
                    class="flex min-h-0 min-w-0 flex-col overflow-hidden rounded-lg border bg-card"
                >
                    <div class="shrink-0 border-b px-4 py-3">
                        <h3 class="text-sm font-semibold">{tab.label}</h3>
                        <p class="mt-1 text-xs text-muted-foreground">
                            Select resources to grant access.
                        </p>
                        <div class="relative mt-3">
                            <Search
                                class="pointer-events-none absolute left-3 top-1/2 size-4 -translate-y-1/2 text-muted-foreground"
                            />
                            <Input
                                bind:value={input}
                                placeholder={`Search ${tab.label.toLowerCase()}...`}
                                class="pl-9"
                            />
                        </div>
                    </div>

                    <div class="min-h-0 flex-1 overflow-y-auto p-2">
                        {#if optionsQuery.isLoading}
                            <p
                                class="px-3 py-6 text-center text-sm text-muted-foreground"
                            >
                                Loading {tab.label.toLowerCase()}...
                            </p>
                        {:else if matchingOptions.length === 0}
                            <p
                                class="px-3 py-6 text-center text-sm text-muted-foreground"
                            >
                                {input
                                    ? "No matching resources."
                                    : `No ${tab.label.toLowerCase()} available.`}
                            </p>
                        {:else}
                            {#each matchingOptions as option (option.id)}
                                <label
                                    class="flex cursor-pointer items-center gap-3 rounded-md px-3 py-2.5 transition-colors hover:bg-muted/60"
                                >
                                    <Checkbox
                                        checked={isAssigned(option.id)}
                                        onCheckedChange={() =>
                                            toggleAssignment(option)}
                                    />
                                    <span
                                        class="flex size-8 shrink-0 items-center justify-center rounded-full bg-primary/10 text-xs font-semibold text-primary"
                                    >
                                        {option.name
                                            .substring(0, 1)
                                            .toUpperCase()}
                                    </span>
                                    <span
                                        class="min-w-0 flex-1 truncate text-sm font-medium"
                                    >
                                        {option.name}
                                    </span>
                                </label>
                            {/each}
                        {/if}
                    </div>
                </Resizable.Pane>
            </Resizable.PaneGroup>
        </Tabs.Content>
    {/each}
</Tabs.Root>
