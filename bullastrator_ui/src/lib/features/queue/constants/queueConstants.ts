import type { JobStatus, QueueJobCounts } from "job/interface/job.types";

export const QUEUE_STATUS_TAB = [
  {
    value: "wait" as JobStatus,
    label: "Waiting",
    countKey: "wait" as keyof QueueJobCounts,
    badgeColor: "bg-amber-500/90",
    color: "text-amber-500",
    border:
      "dark:data-[state=active]:border-amber-500 data-[state=active]:border-amber-500",
    background:
      "dark:data-[state=active]:bg-amber-500/10 data-[state=active]:bg-amber-500/10",
    hover: "dark:hover:bg-amber-500/90 hover:bg-amber-500/90",
  },
  {
    value: "delayed" as JobStatus,
    label: "Delayed",
    countKey: "delayed" as keyof QueueJobCounts,
    badgeColor: "bg-purple-500/90",
    color: "text-purple-400",
    border:
      "dark:data-[state=active]:border-purple-500 data-[state=active]:border-purple-500",
    background:
      "dark:data-[state=active]:bg-purple-500/10 data-[state=active]:bg-purple-500/10",
    hover: "dark:hover:bg-purple-500/90 hover:bg-purple-500/90",
  },
  {
    value: "active" as JobStatus,
    label: "Active",
    countKey: "active" as keyof QueueJobCounts,
    badgeColor: "bg-sky-500/90",
    color: "text-sky-500",
    border:
      "dark:data-[state=active]:border-sky-500 data-[state=active]:border-sky-500",
    background:
      "dark:data-[state=active]:bg-sky-500/10 data-[state=active]:bg-sky-500/10",
    hover: "dark:hover:bg-sky-500/90 hover:bg-sky-500/90",
  },
  {
    value: "completed" as JobStatus,
    label: "Completed",
    countKey: "completed" as keyof QueueJobCounts,
    badgeColor: "bg-emerald-500/90",
    color: "text-emerald-400",
    border:
      "dark:data-[state=active]:border-emerald-400 data-[state=active]:border-emerald-400",
    background:
      "dark:data-[state=active]:bg-emerald-500/10 data-[state=active]:bg-emerald-500/10",
    hover: "dark:hover:bg-emerald-500/90 hover:bg-emerald-500/90",
  },
  {
    value: "failed" as JobStatus,
    label: "Failed",
    countKey: "failed" as keyof QueueJobCounts,
    badgeColor: "bg-red-400/90",
    color: "text-red-400",
    border:
      "dark:data-[state=active]:border-red-400 data-[state=active]:border-red-400",
    background:
      "dark:data-[state=active]:bg-red-500/10 data-[state=active]:bg-red-500/10",
    hover: "dark:hover:bg-red-500/90 hover:bg-red-500/90",
  },
  {
    value: "prioritized" as JobStatus,
    label: "Prioritized",
    countKey: "prioritized" as keyof QueueJobCounts,
    badgeColor: "bg-pink-500/90",
    color: "text-pink-500",
    border:
      "dark:data-[state=active]:border-pink-500 data-[state=active]:border-pink-500",
    background:
      "dark:data-[state=active]:bg-pink-500/10 data-[state=active]:bg-pink-500/10",
    hover: "dark:hover:bg-pink-500/90 hover:bg-pink-500/90",
  },
  {
    value: "paused" as JobStatus,
    label: "Paused",
    countKey: "paused" as keyof QueueJobCounts,
    badgeColor: "bg-slate-500/90",
    color: "text-slate-500",
    border:
      "dark:data-[state=active]:border-slate-500 data-[state=active]:border-slate-500",
    background:
      "dark:data-[state=active]:bg-slate-500/10 data-[state=active]:bg-slate-500/10",
    hover: "dark:hover:bg-slate-500/90 hover:bg-slate-500/90",
  },
];

export const QUEUE_ACTIONS = [
  [
    {
      actionName: "add",
      description: "Add New Job",
      size: "icon",
      visibleOnlyFor: "all",
      className: "rounded-e-none",
    },
    {
      actionName: "add-multiple",
      description: "Add Multiple Jobs",
      size: "default",
      visibleOnlyFor: "all",
      className: "rounded-s-none",
    },
  ],
  [
    {
      actionName: "import-jobs",
      description: "Import Jobs",
      size: "default",
      visibleOnlyFor: "all",
      className: "rounded-e-none",
    },
    {
      actionName: "export-jobs",
      description: "Export Jobs",
      visibleOnlyFor: "all",
      size: "default",
      className: "rounded-s-none",
    },
  ],
  [
    {
      actionName: "retry-failed",
      description: "Retry Failed Jobs",
      visibleOnlyFor: "failed",
      size: "default",
      className: "rounded-e-none",
    },
    {
      actionName: "promote-delayed",
      description: "Promote Delayed Jobs",
      visibleOnlyFor: "delayed",
      size: "default",
      className: "rounded-s-none",
    },
  ],
  [
    {
      actionName: "delete",
      description: "Delete Selected Jobs",
      size: "default",
      visibleOnlyFor: "all",
      className: "hover:text-red-500",
    },
  ],
];
