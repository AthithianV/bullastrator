<script lang="ts">
    import { ChevronDown, LoaderCircle } from "@lucide/svelte";
    import type { Component, Snippet } from "svelte";
    import { cn } from "tailwind-variants";
    import * as ButtonGroup from "ui/button-group/index";
    import Button from "ui/button/button.svelte";
    import * as DropdownMenu from "ui/dropdown-menu/index";
    import TooltipContent from "ui/tooltip/tooltip-content.svelte";
    import TooltipTrigger from "ui/tooltip/tooltip-trigger.svelte";
    import Tooltip from "ui/tooltip/tooltip.svelte";

    let {
        onclick,
        isLoading = false,
        isDisabled = false,
        isDropdownDisabled = false,
        icon: Icon,
        variant = "outline",
        class: className = "",
        title = "",
        shortcut,
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
        shortcut?: string;
        menuContent?: Snippet;
    } = $props();
</script>

<Tooltip>
    <TooltipTrigger>
        <span>
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
                        " hover:bg-card!",
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
                        <DropdownMenu.Trigger
                            disabled={isDropdownDisabled || isLoading}
                        >
                            <Button
                                size="icon"
                                variant={"ghost"}
                                class={cn(
                                    "py-2 px-2! cursor-pointer rounded-sm border",
                                    "rounded-s-none px-2 border",
                                    "p-1 py-2",
                                    "cursor-pointer hover:bg-card!",
                                    className,
                                )}
                                disabled={isDropdownDisabled}
                            >
                                <ChevronDown class="h-4 w-4" />
                            </Button>
                        </DropdownMenu.Trigger>
                        <DropdownMenu.Content align="start">
                            {@render menuContent()}
                        </DropdownMenu.Content>
                    </DropdownMenu.Root>
                {/if}
            </ButtonGroup.Root>
        </span>
    </TooltipTrigger>
    <TooltipContent>
        {title}
        {#if shortcut}
            <kbd
                class="ml-2 rounded border border-border/50 px-1.5 py-0.5 font-mono text-[10px]"
            >
                {shortcut}
            </kbd>
        {/if}
    </TooltipContent>
</Tooltip>
