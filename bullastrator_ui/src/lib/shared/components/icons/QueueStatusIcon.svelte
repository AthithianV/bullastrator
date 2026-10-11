<script lang="ts">
  import type { JobStatus } from "job/interface/job.types";
  import {
    Clock,
    Zap,
    Timer,
    CircleCheckBig,
    CirclePause,
    CircleX,
  } from "@lucide/svelte";

  const {
    status,
    size = 12,
    className = "",
  } = $props<{ status: JobStatus; size?: number; className?: string }>();

  const icons: Record<string, typeof Clock> = {
    wait: Clock,
    delayed: Timer,
    active: Zap,
    completed: CircleCheckBig,
    failed: CircleX,
    paused: CirclePause,
  };

  const Icon = $derived(icons[status] ?? Clock);
</script>

<Icon {size} class={className} />
