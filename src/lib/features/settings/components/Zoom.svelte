<script lang="ts">
  import { handleZoom } from "shared/helpers/zoom";
  import { onMount } from "svelte";

  onMount(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.ctrlKey || e.metaKey) {
        if (e.key === "=" || e.key === "+") {
          e.preventDefault();
          handleZoom("in");
        } else if (e.key === "-") {
          e.preventDefault();
          handleZoom("out");
        } else if (e.key === "0") {
          e.preventDefault();
          handleZoom("reset");
        }
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  });
</script>
