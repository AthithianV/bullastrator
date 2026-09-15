<script lang="ts">
  import { Check, Copy } from "@lucide/svelte";
  import { Button } from "../ui/button";

  const { value }: { value: string } = $props();
  let copied = $state(false);

  async function handleCopy() {
    try {
      await navigator.clipboard.writeText(value);
      copied = true;
      setTimeout(() => {
        copied = false;
      }, 1000);
    } catch (err) {
      console.error("Failed to copy: ", err);
    }
  }
</script>

<Button
  size="icon"
  variant="ghost"
  onclick={handleCopy}
  title="Copy to Clipboard"
>
  {#if copied}
    <Check class="text-green-500" size={14} />
  {:else}
    <Copy size={14} />
  {/if}
</Button>
