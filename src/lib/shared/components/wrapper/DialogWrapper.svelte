<script lang="ts">
    import { cn } from "shared/utils";
    import type { Snippet } from "svelte";
    import * as Dialog from "ui/dialog";

    interface Props {
        title: string;
        description: string;
        trigger?: Snippet;
        children: Snippet<[closeDialog: () => void]>;
        triggerClass?: string;
        contentClass?: string;
        bodyClass?: string;
        showTrigger?: boolean;
        open?: boolean;
    }

    let {
        title,
        description,
        trigger,
        children,
        triggerClass,
        contentClass,
        bodyClass,
        showTrigger = true,
        open = $bindable(false),
    }: Props = $props();

    function handleClose() {
        open = false;
    }
</script>

<Dialog.Root bind:open>
    {#if showTrigger && trigger}
        <Dialog.Trigger
            class={cn(
                "p-1 py-2 rounded-sm",
                "cursor-pointer hover:bg-card hover:text-primary",
                triggerClass,
            )}
        >
            {@render trigger()}
        </Dialog.Trigger>
    {/if}

    <Dialog.Content
        class={cn(
            "max-h[80vh]",
            "sm:max-w-[80vw] md:max-w-[60vw] lg:max-w-[40vw]",
            "flex min-h-0 flex-col overflow-hidden bg-card",
            contentClass,
        )}
    >
        <Dialog.Header>
            <Dialog.Title>{title}</Dialog.Title>
            <Dialog.Description>{description}</Dialog.Description>
        </Dialog.Header>

        <div class={cn("min-h-0 rounded-lg bg-background", bodyClass)}>
            {@render children(handleClose)}
        </div>
    </Dialog.Content>
</Dialog.Root>
