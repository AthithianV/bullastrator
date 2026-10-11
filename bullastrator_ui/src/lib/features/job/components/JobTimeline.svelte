<script lang="ts">
    import { fly } from "svelte/transition";
    import { getJobTimeline } from "../helpers/jobTimeline";
    import type { Job } from "../interface/job.types";
    import dayjs from "dayjs";

    let { job } = $props<{ job: Job }>();

    let events = $derived(getJobTimeline(job));

    const formatTime = (ts: number) => {
        return dayjs(ts).format("DD MMM YYYY, hh:mm:ss A");
    };

    const icons = {
        inbox: `<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4"><path stroke-linecap="round" stroke-linejoin="round" d="M2.25 13.5h3.86a2.25 2.25 0 012.012 1.244l.256.512a2.25 2.25 0 002.013 1.244h3.218a2.25 2.25 0 002.013-1.244l.256-.512a2.25 2.25 0 012.013-1.244h3.859M12 3v8.25m0 0l-3-3m3 3l3-3" /></svg>`,
        cpu: `<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4"><path stroke-linecap="round" stroke-linejoin="round" d="M8.25 3v1.5M4.5 8.25H3m18 0h-1.5M4.5 12H3m18 0h-1.5m-15 3.75H3m18 0h-1.5M8.25 19.5V21M12 3v1.5m0 15V21m3.75-18v1.5m0 15V21m-9-1.5h10.5a2.25 2.25 0 002.25-2.25V6.75a2.25 2.25 0 00-2.25-2.25H6.75A2.25 2.25 0 004.5 6.75v10.5a2.25 2.25 0 002.25 2.25z" /></svg>`,
        "check-circle": `<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4"><path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75L11.25 15 15 9.75M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>`,
        "alert-circle": `<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4"><path stroke-linecap="round" stroke-linejoin="round" d="M12 9v3.75m9-.75a9 9 0 11-18 0 9 9 0 0118 0zm-9 3.75h.008v.008H12v-.008z" /></svg>`,
    };

    // Color Mapping for the icon background
    const colors = {
        default: "bg-blue-100 text-blue-600",
        red: "bg-red-100 text-red-600",
        green: "bg-green-100 text-green-600",
        gray: "bg-gray-100 text-gray-500",
    };
</script>

<div class="flex flex-col space-y-0 relative pl-2">
    <div
        class="absolute left-4.75 top-2 bottom-4 w-0.5 bg-gray-200 dark:bg-gray-700 -z-10"
    ></div>

    {#each events as event, i (event.label)}
        <div
            in:fly={{ y: 10, duration: 300, delay: i * 100 }}
            class="relative flex items-start gap-4 pb-6 last:pb-0 group p-4"
        >
            <div
                class="relative z-10 flex h-8 w-8 shrink-0 items-center justify-center rounded-full border-2 border-white dark:border-gray-800 shadow-sm
        {event.color
                    ? colors[event.color as keyof typeof colors]
                    : colors.default}"
            >
                {@html icons[event.icon as keyof typeof icons] || icons.inbox}
            </div>

            <div class="flex flex-col min-w-0 pt-1">
                <div class="flex items-baseline gap-2">
                    <p
                        class="text-sm font-semibold text-gray-900 dark:text-gray-100"
                    >
                        {event.label}
                    </p>
                    <span class="text-xs text-gray-400 font-mono">
                        {formatTime(event.time)}
                    </span>
                </div>

                <p
                    class="text-xs text-gray-500 dark:text-gray-400 mt-0.5 wrap-break-words"
                >
                    {event.details}
                </p>
            </div>
        </div>
    {/each}
</div>
