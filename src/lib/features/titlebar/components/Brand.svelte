<script lang="ts">
    import * as Avatar from "ui/avatar";
    let { collapsed = false } = $props<{ collapsed?: boolean }>();
</script>

<div
    class="flex items-center gap-1 px-3 py-6 transition-all duration-300 select-none"
>
    <!-- <Avatar.Root class="h-10 w-10 bg-background">
        <Avatar.Image src="/logo.png" alt="bullastrator" class="object-contain p-1.5 rounded-full" />
        <Avatar.Fallback class="rounded-xl bg-primary text-primary-foreground font-bold">
            CB
        </Avatar.Fallback>
    </Avatar.Root> -->

    {#if !collapsed}
        <div class="flex flex-col space-y-0.5">
            <h1
                class="text-md font-black tracking-tight text-foreground italic"
            >
                Bullastrator
            </h1>
            <!-- <div class="flex items-center gap-1.5">
                <span class="relative flex h-2 w-2">
                    <span
                        class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"
                    ></span>
                    <span class="relative inline-flex rounded-full h-2 w-2 bg-emerald-500"></span>
                </span>
                <p
                    class="text-[10px] font-semibold uppercase tracking-widest text-muted-foreground"
                >
                    Tauri-Redis
                </p>
            </div> -->
        </div>
    {/if}
</div>
