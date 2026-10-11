import type {
  Job,
  JobDetailsTab,
  JobStatus,
  QueueJobCounts,
} from "job/interface/job.types";
import type { JobFilterType } from "job/interface/jobPayload.types";

export interface ConnectionWithQueue {
  id: string;
  name: string;
  color: string;
  queues: ReadQueueModel[];
}

export interface ReadQueueModel {
  id: string;
  connectionId: string;
  queueName: string;
}

export interface CreateQueueModel {
  connectionId: string;
  queueName: string;
  displayName: string | null;
  isStarred: boolean | null;
  autoRefreshRate: number | null;
  notificationSettings: string | null;
}

export interface UpdateQueueModel {
  displayName?: string;
  isStarred?: boolean;
  autoRefreshRate?: number;
  notificationSettings?: string;
}

export type QueueAction =
  | "add"
  | "add-multiple"
  | "import-jobs"
  | "export-jobs"
  | "retry-failed"
  | "promote-delayed"
  | "delete"
  | "filter"
  | "refresh"
  | "clear"
  | "search";

export interface QueueActionItem {
  actionName: QueueAction;
  description: string;
  onClick?: () => void;
  visibleOnlyFor?: JobStatus;
  size?: "default" | "sm" | "lg" | "icon";
  className?: string;
}

export interface QueueTabParams {
  type?: "QUEUE";
  connectionId: string;
  queueName: string;
  searchFilters?: JobFilterType[];
  state?: JobStatus;
}

export interface QueueDetails {
  name: string;
  isPaused: boolean;
  version: string;
  prefix: string;
  activeWorkers: number;
}

export interface ReadQueueWithCounts extends ReadQueueModel {
  counts: QueueJobCounts;
}
