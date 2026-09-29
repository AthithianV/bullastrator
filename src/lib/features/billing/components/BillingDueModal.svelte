<script lang="ts">
    import * as Dialog from "ui/dialog";
    import { Button } from "ui/button";
    import { AlertTriangle, ExternalLink } from "lucide-svelte";
    import type { BillingSubscription } from "$lib/shared/interfaces/billing.types";

    let {
        subscription = $bindable<BillingSubscription | null>(null),
        open = $bindable(false),
    } = $props<{
        subscription: BillingSubscription | null;
        open: boolean;
    }>();

    const handleManageBilling = () => {
        window.open(
            `${import.meta.env.VITE_QHOUND_FRONTEND_BASE_URL ?? "https://bullastrator.dev"}/settings/billing`,
            "_blank",
        );
        open = false;
    };
</script>

<Dialog.Root bind:open>
    <Dialog.Content class="sm:max-w-[30vw]">
        <Dialog.Header>
            <div class="flex items-center gap-2 text-destructive mb-2">
                <AlertTriangle size={24} />
                <Dialog.Title>Subscription Action Required</Dialog.Title>
            </div>
            <Dialog.Description>
                Your <strong
                    >{subscription?.plan
                        .replace("bullastrator_", "")
                        .toUpperCase()}</strong
                >
                subscription is currently
                <strong>{subscription?.status}</strong>. Please complete your
                payment to continue using Pro features.
            </Dialog.Description>
        </Dialog.Header>

        <div class="py-4">
            <div class="bg-muted p-3 rounded-md text-sm space-y-1">
                <div class="flex justify-between">
                    <span class="text-muted-foreground">Status:</span>
                    <span class="font-medium capitalize"
                        >{subscription?.status}</span
                    >
                </div>
                <div class="flex justify-between">
                    <span class="text-muted-foreground">Period End:</span>
                    <span class="font-medium">
                        {subscription?.currentPeriodEnd
                            ? new Date(
                                  subscription.currentPeriodEnd,
                              ).toLocaleDateString()
                            : "N/A"}
                    </span>
                </div>
            </div>
        </div>

        <Dialog.Footer>
            <Button variant="outline" onclick={() => (open = false)}
                >Close</Button
            >
            <Button onclick={handleManageBilling} class="gap-2">
                Manage Billing
                <ExternalLink size={14} />
            </Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>
