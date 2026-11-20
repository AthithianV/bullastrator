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
    import * as Command from "components/ui/command";

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
    let assignedInput = $state("");
    let selectedConnectionIds = $state<string[]>([]);

    const rolesQuery = useGetUserRoles(
        () => selectedUserId,
        () => activeTab,
        () => assignedInput,
    );
    const optionsQuery = useGetRoleOptions(
        () => activeTab,
        () => selectedUserId,
        () => input,
    );
    const createRole = useCreateUserRole(() => selectedUserId);
    const deleteRole = useDeleteUserRole(() => selectedUserId);
    const updateRole = useUpdateUserRole(() => selectedUserId);

    const allRolePages = $derived(
        rolesQuery.data?.pages.flatMap((page) => page.items) ?? [],
    );
    const allOptionPages = $derived(
        optionsQuery.data?.pages.flatMap((page) => page.items) ?? [],
    );

    const matchingOptions = $derived(allOptionPages);

    type OptionGroup = {
        id: string;
        name: string;
        options: RoleEntityOption[];
    };

    function groupByWorkspace(options: RoleEntityOption[]): OptionGroup[] {
        const groups = new Map<string, OptionGroup>();

        for (const option of options) {
            const id = option.workspaceId ?? "unassigned";
            const group = groups.get(id) ?? {
                id,
                name: option.workspaceName ?? "Workspace",
                options: [],
            };
            group.options.push(option);
            groups.set(id, group);
        }

        return [...groups.values()];
    }

    const workspaceGroups = $derived(groupByWorkspace(matchingOptions));
    const connectionGroups = $derived(
        groupByWorkspace(
            matchingOptions.filter(
                (option, index, options) =>
                    options.findIndex(
                        (candidate) =>
                            candidate.connectionId === option.connectionId,
                    ) === index,
            ),
        ),
    );
    const visibleConnectionGroups = $derived(
        selectedConnectionIds.length === 0
            ? connectionGroups
            : connectionGroups
                  .map((group) => ({
                      ...group,
                      options: group.options.filter((option) =>
                          selectedConnectionIds.includes(
                              option.connectionId ?? "",
                          ),
                      ),
                  }))
                  .filter((group) => group.options.length > 0),
    );

    const visibleRoles = $derived(
        allRolePages.filter((role) => role.entityType === activeTab),
    );
    const assignedItems = $derived(
        visibleRoles.map((role) => ({
            id: role.entityId,
            name: role.entityName,
            workspaceId: role.workspaceId,
            workspaceName: role.workspaceName,
            connectionId: role.connectionId,
            connectionName: role.connectionName,
        })),
    );
    const assignedWorkspaceGroups = $derived(groupByWorkspace(assignedItems));
    const assignedConnectionGroups = $derived(
        groupByWorkspace(
            assignedItems.filter(
                (option, index, options) =>
                    options.findIndex(
                        (candidate) =>
                            candidate.connectionId === option.connectionId,
                    ) === index,
            ),
        ),
    );

    function roleForEntity(entityId: string) {
        return visibleRoles.find((role) => role.entityId === entityId);
    }

    function rolesForConnection(connectionId: string) {
        return visibleRoles.filter(
            (role) => role.connectionId === connectionId,
        );
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

    function changeScope(
        role: UserRole,
        scope: "folderScope" | "queueScope",
        value: "SELECTED" | "ALL",
    ) {
        updateRole.mutate({
            id: role.id,
            role: role.role,
            [scope]: value,
        });
    }

    function toggleConnection(connectionId: string) {
        selectedConnectionIds = selectedConnectionIds.includes(connectionId)
            ? selectedConnectionIds.filter((id) => id !== connectionId)
            : [...selectedConnectionIds, connectionId];
    }

    function selectTab(tab: AccessTab) {
        activeTab = tab;
        selectedConnectionIds = [];
        input = "";
        assignedInput = "";
    }

    function loadMoreRoles(event: Event) {
        const element = event.currentTarget as HTMLElement;
        if (
            element.scrollTop + element.clientHeight >=
                element.scrollHeight - 80 &&
            rolesQuery.hasNextPage &&
            !rolesQuery.isFetchingNextPage
        ) {
            void rolesQuery.fetchNextPage();
        }
    }

    function loadMoreOptions(event: Event) {
        const element = event.currentTarget as HTMLElement;
        if (
            element.scrollTop + element.clientHeight >=
                element.scrollHeight - 80 &&
            optionsQuery.hasNextPage &&
            !optionsQuery.isFetchingNextPage
        ) {
            void optionsQuery.fetchNextPage();
        }
    }
</script>

{#snippet roleCard(role: UserRole, label: string)}
    <div
        class="mb-1 flex min-h-16 flex-wrap items-center gap-2 rounded-lg bg-card px-3 py-2.5 sm:gap-3 sm:px-4"
    >
        <div
            class="flex size-9 shrink-0 items-center justify-center rounded-full bg-primary/10 text-sm font-semibold text-primary ring-1 ring-primary/15"
        >
            {role.entityName.substring(0, 1).toUpperCase()}
        </div>
        <div class="min-w-0 flex-1">
            <p class="truncate text-sm font-medium">{role.entityName}</p>
            <p class="mt-0.5 text-xs text-muted-foreground">{label} access</p>
        </div>
        <Select.Root
            type="single"
            value={role.role}
            onValueChange={(value) => changeRole(role, value)}
        >
            <Select.Trigger class="h-8 w-32 bg-background text-xs">
                {roleOptions.find((option) => option.value === role.role)
                    ?.label ?? role.role}
            </Select.Trigger>
            <Select.Content>
                {#each roleOptions as option}
                    <Select.Item value={option.value}
                        >{option.label}</Select.Item
                    >
                {/each}
            </Select.Content>
        </Select.Root>
        {#if role.entityType === "CONNECTION"}
            <div class="flex shrink-0 items-center gap-3">
                <label class="flex flex-col items-center justify-center gap-1">
                    <span class="text-xs font-medium text-foreground"
                        >All folder access</span
                    >
                    <span
                        class="flex items-center gap-1.5 text-[11px] text-muted-foreground"
                    >
                        <Checkbox
                            checked={role.folderScope === "ALL"}
                            onCheckedChange={(checked) =>
                                changeScope(
                                    role,
                                    "folderScope",
                                    checked === true ? "ALL" : "SELECTED",
                                )}
                        />
                    </span>
                </label>
                <label class="flex flex-col items-center gap-1">
                    <span class="text-xs font-medium text-foreground"
                        >All queue access</span
                    >
                    <span
                        class="flex items-center gap-1.5 text-[11px] text-muted-foreground"
                    >
                        <Checkbox
                            checked={role.queueScope === "ALL"}
                            onCheckedChange={(checked) =>
                                changeScope(
                                    role,
                                    "queueScope",
                                    checked === true ? "ALL" : "SELECTED",
                                )}
                        />
                    </span>
                </label>
            </div>
        {/if}
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
{/snippet}

<Tabs.Root
    bind:value={activeTab}
    class="flex h-full min-h-0 flex-1 flex-col gap-0"
>
    <Tabs.List
        class="mx-2 py-2 rounded-none border-b grid h-auto w-auto grid-cols-4"
    >
        {#each tabs as tab}
            <Tabs.Trigger value={tab.value} onclick={() => selectTab(tab.value)}
                >{tab.label}</Tabs.Trigger
            >
        {/each}
    </Tabs.List>

    {#each tabs as tab}
        <Tabs.Content
            value={tab.value}
            class="flex min-h-0 flex-1 flex-col overflow-hidden p-2"
        >
            <Resizable.PaneGroup
                direction="horizontal"
                class="min-h-0 flex-1 w-full gap-1"
            >
                <Resizable.Pane
                    defaultSize={50}
                    minSize={50}
                    class="min-h-0 min-w-0 flex-1 overflow-hidden"
                >
                    <div class="shrink-0 pb-3">
                        <div
                            class="flex items-center bg-card rounded-lg px-4 w-[50%]"
                        >
                            <Search class="h-4 w-4" />
                            <Input
                                bind:value={assignedInput}
                                placeholder={`Search assigned ${tab.label.toLowerCase()}...`}
                                class="border-none bg-transparent!"
                            />
                        </div>
                    </div>
                    <div
                        class="h-full min-h-0 overflow-y-auto scroll-pb-16 pb-16 pr-1"
                        onscroll={loadMoreRoles}
                    >
                        {#if visibleRoles.length === 0}
                            <div
                                class="rounded-lg border border-dashed px-4 py-12 text-center text-sm text-muted-foreground"
                            >
                                Select a {tab.label.toLowerCase().slice(0, -1)} to
                                assign access.
                            </div>
                        {:else if activeTab === "WORKSPACE" || activeTab === "CONNECTION"}
                            {#each assignedWorkspaceGroups as group (group.id)}
                                <p
                                    class="px-2! py-2! text-xs font-semibold uppercase tracking-wide text-muted-foreground"
                                >
                                    {group.name}
                                </p>
                                {#each group.options as option (option.id)}
                                    {@const role = roleForEntity(option.id)}
                                    {#if role}
                                        {@render roleCard(
                                            role,
                                            tab.label.slice(0, -1),
                                        )}
                                    {/if}
                                {/each}
                            {/each}
                        {:else}
                            {#each assignedConnectionGroups as group (group.id)}
                                {#each group.options as connection (connection.connectionId)}
                                    {@const connectionId =
                                        connection.connectionId ?? ""}
                                    <p
                                        class="px-2 py-1.5 text-xs font-semibold uppercase tracking-wide text-muted-foreground"
                                    >
                                        {group.name} / {connection.connectionName}
                                    </p>
                                    {#each rolesForConnection(connectionId) as role (role.id)}
                                        {@render roleCard(
                                            role,
                                            tab.label.slice(0, -1),
                                        )}
                                    {/each}
                                {/each}
                            {/each}
                        {/if}
                    </div>
                </Resizable.Pane>

                <Resizable.Handle class="bg-transparent" />

                <Resizable.Pane
                    defaultSize={30}
                    minSize={30}
                    class="flex min-h-0 min-w-0 flex-col overflow-hidden rounded-lg bg-card"
                >
                    <Command.Root
                        class="h-full min-h-0 max-h-full flex-1 rounded-lg bg-card p-2"
                    >
                        <div class="border-b mb-1">
                            <Command.Input
                                bind:value={input}
                                placeholder={`Search ${tab.label.toLowerCase()}...`}
                                class="px-4 py-2 "
                            />
                        </div>

                        {#if activeTab === "FOLDER" || activeTab === "QUEUE"}
                            <div class="flex shrink-0 flex-wrap gap-2 py-2">
                                {#each connectionGroups.flatMap((group) => group.options) as connection (connection.connectionId)}
                                    {@const connectionId =
                                        connection.connectionId ?? ""}
                                    <Button
                                        type="button"
                                        aria-pressed={selectedConnectionIds.includes(
                                            connectionId,
                                        )}
                                        onclick={() =>
                                            toggleConnection(connectionId)}
                                        class={`h-7 rounded-sm border px-3! py-1! text-xs transition-colors ${selectedConnectionIds.includes(connectionId) ? "border-primary bg-primary text-primary-foreground" : "bg-transparent text-muted-foreground hover:bg-accent"}`}
                                    >
                                        {connection.connectionName}
                                    </Button>
                                {/each}
                            </div>
                        {/if}

                        <Command.List
                            class="min-h-0 max-h-none flex-1 overflow-y-auto scroll-pb-6 pb-6"
                            onscroll={loadMoreOptions}
                        >
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
                            {:else if activeTab === "WORKSPACE"}
                                {#each workspaceGroups as group (group.id)}
                                    <div class="mb-3 last:mb-0">
                                        <p
                                            class="px-3 py-1.5 text-xs font-semibold uppercase tracking-wide text-muted-foreground"
                                        >
                                            {group.name}
                                        </p>
                                        {#each group.options as option (option.id)}
                                            <label
                                                class="flex cursor-pointer items-center gap-3 rounded-md px-3 py-2.5 transition-colors hover:bg-muted/60"
                                            >
                                                <Checkbox
                                                    checked={isAssigned(
                                                        option.id,
                                                    )}
                                                    onCheckedChange={() =>
                                                        toggleAssignment(
                                                            option,
                                                        )}
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
                                    </div>
                                {/each}
                            {:else if activeTab === "CONNECTION"}
                                {#each workspaceGroups as group (group.id)}
                                    <Command.Group heading={group.name}>
                                        {#each group.options as option (option.id)}
                                            <Command.Item
                                                value={`${group.name} ${option.name}`}
                                                onSelect={() =>
                                                    toggleAssignment(option)}
                                                class="px-3 py-2.5 hover:bg-muted/60! [&_svg:not([class*='text-'])]:text-black aria-selected:bg-muted/60 aria-selected:text-white"
                                            >
                                                <Checkbox
                                                    checked={isAssigned(
                                                        option.id,
                                                    )}
                                                />
                                                <span
                                                    class="flex size-8 shrink-0 items-center justify-center rounded-full bg-primary/10 text-xs font-semibold text-primary"
                                                >
                                                    {option.name
                                                        .substring(0, 1)
                                                        .toUpperCase()}
                                                </span>
                                                <span
                                                    class="truncate text-sm font-medium"
                                                >
                                                    {option.name}
                                                </span>
                                            </Command.Item>
                                        {/each}
                                    </Command.Group>
                                {/each}
                            {:else}
                                {#each visibleConnectionGroups as group (group.id)}
                                    {#each group.options as connection (connection.connectionId)}
                                        {@const connectionId =
                                            connection.connectionId ?? ""}
                                        <Command.Group
                                            heading={`${group.name} / ${connection.connectionName}`}
                                        >
                                            {#each matchingOptions.filter((option) => option.connectionId === connectionId) as option (option.id)}
                                                <Command.Item
                                                    value={`${group.name} ${connection.connectionName} ${option.name}`}
                                                    onSelect={() => {
                                                        if (
                                                            !option.isInherited
                                                        ) {
                                                            toggleAssignment(
                                                                option,
                                                            );
                                                        }
                                                    }}
                                                    aria-disabled={option.isInherited}
                                                    class="px-3 py-2.5 [&_svg:not([class*='text-'])]:text-black aria-selected:bg-muted/60  aria-selected:text-white"
                                                >
                                                    <Checkbox
                                                        checked={isAssigned(
                                                            option.id,
                                                        ) || option.isInherited}
                                                        disabled={option.isInherited}
                                                    />
                                                    <span
                                                        class="flex size-8 shrink-0 items-center justify-center rounded-full bg-primary/10 text-xs font-semibold text-primary"
                                                    >
                                                        {option.name
                                                            .substring(0, 1)
                                                            .toUpperCase()}
                                                    </span>
                                                    <span
                                                        class="truncate text-sm font-medium"
                                                    >
                                                        {option.name}
                                                    </span>
                                                    {#if option.isInherited}
                                                        <span
                                                            class="ml-auto text-xs text-muted-foreground"
                                                        >
                                                            Inherited ({option.inheritedRole?.toLowerCase()})
                                                        </span>
                                                    {/if}
                                                </Command.Item>
                                            {/each}
                                        </Command.Group>
                                    {/each}
                                {/each}
                            {/if}
                        </Command.List>
                    </Command.Root>
                </Resizable.Pane>
            </Resizable.PaneGroup>
        </Tabs.Content>
    {/each}
</Tabs.Root>
