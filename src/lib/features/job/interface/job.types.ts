export type JobStatus =
  | "wait"
  | "active"
  | "completed"
  | "failed"
  | "delayed"
  | "paused"
  | "prioritized";

export type JobDetailsTab = "data" | "error" | "options" | "logs" | "timeline";

export type BackoffStrategy = number | { type: string; delay: number };

export type KeepJobs = boolean | number | { age?: number; count?: number };

export interface JobOptions {
  priority?: number;
  delay?: number;
  attempts?: number;
  backoff?: BackoffStrategy;
  timeout?: number;
  removeOnComplete?: KeepJobs;
  removeOnFail?: KeepJobs;
  stackTraceLimit?: number;
}

export interface JobDetails {
  // Identity
  id: string;
  name: string;

  // Payloads
  data: any; // generic Value
  opts: JobOptions;

  // Status & Progress
  progress: any; // number | object
  attemptsMade: number; // mapped from attempts_made

  // Timestamps
  timestamp: number;
  delay: number;
  finishedOn?: number | null; // mapped from finished_on
  processedOn?: number | null; // mapped from processed_on

  // Results & Errors
  returnvalue?: any | null;
  failedReason?: string | null; // mapped from failed_reason
  stacktrace: string[];

  // logs
  logs: string[];
}

export interface Job {
  id: string;
  name: string;

  attemptsMade: number; // mapped from attempts_made

  timestamp: number;
  delay: number;
  finishedOn?: number | null; // mapped from finished_on
  processedOn?: number | null; // mapped from processed_on
}

export interface PaginatedJobs {
  jobs: Job[];
  totalCount: number;
  totalPages: number;
  currentPage: number;
}

export interface JobSearchResult {
  jobs: Job[];
  nextCursor: number;
  hasMore: number;
  scannedCount: number;
}

export interface QueueJobCounts {
  total: number;
  wait: number;
  active: number;
  completed: number;
  failed: number;
  delayed: number;
  paused: number;
  prioritized: number;
}
