import type { JobOptions, JobStatus } from "./job.types";

export interface GetJobsPayload {
  queueName: string;
  status: JobStatus;
  limit: number;
  connectionId: string;
  filters: JobFilterType[];
}

export interface AddJobPayload {
  data: any;
  name: string;
  opts?: JobOptions;
}

export interface UpdateJobPayload {
  jobId: string;
  jobData: any;
}

export interface RetryJobPayload {
  jobIds?: string[];
  strategy: "ToBack" | "ToFront";
  connectionId: string;
  queueName: string;
  retryJobStatus: 'failed' | 'completed';
}

export interface PromoteJobsPayload {
  queueName: string;
  connectionId: string;
  jobIds?: string[];
}

export interface DeleteJobsPayload {
  queueName: string;
  connectionId: string;
  jobIds?: string[];
  status: JobStatus;
}

export type JobFilterType =
  | { type: "JobId"; value: string }
  | { type: "Keyword"; value: string }
  | { type: "ReturnValue"; value: string }
  | { type: "JobName"; value: string }
  | { type: "FailedReason"; value: string }
  | { type: "Attempts"; value: number }
  | { type: "CreatedAtRange"; value: [number, number] }
  | { type: "DurationLongerThan"; value: number };

export interface SearchOptions {
  limit: number;
  filters: JobFilterType[];
}

export interface FilterJobsPayload {
  connectionId: string;
  queueName: string;
  status: JobStatus;
  options: SearchOptions;
}
