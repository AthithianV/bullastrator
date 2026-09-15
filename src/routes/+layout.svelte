<script lang="ts">
  import "$lib/styles/app.css";

  import { Toaster } from "ui/sonner";

  import { browser } from "$app/environment";
  import { QueryClient, QueryClientProvider } from "@tanstack/svelte-query";
  import TooltipProvider from "ui/tooltip/tooltip-provider.svelte";

  const queryClient = new QueryClient({
    defaultOptions: {
      queries: {
        enabled: browser,
      },
    },
  });
  let { children } = $props();
</script>

<QueryClientProvider client={queryClient}>
  <Toaster richColors />
  <TooltipProvider>
    {@render children()}
  </TooltipProvider>
</QueryClientProvider>
