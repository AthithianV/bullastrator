import { getContext, setContext } from "svelte";
import type {
  JobDetails,
  JobDetailsTab,
  JobStatus,
  QueueJobCounts,
} from "job/interface/job.types";
import type { JobFilterType } from "job/interface/jobPayload.types";

const QUEUE_KEY = Symbol("QUEUE_STATE");

class StatusState {
  status: JobStatus;
  selectedJob = $state<string | null>(null);
  selectedJobDetails = $state<JobDetails | null>(null);
  selectedJobIds = $state<string[]>([]);
  currentPage = $state(0);
  jobViewSize = $state(0);
  jobDetailsTab = $state<JobDetailsTab | null>(null);

  constructor(status: JobStatus) {
    this.status = status;
  }

  reset() {
    this.selectedJob = null;
    this.selectedJobIds = [];
  }
}

export class QueueState {
  // 1. Context (The "Where")
  connectionId = $state<string>("");
  queueName = $state<string>("");
  tabId = $state<string | null>(null);

  // 2. Filter State (The "What")
  currentStatus = $state<JobStatus>("wait");
  itemsPerPage = $state(50);

  statusStates: Record<JobStatus, StatusState> = {
    wait: new StatusState("wait"),
    active: new StatusState("active"),
    completed: new StatusState("completed"),
    failed: new StatusState("failed"),
    delayed: new StatusState("delayed"),
    paused: new StatusState("paused"),
    prioritized: new StatusState("prioritized"),
  };

  actionInProgress: "RETRY" | "PROMOTE" | "DELETE" | "ADD" | "UPDATE" | null =
    $state(null);

  // Helper to get the state of the tab the user is looking at
  get active() {
    return this.statusStates[this.currentStatus];
  }

  // Job Search
  searchFilters = $state<JobFilterType[]>([]);
  searchCursor = $state<number>(0);
  searchLimit = $state<number>(500);

  jobCounts = $state<QueueJobCounts>({
    wait: 0,
    active: 0,
    completed: 0,
    failed: 0,
    delayed: 0,
    paused: 0,
    prioritized: 0,
    total: 0,
  });

  constructor(tabId: string, initialParams: any) {
    this.tabId = tabId;
    this.hydrate(initialParams);
  }

  hydrate(params: any) {
    if (!params) return;
    this.connectionId = params.connectionId;
    this.queueName = params.queueName;
    this.currentStatus = params.state;
    this.searchFilters = params.searchFilters ?? [];

    // Loop through statuses dynamically instead of hardcoding if/else
    Object.keys(this.statusStates).forEach((status) => {
      const s = status as JobStatus;
      if (params.statusStates?.[s]) {
        this.statusStates[s].selectedJob = params.statusStates[s].selectedJob;
        this.statusStates[s].jobDetailsTab =
          params.statusStates[s].jobDetailsTab;
      }
    });
  }

  toJSON() {
    return {
      type: "QUEUE" as const,
      connectionId: this.connectionId,
      queueName: this.queueName,
      state: this.currentStatus,
      searchFilters: $state.snapshot(this.searchFilters),
    };
  }

  static getDefaultParams(connectionId: string, queueName: string) {
    return {
      type: "QUEUE" as const,
      connectionId,
      queueName,
      state: "active" as JobStatus,
      searchFilters: [],
    };
  }
}

export function setQueueState(tabId: string, params: any) {
  const state = new QueueState(tabId, params);
  setContext(QUEUE_KEY, state);
  return state;
}

export function useQueueState() {
  return getContext<QueueState>(QUEUE_KEY);
}
