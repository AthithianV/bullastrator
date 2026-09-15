<script lang="ts">
  import TextViewer from "shared/components/editor/TextViewer.svelte";
  import * as Scroll from "ui/scroll-area";
  import * as Alert from "ui/alert";
  import { Separator } from "shared/components/ui/separator";
  import { extractLink, isLink } from "shared/helpers/linkify";
  import CopyButton from "shared/components/actions/CopyButton.svelte";

  type Props = {
    stackTrace: string[];
    failedReason?: string | null;
  };
  let { stackTrace, failedReason }: Props = $props<{
    stackTrace: string[];
    failedReason?: string | null;
  }>();

  let reversedStack = $derived([...stackTrace].reverse());
</script>

{#if stackTrace.length > 0}
  <div class="space-y-2 p-4 select-text">
    {#if failedReason}
      <Alert.Root variant="destructive" class="rounded-sm w-full overflow-auto">
        <div class="absolute top-3 right-3 text-gray-100">
          <CopyButton value={failedReason} />
        </div>
        <Alert.Title>Failed Reason</Alert.Title>
        <Alert.Description class="relative select">
          {#each extractLink(failedReason) as part}
            {#if isLink(part)}
              <a
                href={part}
                target="_blank"
                rel="noopener noreferrer"
                class="text-blue-800 underline break-all"
              >
                {part}
              </a>
            {:else}
              {part}
            {/if}
          {/each}
        </Alert.Description>
      </Alert.Root>
    {/if}
    <Separator />
    {#each reversedStack as error, groupIndex}
      <div class="overflow-hidden">
        <div class="font-bold opacity-50 text-gray-900 dark:text-gray-100 mb-2">
          # {stackTrace.length - groupIndex}
        </div>
        <Scroll.ScrollArea
          orientation="horizontal"
          class="w-full min-w-0 relative"
        >
          <div class="absolute top-3 right-3 text-gray-100">
            <CopyButton value={error} />
          </div>
          <TextViewer stacktrace={error} />
          <Scroll.Scrollbar orientation="horizontal" />
        </Scroll.ScrollArea>
      </div>
      <Separator />
    {/each}
  </div>
{:else}
  <div class="text-center py-10 text-muted-foreground text-sm">
    <span class="flex items-center justify-center gap-2">
      <div class="h-2 w-2 rounded-full bg-green-500"></div>
      No errors detected.
    </span>
  </div>
{/if}
