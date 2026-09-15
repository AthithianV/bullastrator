<script lang="ts">
  import * as Alert from "ui/alert";
  import { Button } from "ui/button";
  import { Checkbox } from "ui/checkbox";
  import * as Field from "ui/field";
  import { Input } from "ui/input";

  import { defaults, superForm } from "sveltekit-superforms";
  import { zod4 } from "sveltekit-superforms/adapters";
  import { connectionSchema } from "../schema/connection.schema";

  import { CircleAlert } from "@lucide/svelte";
  import { confirm } from "@tauri-apps/plugin-dialog";
  import {
    useCreateConnection,
    useManualConnectionsHealth,
    useUpdateConnection,
  } from "connection/hooks/connection.hooks";
  import type { ReadConnection } from "connection/interface/connection.types";
  import { useSyncAllQueues } from "queue/hooks/queue.hooks";
  import { queueStore } from "queue/store/queueStore.svelte";
  import ColorPicker from "shared/components/form/ColorPicker.svelte";
  import InfiniteScanLoader from "shared/components/loaders/InfiniteScanLoader.svelte";

  let { data = null, onComplete } = $props<{
    data?: ReadConnection | null;
    onComplete: () => void;
  }>();
  let errorTitle = $state<string | null>(null);
  let errorDescription = $state<string | null>(null);
  const { mutateAsync: createConnection } = useCreateConnection();
  const { mutateAsync: updateConnection } = useUpdateConnection();
  const { mutateAsync: checkHealth } = useManualConnectionsHealth();
  const syncQuery = useSyncAllQueues();

  const initialData = defaults(
    (() => data)() ?? zod4(connectionSchema),
    zod4(connectionSchema),
  );

  const { form, errors, enhance, submitting } = superForm(initialData, {
    id: "add-or-update-connection",
    validators: zod4(connectionSchema),
    SPA: true,
    dataType: "json",
    invalidateAll: false,
    async onUpdate({ form: f }) {
      if (!f.valid) return;
      try {
        let connection;
        if (data) {
          connection = await updateConnection({ id: data.id, data: f.data });
        } else {
          connection = await createConnection(f.data);
        }

        if (onComplete) {
          onComplete();
        }

        const confirmation = await confirm(
          `Do you want to sync all queues in the Connection?`,
          { title: "Sync all Queues", kind: "info" },
        );

        if (confirmation) {
          syncQuery.mutateAsync(connection.id);
          queueStore.isSyncing = true;
          queueStore.connectionOnSync = connection.id;
        }
      } catch (error: any) {
        const msg = error.message || "";
        if (msg.includes("Connection timed out")) {
          errorTitle = "Connection Timeout";
          errorDescription =
            "Check your Host/Port. If you're using a local Redis, ensure 'TLS Enabled' is turned OFF.";
        } else if (msg.includes("failed to lookup address")) {
          errorTitle = "DNS Error";
          errorDescription = "Could not resolve the Host address.";
        } else if (msg.includes("invalid password")) {
          errorTitle = "Authentication Failed";
          errorDescription = "Please check your Redis password.";
        } else if (msg.includes("unsupported")) {
          errorTitle = "Unsupported Version";
          errorDescription = msg;
        } else {
          errorTitle = "Unknow Error";
          errorDescription = "Something went wrong";
        }
      }
    },
  });

  // Add this variable to track if the user manually changed the label
  let isManualLabel = $state(false);

  function handleNameInput(e: Event) {
    const input = e.target as HTMLInputElement;
    const name = input.value;

    // Only auto-generate if it's a new connection and user hasn't typed a manual label
    if (!data && !isManualLabel && name) {
      const words = name.trim().split(/\s+/);
      const generated =
        words.length >= 2 ? words[0][0] + words[1][0] : words[0].slice(0, 2);

      $form.label = generated.toUpperCase();
    }
  }
</script>

<div class="max-h-[80vh] overflow-auto p-6 relative">
  <InfiniteScanLoader loading={$submitting} />

  <Alert.Root variant="default" class="my-4 bg-yellow-400/10">
    <Alert.Title class="flex gap-2 items-center text-xs  text-yellow-400"
      ><CircleAlert size={12} />Only BullMQ v5 is supported for now!</Alert.Title
    >
  </Alert.Root>

  {#if errorTitle || errorDescription}
    <Alert.Root variant="destructive" class="my-2">
      <Alert.Title class="flex gap-2 items-center my-2"
        ><CircleAlert size={20} /> {errorTitle}</Alert.Title
      >
      <Alert.Description>
        {errorDescription}
      </Alert.Description>
    </Alert.Root>
  {/if}

  <form use:enhance>
    <Field.Set>
      <Field.Group>
        <Field.Field data-invalid={$errors.name ? "" : undefined}>
          <Field.Label for="name">Connection Name</Field.Label>
          <Input
            id="name"
            bind:value={$form.name}
            placeholder="Production Redis"
            aria-invalid={$errors.name ? "true" : undefined}
            oninput={handleNameInput}
          />
          {#if $errors.name}<Field.Error>{$errors.name}</Field.Error>{/if}
        </Field.Field>

        <div class="grid grid-cols-3 gap-4">
          <Field.Field
            class="col-span-2"
            data-invalid={$errors.host ? "" : undefined}
          >
            <Field.Label for="host">Host</Field.Label>
            <Input id="host" bind:value={$form.host} placeholder="127.0.0.1" />
            {#if $errors.host}<Field.Error>{$errors.host}</Field.Error>{/if}
          </Field.Field>

          <Field.Field data-invalid={$errors.port ? "" : undefined}>
            <Field.Label for="port">Port</Field.Label>
            <Input id="port" type="number" bind:value={$form.port} />
            {#if $errors.port}<Field.Error>{$errors.port}</Field.Error>{/if}
          </Field.Field>
        </div>
        <div class="grid grid-cols-2 gap-4">
          <Field.Field>
            <Field.Label for="username">Username (Optional)</Field.Label>
            <Input id="username" bind:value={$form.username} />
          </Field.Field>

          <Field.Field>
            <Field.Label for="password">Password (Optional)</Field.Label>
            <Input id="password" type="password" bind:value={$form.password} />
          </Field.Field>
        </div>

        <Field.Separator />
        <div>
          <Field.Field data-invalid={$errors.isTlsEnabled ? "" : undefined}>
            <div class="flex items-center gap-2">
              <Checkbox
                id="is_tls_enabled"
                bind:checked={$form.isTlsEnabled}
                class="w-4 self-start"
              />
              <Field.Label for="bullmq_prefix">TLS Enabled</Field.Label>
            </div>
            {#if $errors.isTlsEnabled}<Field.Error
                >{$errors.isTlsEnabled}</Field.Error
              >{/if}
          </Field.Field>
        </div>

        <Field.Separator />
        <div class="grid grid-cols-3 gap-4">
          <Field.Field data-invalid={$errors.db ? "" : undefined}>
            <Field.Label for="db">DB Index</Field.Label>
            <Input id="db" type="number" bind:value={$form.db} />
            {#if $errors.db}<Field.Error>{$errors.db}</Field.Error>{/if}
          </Field.Field>

          <Field.Field data-invalid={$errors.bullmqPrefix ? "" : undefined}>
            <Field.Label for="bullmq_prefix">BullMQ Prefix</Field.Label>
            <Input id="bullmq_prefix" bind:value={$form.bullmqPrefix} />
            {#if $errors.bullmqPrefix}<Field.Error
                >{$errors.bullmqPrefix}</Field.Error
              >{/if}
          </Field.Field>
        </div>

        <Field.Separator />
        <div class="grid grid-cols-2 gap-4">
          <Field.Field data-invalid={$errors.label ? "" : undefined}>
            <Field.Label for="label">Label</Field.Label>
            <Input
              id="label"
              bind:value={$form.label}
              maxlength={3}
              placeholder="PR"
            />
          </Field.Field>

          <Field.Field>
            <Field.Label>Color</Field.Label>
            <div class="flex h-10 items-center gap-2">
              <ColorPicker bind:data={$form.color} />
            </div>
          </Field.Field>
        </div>

        <div class="flex justify-end gap-3 pt-4">
          <Button
            variant="outline"
            type="button"
            disabled={$submitting}
            onclick={() => checkHealth($form)}>Test Connection</Button
          >
          <Button disabled={$submitting} type="submit"
            >{data ? "Update" : "Save"}</Button
          >
        </div>
      </Field.Group>
    </Field.Set>
  </form>
</div>
