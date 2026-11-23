<script lang="ts">
    import * as AlertDialog from "ui/alert-dialog";
    import {
        confirmationRequest,
        resolveConfirmation,
    } from "shared/helpers/confirmAction";
    import Input from "components/ui/input/input.svelte";

    let open = $state(false);
    let confirmationInput = $state("");

    $effect(() => {
        open = $confirmationRequest !== null;
        if ($confirmationRequest) confirmationInput = "";
    });

    const canConfirm = $derived(
        !$confirmationRequest?.requiredText ||
            confirmationInput === $confirmationRequest.requiredText,
    );

    function resolve(confirmed: boolean) {
        open = false;
        resolveConfirmation(confirmed);
    }
</script>

<AlertDialog.Root
    bind:open
    onOpenChange={(value) => {
        if (!value && $confirmationRequest) resolve(false);
    }}
>
    <AlertDialog.Content class="bg-card">
        <AlertDialog.Header>
            <AlertDialog.Title>
                {$confirmationRequest?.title ?? "Confirm action"}
            </AlertDialog.Title>
            <AlertDialog.Description>
                {$confirmationRequest?.message ?? "Are you sure?"}
            </AlertDialog.Description>
        </AlertDialog.Header>
        {#if $confirmationRequest?.requiredText}
            <div class="space-y-2">
                <p class="text-sm text-muted-foreground">
                    Type
                    <code class="rounded bg-muted px-1.5 py-0.5 font-semibold text-foreground">
                        {$confirmationRequest.requiredText}
                    </code>
                    to continue.
                </p>
                <Input
                    bind:value={confirmationInput}
                    placeholder={$confirmationRequest.requiredText}
                    autocomplete="off"
                />
            </div>
        {/if}
        <AlertDialog.Footer>
            <AlertDialog.Cancel onclick={() => resolve(false)}>
                Cancel
            </AlertDialog.Cancel>
            <AlertDialog.Action
                disabled={!canConfirm}
                class="bg-destructive text-white hover:bg-destructive/90"
                onclick={() => resolve(true)}
            >
                Continue
            </AlertDialog.Action>
        </AlertDialog.Footer>
    </AlertDialog.Content>
</AlertDialog.Root>
