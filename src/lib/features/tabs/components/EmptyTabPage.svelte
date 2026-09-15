<script lang="ts">
  import { Database } from "@lucide/svelte";
  import * as Empty from "ui/empty/index";

  const shortcuts = [
    { label: "New Redis Connection", keys: ["Ctrl", "Shift", "N"] },
    { label: "Quick Open Queue", keys: ["Ctrl", "P"] },
    { label: "Show Commands", keys: ["Ctrl", "Shift", "P"] },
    { label: "Toggle Sidebar", keys: ["Ctrl", "B"] },
  ];
</script>

<div class="flex-1 flex items-center justify-center">
  <Empty.Root>
    <Empty.Header>
      <Empty.Media variant="icon">
        <Database class="size-10 text-muted-foreground" />
      </Empty.Media>
      <Empty.Title>No Active Tabs</Empty.Title>
      <Empty.Description>
        You haven't opened any queues or connections yet. Select a connection
        from the sidebar to get started.
      </Empty.Description>
    </Empty.Header>
    <!-- 
    <Empty.Content>
      <div class="flex gap-2 justify-center">
        <Button
          href="https://docs.bullmq.io"
          target="_blank"
          class="flex-center"
        >
          <BookOpen class="size-4" />
          Documentation
        </Button>
      </div>
    </Empty.Content> -->

    <!-- <div class="rounded-lg p-4 w-full max-w-xs border border-border/50">
      <div
        class="flex items-center gap-2 text-xs font-semibold text-muted-foreground mb-3 uppercase tracking-wider"
      >
        <Command class="size-3" /> Shortcuts
      </div>
      <div class="grid gap-2">
        {#each shortcuts as { label, keys }}
          <div class="flex items-center justify-between text-sm">
            <span class="text-muted-foreground">{label}</span>
            <Kbd.Group class="scale-90 origin-right">
              {#each keys as key, i}
                <Kbd.Root>{key}</Kbd.Root>
                {#if i < keys.length - 1}
                  <span class="text-muted-foreground/50 mx-0.5">+</span>
                {/if}
              {/each}
            </Kbd.Group>
          </div>
        {/each}
      </div>
    </div> -->
  </Empty.Root>
</div>
