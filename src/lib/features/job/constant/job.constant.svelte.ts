import {
  Hash,
  Type,
  Timer,
  Repeat
} from "@lucide/svelte";
import type { Component } from "svelte";

interface HeaderItem {
  label?: string;
  icon?: Component;
  class?: string;
}

export const JOB_LIST_HEADER: HeaderItem[] = [
  { class: "w-10", label: "No." },
  { label: "ID", icon: Hash },
  { label: "Job Name", icon: Type },
  { label: "Added at", icon: Timer },
  { label: "Delayed", icon: Timer },
  { label: "Waited For", icon: Timer },
  { label: "Exection Time", icon: Timer },
  { label: "Attempts", icon: Repeat },
];
