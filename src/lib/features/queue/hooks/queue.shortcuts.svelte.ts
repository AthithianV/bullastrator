import { tabStore } from "tabs/store/tabStore.svelte";

type QueueShortcutOptions = {
    tabId: () => string | null;
    toolbar: () => HTMLElement | null;
    hasSelection: () => boolean;
    hasJobsInState: () => boolean;
    onPromoteSelected: () => void;
    onPromoteAll: () => void;
    onRetrySelected: () => void;
    onRetryAll: () => void;
    onDeleteSelected: () => void;
    onDeleteAll: () => void;
    onRefresh: () => void;
    onPause: () => void;
};

export function useQueueShortcuts(options: QueueShortcutOptions) {
    const click = (name: string) => {
        options.toolbar()?.querySelector<HTMLElement>(
            `[data-queue-shortcut="${name}"]`,
        )?.click();
    };

    const focusSearch = () => {
        options.toolbar()?.querySelector<HTMLInputElement>(
            '[data-queue-shortcut="search"]',
        )?.focus();
    };

    const handleShortcut = (event: KeyboardEvent) => {
        if (tabStore.activeTab !== options.tabId()) return;

        const modifier = event.metaKey || event.ctrlKey;
        const key = event.key.toLowerCase();

        if (modifier && key === "n" && !event.shiftKey) {
            event.preventDefault();
            click("new-job");
        } else if (modifier && key === "p") {
            event.preventDefault();
            if (event.shiftKey) {
                if (options.hasJobsInState()) options.onPromoteAll();
            } else if (options.hasSelection()) {
                options.onPromoteSelected();
            }
        } else if (modifier && key === "z") {
            event.preventDefault();
            if (event.shiftKey) {
                if (options.hasJobsInState()) options.onRetryAll();
            } else if (options.hasSelection()) {
                options.onRetrySelected();
            }
        } else if (!modifier && event.key === "Delete") {
            event.preventDefault();
            if (event.shiftKey) {
                if (options.hasJobsInState()) options.onDeleteAll();
            } else if (options.hasSelection()) {
                options.onDeleteSelected();
            }
        } else if (modifier && key === "e") {
            event.preventDefault();
            click("export");
        } else if (modifier && key === "f") {
            event.preventDefault();
            if (event.shiftKey) click("filter");
            else focusSearch();
        } else if (modifier && key === "r") {
            event.preventDefault();
            options.onRefresh();
        } else if (event.ctrlKey && !event.metaKey && key === "u") {
            event.preventDefault();
            options.onPause();
        }
    };

    $effect(() => {
        window.addEventListener("keydown", handleShortcut);
        return () => window.removeEventListener("keydown", handleShortcut);
    });
}
