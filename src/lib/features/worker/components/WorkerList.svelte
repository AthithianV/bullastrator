<script lang="ts">
  import SidebarWrapper from "components/wrapper/SidebarWrapper.svelte";
</script>

<SidebarWrapper>
  {#snippet header()}
    <h1>Worker</h1>
    <div></div>
  {/snippet}

  {#snippet content()}
    Worker Content
  {/snippet}
</SidebarWrapper>
