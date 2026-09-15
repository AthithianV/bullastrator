import type { JobDetails } from "job/interface/job.types";

export function getJobTimeline(job: JobDetails) {
  const events = [];

  // 1. Created Event
  events.push({
    label: "Added to Queue",
    time: job.timestamp,
    icon: "inbox", // use your icon set
    details: `Job ID: ${job.id}`,
  });

  // 2. Processing Event (only if started)
  if (job.processedOn) {
    const waitTime = job.processedOn - job.timestamp;
    events.push({
      label: "Processing Started",
      time: job.processedOn,
      icon: "cpu",
      details: `Waited for ${(waitTime / 1000).toFixed(2)}s`,
    });
  }

  // 3. Finished Event (only if finished)
  if (job.finishedOn) {
    const duration = job.finishedOn - job.processedOn!;
    const isFailed = !!job.failedReason;

    events.push({
      label: isFailed ? "Failed" : "Completed",
      time: job.finishedOn,
      icon: isFailed ? "alert-circle" : "check-circle",
      color: isFailed ? "red" : "green",
      details: isFailed
        ? `Error: ${job.failedReason}`
        : `Duration: ${(duration / 1000).toFixed(2)}s`,
    });
  }

  return events.sort((a, b) => a.time - b.time);
}
