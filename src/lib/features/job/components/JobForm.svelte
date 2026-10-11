<script lang="ts">
    import { Button } from "ui/button";
    import * as Dialog from "ui/dialog";
    import * as Field from "ui/field";
    import { Input } from "ui/input";
    import * as Resizable from "ui/resizable";

    import Editor from "components/editor/Editor.svelte";

    import { defaults, superForm } from "sveltekit-superforms";
    import { zod4 } from "sveltekit-superforms/adapters";
    import { addJobSchema } from "../schema/job.schema";

    import { Plus } from "@lucide/svelte";
    import { useQueueState } from "queue/store/queueContext.svelte";
    import InfiniteScanLoader from "shared/components/loaders/InfiniteScanLoader.svelte";
    import { useAddJob } from "../hooks/job.hooks.svelte";
    import { cn } from "utils";

    const { mutateAsync: addJob } = useAddJob();
    const queueState = useQueueState();

    const initialData = defaults(zod4(addJobSchema));

    let isFormOpen = $state(false);

    const { form, errors, enhance, reset, submitting } = superForm(
        initialData,
        {
            id: "add-or-update-job",
            validators: zod4(addJobSchema),
            SPA: true,
            dataType: "json",
            async onUpdate({ form: f }) {
                if (f.valid) {
                    let jobs = [];
                    const allJobData = JSON.parse(f.data.data);
                    if (Array.isArray(allJobData)) {
                        allJobData.forEach((data, index) => {
                            jobs.push({
                                data: data,
                                opts: f.data.opts,
                                name: `${f.data.name}-${index + 1}`,
                            });
                        });
                    } else {
                        jobs.push({
                            data: allJobData,
                            opts: f.data.opts,
                            name: f.data.name,
                        });
                    }

                    await addJob(jobs);
                    reset();
                    isFormOpen = false;
                }
            },
        },
    );
</script>

<Dialog.Root bind:open={isFormOpen}>
    <Dialog.Trigger
        class={cn(
            "py-2 px-3! cursor-pointer rounded-sm",
            "shadow",
            " hover:bg-slate-800 hover:text-primary",
        )}><Plus class="h-4 w-4" /></Dialog.Trigger
    >
    <Dialog.Content
        class="bg-card! w-[80vw] max-w-[80vw] sm:max-w-[80vw] border-none p-0"
    >
        <div class="max-h-[90vh] overflow-y-auto shadow relative">
            <InfiniteScanLoader loading={$submitting} />
            <form use:enhance>
                <h3 class="p-3 font-semibold text-lg">
                    Add New Job ({queueState.queueName})
                </h3>
                <Resizable.PaneGroup
                    direction="horizontal"
                    class="min-h-[65vh] rounded-lg px-1 pb-1"
                >
                    <Resizable.Pane defaultSize={30}>
                        <div
                            class="p-4 h-full min-w-75 bg-background rounded-lg border-none overflow-x-auto"
                        >
                            <Field.Set class="gap-2 h-full">
                                <Field.Group>
                                    <Field.Field
                                        data-invalid={$errors.name
                                            ? ""
                                            : undefined}
                                    >
                                        <Field.Label for="name"
                                            >Job Name</Field.Label
                                        >
                                        <Input
                                            id="name"
                                            bind:value={$form.name}
                                            placeholder="e.g. send-welcome-email"
                                            aria-invalid={$errors.name
                                                ? "true"
                                                : undefined}
                                        />
                                        {#if $errors.name}<Field.Error
                                                >{$errors.name}</Field.Error
                                            >{/if}
                                    </Field.Field>

                                    <div class="grid grid-cols-3 gap-4">
                                        <Field.Field
                                            data-invalid={$errors.opts?.delay
                                                ? ""
                                                : undefined}
                                        >
                                            <Field.Label for="delay"
                                                >Delay (ms)</Field.Label
                                            >
                                            <Input
                                                id="delay"
                                                type="number"
                                                bind:value={$form.opts.delay}
                                                placeholder="0"
                                            />
                                            {#if $errors.opts?.delay}<Field.Error
                                                    >{$errors.opts
                                                        .delay}</Field.Error
                                                >{/if}
                                        </Field.Field>

                                        <Field.Field
                                            data-invalid={$errors.opts?.attempts
                                                ? ""
                                                : undefined}
                                        >
                                            <Field.Label for="attempts"
                                                >Attempts</Field.Label
                                            >
                                            <Input
                                                id="attempts"
                                                type="number"
                                                bind:value={$form.opts.attempts}
                                                min="1"
                                            />
                                            {#if $errors.opts?.attempts}<Field.Error
                                                    >{$errors.opts
                                                        .attempts}</Field.Error
                                                >{/if}
                                        </Field.Field>

                                        <Field.Field
                                            data-invalid={$errors.opts?.priority
                                                ? ""
                                                : undefined}
                                        >
                                            <Field.Label for="priority"
                                                >Priority</Field.Label
                                            >
                                            <Input
                                                id="priority"
                                                type="number"
                                                bind:value={$form.opts.priority}
                                                placeholder="Optional"
                                            />
                                        </Field.Field>
                                    </div>
                                </Field.Group>
                            </Field.Set>
                        </div>
                    </Resizable.Pane>
                    <Resizable.Handle class="bg-card w-1" />
                    <Resizable.Pane defaultSize={70}>
                        <div
                            class="overflow-x-auto p-2 bg-background rounded-lg"
                        >
                            <Field.Set class="flex flex-col gap-2">
                                <Field.Group class="">
                                    <Field.Field
                                        data-invalid={$errors.data
                                            ? ""
                                            : undefined}
                                    >
                                        <Editor
                                            class="h-[70vh] rounded-sm"
                                            bind:value={$form.data}
                                        />
                                    </Field.Field>
                                </Field.Group>
                            </Field.Set>
                            <div class="flex justify-between gap-3 p-4">
                                {#if $errors.data}
                                    <Field.Error>{$errors.data}</Field.Error>
                                {:else}
                                    <p
                                        class="p-2 text-[0.8rem] text-muted-foreground"
                                    >
                                        Enter valid JSON object payload for the
                                        job. Add an Array of Object to bulk add
                                        jobs.
                                    </p>
                                {/if}
                                <Button
                                    type="submit"
                                    disabled={$submitting ||
                                        queueState.actionInProgress === "ADD"}
                                >
                                    {$submitting ||
                                    queueState.actionInProgress === "ADD"
                                        ? "Adding..."
                                        : "Add Job"}
                                </Button>
                            </div>
                        </div>
                    </Resizable.Pane>
                </Resizable.PaneGroup>
            </form>
        </div>
    </Dialog.Content>
</Dialog.Root>
