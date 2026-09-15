<script lang="ts">
    import { ChevronDown, LoaderCircle } from "@lucide/svelte";
    import type { Component, Snippet } from "svelte";
    import { cn } from "tailwind-variants";
    import * as ButtonGroup from "ui/button-group/index";
    import Button from "ui/button/button.svelte";
    import * as DropdownMenu from "ui/dropdown-menu/index";

    let {
        onclick,
        isLoading = false,
        isDisabled = false,
        isDropdownDisabled = false,
        icon: Icon,
        variant = "outline",
        class: className = "",
        title = "",
        menuContent,
    }: {
        onclick: () => void;
        isLoading?: boolean;
        isDisabled?: boolean;
        isDropdownDisabled?: boolean;
        icon: Component;
        variant?:
            | "default"
            | "destructive"
            | "outline"
            | "secondary"
            | "ghost"
            | "link";
        class?: string;
        title?: string;
        menuContent?: Snippet;
    } = $props();
</script>

<ButtonGroup.Root>
    <Button
        variant={"ghost"}
        {onclick}
        disabled={isDisabled || isLoading}
        {title}
        class={cn(
            {
                "rounded-e-none border-r-0": !!menuContent,
            },
            "py-2 px-2! cursor-pointer rounded-sm border",
            "shadow",
            " hover:bg-slate-800!",
            className,
        )}
    >
        {#if isLoading}
            <LoaderCircle class="h-4 w-4 animate-spin" />
        {:else}
            <Icon class="h-4 w-4" />
        {/if}
    </Button>

    {#if menuContent}
        <DropdownMenu.Root>
            <DropdownMenu.Trigger disabled={isDropdownDisabled || isLoading}>
                <Button
                    size="icon"
                    variant={"ghost"}
                    class={cn(
                        "py-2 px-2! cursor-pointer rounded-sm border",
                        "rounded-s-none px-2 border",
                        "p-1 py-2",
                        "cursor-pointer hover:bg-slate-800!",
                        className,
                    )}
                    disabled={isDropdownDisabled}
                >
                    <ChevronDown class="h-4 w-4" />
                </Button>
            </DropdownMenu.Trigger>
            <DropdownMenu.Content align="end" class="w-56">
                {@render menuContent()}
            </DropdownMenu.Content>
        </DropdownMenu.Root>
    {/if}
</ButtonGroup.Root>
