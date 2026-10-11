<script lang="ts">
    import * as AlertDialog from "ui/alert-dialog";
    import {
        confirmationRequest,
        resolveConfirmation,
    } from "shared/helpers/confirmAction";

    let open = $state(false);

    $effect(() => {
        open = $confirmationRequest !== null;
    });

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
    <AlertDialog.Content>
        <AlertDialog.Header>
            <AlertDialog.Title>
                {$confirmationRequest?.title ?? "Confirm action"}
            </AlertDialog.Title>
            <AlertDialog.Description>
                {$confirmationRequest?.message ?? "Are you sure?"}
            </AlertDialog.Description>
        </AlertDialog.Header>
        <AlertDialog.Footer>
            <AlertDialog.Cancel onclick={() => resolve(false)}>
                Cancel
            </AlertDialog.Cancel>
            <AlertDialog.Action
                class="bg-destructive text-white hover:bg-destructive/90"
                onclick={() => resolve(true)}
            >
                Continue
            </AlertDialog.Action>
        </AlertDialog.Footer>
    </AlertDialog.Content>
</AlertDialog.Root>
