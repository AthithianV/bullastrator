<script lang="ts">
  import * as AlertDialog from "ui/alert-dialog";
  import { buttonVariants } from "ui/button/index.js";

  const { trigger, prompt, description, actionButton, onAction } = $props();
</script>

<AlertDialog.Root>
  <AlertDialog.Trigger onclick={(e) => e.stopPropagation()}>
    {@render trigger()}
  </AlertDialog.Trigger>
  <AlertDialog.Content>
    <AlertDialog.Header>
      <AlertDialog.Title>{prompt}</AlertDialog.Title>
      <AlertDialog.Description>
        {description}
      </AlertDialog.Description>
    </AlertDialog.Header>
    <AlertDialog.Footer>
      <AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
      {@render actionButton()}
    </AlertDialog.Footer>
  </AlertDialog.Content>
</AlertDialog.Root>
