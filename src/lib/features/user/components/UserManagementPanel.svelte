<script lang="ts">
    import { UserRound, Users, X } from "@lucide/svelte";
    import * as Avatar from "shared/components/ui/avatar";
    import * as Resizable from "shared/components/ui/resizable";
    import Separator from "shared/components/ui/separator/separator.svelte";
    import Button from "components/ui/button/button.svelte";
    import DialogWrapper from "components/wrapper/DialogWrapper.svelte";
    import { useGetAllUsers } from "../hooks/user.hooks";

    import { cn } from "shared/utils";
    import RoleManagement from "./RoleManagement.svelte";

    let open = $state(false),
        selectedUserId = $state<string | null>(null);

    const usersQuery = useGetAllUsers(() => open);
    const users = $derived(usersQuery.data ?? []);

    $effect(() => {
        if (
            users.length > 0 &&
            (!selectedUserId ||
                !users.some((user) => user.id === selectedUserId))
        )
            selectedUserId = users[0].id;
        if (users.length === 0) selectedUserId = null;
    });

    const selectedUser = $derived(
        users.find((user) => user.id === selectedUserId) ?? null,
    );
</script>

<DialogWrapper
    bind:open
    title="User management"
    description="Manage scoped roles for every user"
    contentClass="sm:max-w-[95vw] md:max-w-[90vw] lg:max-w-[80vw] h-[90vh] p-4!"
    bodyClass="flex flex-1 min-h-0 flex-col overflow-hidden bg-card"
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
        <Resizable.PaneGroup
            direction="horizontal"
            class="min-h-0 flex-1 gap-0.5 border-none"
        >
            <Resizable.Pane
                defaultSize={25}
                minSize={20}
                maxSize={50}
                class="min-w-0 overflow-y-auto rounded-lg bg-background px-2"
                ><div
                    class="py-2 text-xs font-semibold uppercase tracking-wide text-muted-foreground"
                >
                    Users
                </div>
                {#each users as user (user.id)}
                    <Button
                        type="button"
                        class={cn(
                            "justify-start w-full items-center gap-3 p-2! h-fit! text-left bg-transparent hover:bg-transparent shadow-none",
                            selectedUserId === user.id &&
                                "bg-card hover:bg-card!",
                        )}
                        variant="ghost"
                        onclick={() => (selectedUserId = user.id)}
                    >
                        <Avatar.Root class="size-8 bg-primary">
                            <Avatar.Fallback class="bg-primary text-black">
                                {user.name.slice(0, 1)}
                            </Avatar.Fallback>
                        </Avatar.Root>
                        <span class="min-w-0">
                            <span
                                class="block truncate text-sm font-medium dark:text-white text-gray-900"
                            >
                                {user.name}
                            </span>
                            <span
                                class="block truncate text-xs text-muted-foreground"
                            >
                                {user.email}
                            </span>
                        </span>
                    </Button>
                {/each}
            </Resizable.Pane>
            <Resizable.Handle class="bg-card" />
            <Resizable.Pane
                defaultSize={65}
                minSize={50}
                class="min-w-0 overflow-y-auto rounded-lg bg-background"
            >
                {#if selectedUser}
                    <div class="flex items-center gap-3 px-6 py-5">
                        <Avatar.Root class="size-11">
                            <Avatar.Fallback>
                                {selectedUser.name.slice(0, 1)}
                            </Avatar.Fallback>
                        </Avatar.Root>
                        <div>
                            <h2 class="font-semibold">
                                {selectedUser.name}
                            </h2>
                            <p class="text-sm text-muted-foreground">
                                {selectedUser.email}
                            </p>
                        </div>
                    </div>
                    <Separator />

                    {#if selectedUserId}
                        <RoleManagement {selectedUserId} />
                    {/if}
                {:else}
                    <div
                        class="flex flex-1 flex-col items-center justify-center px-8 text-center text-muted-foreground"
                    >
                        <Users class="mb-3 size-10 opacity-40" />
                        <p class="font-medium">
                            Select a user to manage access
                        </p>
                    </div>
                {/if}
            </Resizable.Pane>
        </Resizable.PaneGroup>
    {/snippet}
</DialogWrapper>
