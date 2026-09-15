<script>
  const { loading = false, color = "bg-primary" } = $props();
</script>

<!-- The parent of this component should be relative -->
{#if loading}
  <div class="absolute top-0 left-0 w-full h-1 overflow-hidden z-50">
    <div
      class="h-full w-1/2 {color} shadow-[0_0_8px_rgba(59,130,246,0.5)] animate-loader-scan"
    ></div>
  </div>
{/if}
