<script lang="ts">
    let { collapsed = false } = $props<{ collapsed?: boolean }>();
</script>

<div
    class="flex items-center gap-1 px-3 py-6 transition-all duration-300 select-none"
>
    {#if !collapsed}
        <div class="flex flex-col space-y-0.5">
            <h1
                class="text-md font-black tracking-tight text-foreground italic"
            >
                Bullastrator
            </h1>
        </div>
    {/if}
</div>
