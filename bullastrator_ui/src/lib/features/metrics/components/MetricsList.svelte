<script lang="ts">
  import SidebarWrapper from "components/wrapper/SidebarWrapper.svelte";
</script>

<SidebarWrapper>
  {#snippet header()}
    <h1>Metrics</h1>
    <div></div>
  {/snippet}

  {#snippet content()}
    Metrics Content
  {/snippet}
</SidebarWrapper>
