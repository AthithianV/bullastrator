<script lang="ts">
  import * as ButtonGroup from "ui/button-group";
  import Button from "ui/button/button.svelte";

  import { Check, Copy, Share, SquarePen } from "@lucide/svelte";
  import { useUpdateJobData } from "../hooks/job.hooks.svelte";

  import DataEditor from "components/editor/Editor.svelte";
  import { useQueueState } from "queue/store/queueContext.svelte";

  const { jobData }: { jobData: string } = $props<{ jobData: string }>();

  const { mutateAsync: updateJobData } = useUpdateJobData();

  const queueState = useQueueState();

  let newJobData = $derived(jobData);

  let isJobDataChanges = $derived(jobData !== newJobData);

  const updateData = async () => {
    if (newJobData === jobData) return;

    if (
      jobData &&
      queueState.active.selectedJob &&
      queueState.currentStatus !== "active"
    ) {
      await updateJobData({
        jobData: JSON.parse(newJobData),
        jobId: queueState.active.selectedJob,
      });
    }
  };

  let copied = $state(false); // Using Svelte 5 state; use let copied = false if on Svelte 4

  async function handleCopy() {
    try {
      // jobData is the value from your DataEditor
      await navigator.clipboard.writeText(newJobData);

      copied = true;
      setTimeout(() => {
        copied = false;
      }, 1000);
    } catch (err) {
      console.error("Failed to copy: ", err);
    }
  }
</script>

<DataEditor class="h-[55vh]" bind:value={newJobData} />
<ButtonGroup.Root class="flex justify-between items-center w-full">
  <div></div>

  <ButtonGroup.ButtonGroup>
    {#if queueState.currentStatus !== "active"}
      <Button class="border-e" onclick={updateData} disabled={!isJobDataChanges}
        ><SquarePen /> Update</Button
      >
    {/if}
  </ButtonGroup.ButtonGroup>
</ButtonGroup.Root>
