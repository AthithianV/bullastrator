class QueueStore {
  activeNode: string = $state("");
  isSyncing: boolean = $state(false);
  connectionOnSync = $state<string | null>(null);

  setActiveNode(node: string) {
    this.activeNode = node;
  }

  makeActive(node: string) {
    return this.activeNode === node
      ? "bg-card  hover:bg-card rounded"
      : "hover:bg-slate-500/20 text-foreground/80 rounded";
  }
}

export const queueStore = new QueueStore();
