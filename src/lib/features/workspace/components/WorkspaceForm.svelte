<script lang="ts">
    import { Loader } from "@lucide/svelte";
    import { defaults, superForm } from "sveltekit-superforms";
    import { zod4 } from "sveltekit-superforms/adapters";
    import { Button } from "ui/button/index.js";
    import * as Field from "ui/field/index.js";
    import { Input } from "ui/input/index.js";
    import SubmitShortcut from "shared/components/form/SubmitShortcut.svelte";
    import { submitOnModEnter } from "shared/helpers/formShortcuts";
    import ColorPicker from "shared/components/form/ColorPicker.svelte";
    import {
        useCreateWorkspace,
        useGetWorkspaces,
        useSetActiveWorkspace,
        useUpdateWorkspace,
    } from "workspace/hooks/workspace.hooks";
    import { workspaceSchema } from "../schema/workspace.schema";

    // Props using Svelte 5 runes
    let { data = null, onComplete } = $props();

    const { mutateAsync: createWorkspace } = useCreateWorkspace();
    const workspaces = useGetWorkspaces();
    const { mutateAsync: setActiveWorkspace } = useSetActiveWorkspace();
    const { mutateAsync: updateWorkspace } = useUpdateWorkspace();

    // Initialize form with defaults or existing data
    const initialData = defaults((() => data)(), zod4(workspaceSchema));
    const { form, errors, enhance, submitting } = superForm(initialData, {
        id: "add-or-update-workspace",
        validators: zod4(workspaceSchema),
        SPA: true,
        dataType: "json",
        resetForm: false,
        async onUpdate({ form: f }) {
            if (f.valid) {
                try {
                    if (data?.id) {
                        await updateWorkspace({
                            id: data.id,
                            data: { ...f.data, lastAccessedAt: new Date() },
                        });
                    } else {
                        const workspace = await createWorkspace({
                            ...f.data,
                            lastAccessedAt: new Date(),
                        });
                        await setActiveWorkspace(workspace.id);
                    }
                    onComplete?.();
                } catch (e) {
                    console.error(e);
                }
            }
        },
    });

    const duplicateWorkspaceName = $derived(
        ($form.name ?? "").trim().length > 0 &&
            (workspaces.data ?? []).some(
                (workspace) =>
                    workspace.id !== data?.id &&
                    workspace.name.trim().toLowerCase() ===
                        $form.name.trim().toLowerCase(),
            ),
    );

    const noUpdate = $derived(
        ($form.name ?? "").trim().length > 0 &&
            data &&
            data.name === $form.name,
    );

    const nameError = $derived(
        duplicateWorkspaceName
            ? "A workspace with this name already exists"
            : $errors.name,
    );
</script>

<div class="max-h-[80vh] overflow-auto border rounded-lg p-6 shadow-sm">
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <form use:enhance class="space-y-6" onkeydown={submitOnModEnter}>
        <Field.Set>
            <Field.Group>
                <div class="flex flex-col gap-4 sm:flex-row sm:items-start">
                    <Field.Field
                        class="min-w-0 flex-1"
                        data-invalid={nameError ? "" : undefined}
                    >
                        <Field.Label for="name">Workspace Name</Field.Label>
                        <Input
                            id="name"
                            bind:value={$form.name}
                            maxlength={20}
                            placeholder="e.g. Production, Local, Staging"
                            aria-invalid={nameError ? "true" : undefined}
                            class="py-4! bg-card!"
                        />
                        {#if nameError}<Field.Error>{nameError}</Field.Error
                            >{/if}
                    </Field.Field>

                    <Field.Field
                        class="w-full sm:w-48 sm:shrink-0"
                        data-invalid={$errors.color ? "" : undefined}
                    >
                        <Field.Label for="color">Theme Color</Field.Label>
                        <ColorPicker bind:data={$form.color} />
                        {#if $errors.color}<Field.Error
                                >{$errors.color}</Field.Error
                            >{/if}
                    </Field.Field>
                </div>

                <div class="flex justify-end gap-3 pt-6">
                    <Button
                        type="submit"
                        disabled={$submitting ||
                            duplicateWorkspaceName ||
                            noUpdate ||
                            $form.name.length === 0}
                        class="min-w-25"
                    >
                        {#if $submitting}
                            <Loader class="mr-2 h-4 w-4 animate-spin" />
                        {/if}
                        {data ? "Update Workspace" : "Create Workspace"}
                        <SubmitShortcut />
                    </Button>
                </div></Field.Group
            >
        </Field.Set>
    </form>
</div>
