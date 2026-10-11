<script lang="ts">
    import { cn } from "shared/utils";
    import type { Snippet } from "svelte";
    import * as Dialog from "ui/dialog";

    interface Props {
        title: string;
        description: string;
        trigger: Snippet;
        children: Snippet<[closeDialog: () => void]>;
        triggerClass?: string;
        contentClass?: string;
    }

    let {
        title,
        description,
        trigger,
        children,
        triggerClass,
        contentClass,
    }: Props = $props();

    // Internal state to control the dialog visibility
    let open = $state(false);

    function handleClose() {
        open = false;
    }
</script>

<Dialog.Root bind:open>
    <Dialog.Trigger
        class={cn(
            "p-1 py-2 rounded-sm",
            "cursor-pointer hover:bg-slate-800 hover:text-primary",
            triggerClass,
        )}
    >
        {@render trigger()}
    </Dialog.Trigger>

    <Dialog.Content
        class={cn(
            "max-h[80vh]",
            "sm:max-w-[80vw] md:max-w-[60vw] lg:max-w-[40vw]",
            "overflow-auto bg-card",
            contentClass,
        )}
    >
        <Dialog.Header>
            <Dialog.Title>{title}</Dialog.Title>
            <Dialog.Description>{description}</Dialog.Description>
        </Dialog.Header>

        <div class="bg-background border rounded-lg">
            {@render children(handleClose)}
        </div>
    </Dialog.Content>
</Dialog.Root>
