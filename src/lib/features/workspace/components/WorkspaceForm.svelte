<script lang="ts">
    import { Loader } from "@lucide/svelte";
    import { defaults, superForm } from "sveltekit-superforms";
    import { zod4 } from "sveltekit-superforms/adapters";
    import { Button } from "ui/button/index.js";
    import * as Field from "ui/field/index.js";
    import { Input } from "ui/input/index.js";
    import {
        useCreateWorkspace,
        useUpdateWorkspace,
    } from "workspace/hooks/workspace.hooks";
    import { workspaceSchema } from "../schema/workspace.schema";

    // Props using Svelte 5 runes
    let { data = null, onComplete } = $props<{
        data?: any;
        onComplete?: () => void;
    }>();

    const { mutateAsync: createWorkspace } = useCreateWorkspace();
    const { mutateAsync: updateWorkspace } = useUpdateWorkspace();

    // Initialize form with defaults or existing data
    const initialData = defaults((() => data)(), zod4(workspaceSchema));

    const { form, errors, enhance, submitting } = superForm(initialData, {
        id: "add-or-update-workspace",
        validators: zod4(workspaceSchema),
        SPA: true,
        dataType: "json",
        async onUpdate({ form: f }) {
            if (f.valid) {
                try {
                    if (data?.id) {
                        await updateWorkspace({
                            id: data.id,
                            data: { ...f.data, lastAccessedAt: new Date() },
                        });
                    } else {
                        await createWorkspace({
                            ...f.data,
                            plan: "FREE",
                            role: "OWNER",
                            lastAccessedAt: new Date(),
                            maxConnections: 10,
                        });
                    }
                    onComplete?.();
                } catch (err) {
                    console.error("Workspace Form Error:", err);
                }
            }
        },
    });
</script>

<div class="max-h-[80vh] overflow-auto border rounded-lg p-6 shadow-sm">
    <form use:enhance class="space-y-6">
        <Field.Set>
            <Field.Group>
                <Field.Field data-invalid={$errors.name ? "" : undefined}>
                    <Field.Label for="name">Workspace Name</Field.Label>
                    <Input
                        id="name"
                        bind:value={$form.name}
                        placeholder="e.g. Personal Projects"
                        aria-invalid={$errors.name ? "true" : undefined}
                    />
                    {#if $errors.name}<Field.Error>{$errors.name}</Field.Error
                        >{/if}
                </Field.Field>

                <div class="grid grid-cols-2 gap-4">
                    <Field.Field data-invalid={$errors.icon ? "" : undefined}>
                        <Field.Label for="icon">Icon / Emoji</Field.Label>
                        <Input
                            id="icon"
                            bind:value={$form.icon}
                            placeholder="🚀"
                        />
                        {#if $errors.icon}<Field.Error
                                >{$errors.icon}</Field.Error
                            >{/if}
                    </Field.Field>

                    <Field.Field data-invalid={$errors.color ? "" : undefined}>
                        <Field.Label for="color">Theme Color</Field.Label>
                        <div class="flex gap-2">
                            <Input
                                id="color"
                                type="color"
                                bind:value={$form.color}
                                class="w-12 p-1 h-10"
                            />
                            <Input
                                bind:value={$form.color}
                                placeholder="#00CADB"
                                class="font-mono"
                            />
                        </div>
                        {#if $errors.color}<Field.Error
                                >{$errors.color}</Field.Error
                            >{/if}
                    </Field.Field>
                </div>

                <Field.Separator class="my-4" />

                <!-- <div class="flex items-center justify-between rounded-lg border p-4">
          <div class="space-y-0.5">
            <Field.Label class="text-base">Default Workspace</Field.Label>
            <p class="text-sm text-muted-foreground">
              Automatically open this workspace on startup.
            </p>
          </div>
          {#if $form.is_default}
            <Switch bind:checked={$form.is_default} />
          {/if}
        </div> -->

                <div class="flex justify-end gap-3 pt-6">
                    <Button
                        type="submit"
                        disabled={$submitting}
                        class="min-w-25"
                    >
                        {#if $submitting}
                            <Loader class="mr-2 h-4 w-4 animate-spin" />
                        {/if}
                        {data ? "Update Workspace" : "Create Workspace"}
                    </Button>
                </div>
            </Field.Group>
        </Field.Set>
    </form>
</div>
