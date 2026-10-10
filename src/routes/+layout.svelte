<script lang="ts">
    import "$lib/styles/app.css";
    import "shared/stores/theme.svelte";

    import { Toaster } from "ui/sonner";

    import { browser } from "$app/environment";
    import { QueryClient, QueryClientProvider } from "@tanstack/svelte-query";
    import TooltipProvider from "ui/tooltip/tooltip-provider.svelte";

    const queryClient = new QueryClient({
        defaultOptions: {
            queries: {
                enabled: browser,
                refetchOnWindowFocus: false,
            },
        },
    });
    let { children } = $props();
</script>

<QueryClientProvider client={queryClient}>
    <Toaster richColors />
    <TooltipProvider>
        <main class="w-full fixed top-0 bg-card select-none min-w-0">
            {@render children()}
        </main>
    </TooltipProvider>
</QueryClientProvider>
