<script lang="ts">
  import { codeToHtml } from "shiki";

  let { stacktrace = "" } = $props();
  let highlightedHtml = $state("");

  $effect(() => {
    async function highlight() {
      const cleanTrace = stacktrace
        .replace(/^\["|"]$/g, "")
        .replace(/\\n/g, "\n");

      const html = await codeToHtml(cleanTrace, {
        lang: "bash",
        theme: "one-dark-pro",
      });
      highlightedHtml = html;
    }
    highlight();
  });
</script>

<div class="shiki-error-wrapper border rounded-sm bg-[#282c34] overflow-hidden">
  {@html highlightedHtml}
</div>

<style>
  /* This targets the <pre> tag Shiki generates */
  :global(.shiki) {
    margin: 0 !important;
    padding: 1.25rem !important;
    background-color: transparent !important;
    font-family: "JetBrains Mono", monospace !important;
    font-size: 13px !important;
    font-weight: 600;
    line-height: 1.7 !important;
    min-height: 100%;
    white-space: pre !important;
  }
</style>
