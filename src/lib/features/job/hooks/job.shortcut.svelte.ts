import type { Job } from "../interface/job.types";
import { useQueueState } from "queue/store/queueContext.svelte";

export function useJobShortcuts(getJobs: () => Job[]) {
    const queueState = useQueueState();
    let focusIndex = $state(-1);
    let anchorIndex = $state(-1);

    function handleKeydown(e: KeyboardEvent) {
        const target = e.target as HTMLElement;
        if (
            target.tagName === "INPUT" ||
            target.tagName === "TEXTAREA" ||
            target.isContentEditable
        ) {
            return;
        }

        const jobs = getJobs();
        if (jobs.length === 0) return;

        if (e.key === "Enter") {
            queueState.active.jobViewSize = 50;
        }

        if (e.key !== "ArrowDown" && e.key !== "ArrowUp") return;

        e.preventDefault();
        const direction = e.key === "ArrowDown" ? 1 : -1;
        let newIndex = focusIndex + direction;

        if (newIndex < 0) newIndex = 0;
        if (newIndex >= jobs.length) newIndex = jobs.length - 1;

        const job = jobs[newIndex];
        document
            .getElementById(`job-row-${job.id}`)
            ?.scrollIntoView({ block: "nearest" });

        if (e.shiftKey) {
            if (anchorIndex === -1) anchorIndex = focusIndex;
            const start = Math.min(anchorIndex, newIndex);
            const end = Math.max(anchorIndex, newIndex);
            queueState.active.selectedJobIds = jobs
                .slice(start, end + 1)
                .map((item) => String(item.id));
            queueState.active.selectedJob = job.id;
            queueState.active.selectedJobSummary = job;
            focusIndex = newIndex;
        } else {
            queueState.active.selectedJob = job.id;
            queueState.active.selectedJobSummary = job;
            queueState.active.selectedJobIds = [String(job.id)];
            focusIndex = newIndex;
            anchorIndex = newIndex;
        }
    }

    function handleJobClick(
        job: Job,
        index: number,
        e: MouseEvent | KeyboardEvent,
    ) {
        const jobs = getJobs();

        if (e.shiftKey && anchorIndex !== -1) {
            const start = Math.min(anchorIndex, index);
            const end = Math.max(anchorIndex, index);
            queueState.active.selectedJobIds = jobs
                .slice(start, end + 1)
                .map((item) => String(item.id));
            queueState.active.selectedJob = job.id;
            queueState.active.selectedJobSummary = job;
            focusIndex = index;
        } else if (e.ctrlKey || e.metaKey) {
            const id = String(job.id);
            if (queueState.active.selectedJobIds.includes(id)) {
                queueState.active.selectedJobIds =
                    queueState.active.selectedJobIds.filter(
                        (selectedId) => selectedId !== id,
                    );
            } else {
                queueState.active.selectedJobIds = [
                    ...queueState.active.selectedJobIds,
                    id,
                ];
            }
            queueState.active.selectedJob = job.id;
            queueState.active.selectedJobSummary = job;
            focusIndex = index;
            anchorIndex = index;
        } else {
            queueState.active.selectedJob = job.id;
            queueState.active.selectedJobSummary = job;
            queueState.active.selectedJobIds = [String(job.id)];
            focusIndex = index;
            anchorIndex = index;
        }
    }

    return { handleKeydown, handleJobClick };
}
