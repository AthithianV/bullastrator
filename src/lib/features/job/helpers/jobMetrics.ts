import type { Job } from "job/interface/job.types";
import { formatDuration } from "shared/helpers/date";

export const getJobMetrics = (job: Job) => {
  const now = Date.now();

  const processDuration =
      job.finishedOn && job.processedOn
          ? job.finishedOn - job.processedOn
          : null;

  const waitedFor = job.processedOn
      ? job.processedOn - job.timestamp
      : now - job.timestamp;

  return {
      processDuration: formatDuration(processDuration),
      waitedFor: formatDuration(waitedFor),
      // Raw values are often useful for charts/sorting in Svelte components
      raw: { processDuration, waitedFor },
  };
};
