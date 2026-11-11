<script lang="ts">
    import { Trash } from "@lucide/svelte";
    import * as Tabs from "shared/components/ui/tabs";
    import {
        useCreateUserRole,
        useDeleteUserRole,
        useGetRoleOptions,
        useGetUserRoles,
    } from "../hooks/user.roles.hooks";
    import type { RoleEntityOption } from "../interface";
    import Button from "components/ui/button/button.svelte";
    import Input from "components/ui/input/input.svelte";
    import Badge from "components/ui/badge/badge.svelte";
    import * as Command from "components/ui/command";
    import * as Popover from "components/ui/popover";

    import ChevronDownIcon from "@lucide/svelte/icons/chevron-down";
    import { tick } from "svelte";

    const frameworks = ["Next.js", "SvelteKit", "Nuxt.js", "Remix", "Astro"];

    let open = $state(false);
    let value = $state("");
    let triggerRef = $state<HTMLButtonElement>(null!);

    // We want to refocus the trigger button when the user selects
    // an item from the list so users can continue navigating the
    // rest of the form with the keyboard.
    function closeAndFocusTrigger() {
        open = false;
        tick().then(() => {
            triggerRef.focus();
        });
    }

    type AccessTab = "WORKSPACE" | "CONNECTION" | "FOLDER" | "QUEUE";
    const tabs: Array<{ value: AccessTab; label: string }> = [
        { value: "WORKSPACE", label: "Workspaces" },
        { value: "CONNECTION", label: "Connections" },
        { value: "FOLDER", label: "Folders" },
        { value: "QUEUE", label: "Queues" },
    ];

    let { selectedUserId }: { selectedUserId: string } = $props();

    let activeTab = $state<AccessTab>("WORKSPACE");
    let input = $state("");

    const rolesQuery = useGetUserRoles(() => selectedUserId);
    const optionsQuery = useGetRoleOptions(() => activeTab);
    const createRole = useCreateUserRole(() => selectedUserId);
    const deleteRole = useDeleteUserRole(() => selectedUserId);

    const matchingOptions = $derived(
        (optionsQuery.data ?? []).filter((option) =>
            option.name.toLowerCase().includes(input.toLowerCase()),
        ),
    );

    const visibleRoles = $derived(
        (rolesQuery.data ?? []).filter((role) => role.entityType === activeTab),
    );

    $inspect(visibleRoles);

    function assignRole(entity: RoleEntityOption) {
        if (!selectedUserId) return;

        createRole.mutate({
            userId: selectedUserId,
            entityType: activeTab,
            entityId: entity.id,
            role: "VIEWER",
        });
        input = "";
    }
</script>

<Tabs.Root bind:value={activeTab} class="gap-0">
    <Tabs.List class="mx-6 mt-4 grid h-auto w-auto grid-cols-4 rounded-md">
        {#each tabs as tab}
            <Tabs.Trigger value={tab.value}>
                {tab.label}
            </Tabs.Trigger>
        {/each}
    </Tabs.List>
    {#each tabs as tab}
        <Tabs.Content value={tab.value} class="px-6 py-5">
            <div class="relative">
                <div
                    role="combobox"
                    aria-haspopup="listbox"
                    aria-controls={`role-options-${tab.value.toLowerCase()}`}
                    aria-expanded={input.length > 0 &&
                        matchingOptions.length > 0}
                >
                    <div class="flex min-h-20 items-start gap-2">
                        <div class="mt-4 flex flex-wrap gap-2">
                            {#each visibleRoles as role (role.id)}
                                <Badge
                                    class="inline-flex items-center gap-2 rounded-lg border-primary bg-transparent px-4 py-0! text-xs h-fit!"
                                    variant="secondary"
                                >
                                    <span
                                        class="flex gap-2 justify-start items-center"
                                    >
                                        <span class="font-medium text-base">
                                            {role.entityName}
                                        </span>
                                        <span
                                            class="text-muted-foreground text-xs rounded"
                                        >
                                            {role.role}
                                        </span>
                                    </span>
                                    <Button
                                        type="button"
                                        variant="ghost"
                                        class="p-0! h-1! hover:text-red-400"
                                        aria-label={`Remove ${role.entityName} role`}
                                        onclick={() =>
                                            deleteRole.mutate(role.id)}
                                    >
                                        <Trash class="size-4" />
                                    </Button>
                                </Badge>
                            {/each}
                            <Command.Root class="bg-transparent w-50">
                                <Command.Input bind:value={input} />
                                <Command.List class="bg-card">
                                    {#if input && matchingOptions.length > 0}
                                        <Command.Group>
                                            {#each matchingOptions as option (option.id)}
                                                <Command.Item
                                                    value={option.name}
                                                    data-checked={value ===
                                                        option.name}
                                                    onSelect={() => {
                                                        assignRole(option);
                                                    }}
                                                >
                                                    {option.name}
                                                </Command.Item>
                                            {/each}
                                        </Command.Group>
                                    {/if}
                                </Command.List>
                            </Command.Root>
                        </div>
                    </div>
                </div>
            </div>
        </Tabs.Content>
    {/each}
</Tabs.Root>
