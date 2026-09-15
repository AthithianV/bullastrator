import dayjs from "dayjs";
import relativeTime from "dayjs/plugin/relativeTime";
import updateLocale from "dayjs/plugin/updateLocale";

dayjs.extend(relativeTime);
dayjs.extend(updateLocale);

export function timeAgo(timestamp: number | string): string {
  if (!timestamp) return "Unknown";
  return dayjs(Number(timestamp)).fromNow();
}

export function formatDate(
  timestamp: number | string,
  format: string = "DD-MM-YYYY HH:mm:ss",
): string {
  if (!timestamp) return "Unknown";
  return dayjs(Number(timestamp)).format(format);
}

const MS_PER_SEC = 1000;
const MS_PER_MIN = 60 * MS_PER_SEC;
const MS_PER_HOUR = 60 * MS_PER_MIN;
const MS_PER_DAY = 24 * MS_PER_HOUR;
const MS_PER_MONTH = 30 * MS_PER_DAY; // Approximation
const MS_PER_YEAR = 12 * MS_PER_MONTH;

const UNITS = [
  { label: "years", ms: MS_PER_YEAR },
  { label: "month", ms: MS_PER_MONTH },
  { label: "day", ms: MS_PER_DAY },
  { label: "hrs", ms: MS_PER_HOUR },
  { label: "min", ms: MS_PER_MIN },
  { label: "sec", ms: MS_PER_SEC },
  { label: "ms", ms: 1 },
];

export const formatDuration = (ms: number | null): string => {
  if (ms === null || ms < 0) return "-";
  if (ms === 0) return "0 ms";

  let remaining = ms;
  const parts: string[] = [];

  for (const unit of UNITS) {
    if (remaining >= unit.ms) {
      const value = Math.floor(remaining / unit.ms);
      parts.push(`${value} ${unit.label}`);
      remaining %= unit.ms;
    }

    // Break once we have found the two largest entities
    if (parts.length === 2) break;
  }

  return parts.join(" ");
};
