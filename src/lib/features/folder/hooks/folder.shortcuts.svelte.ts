import { toast } from "svelte-sonner";
import { tabStore } from "tabs/store/tabStore.svelte";

type FolderShortcutOptions = {
    tabId: () => string;
    isAllQueuesFolder: () => boolean;
    folderContainer: () => HTMLDivElement | null;
    searchInput: () => HTMLInputElement | null;
    onSearch: () => void;
    onEdit: () => void;
    onRefresh: () => void;
    onEscape: () => void;
};

export function useFolderShortcuts(options: FolderShortcutOptions) {
    const getCards = () =>
        Array.from(
            options.folderContainer()?.querySelectorAll<HTMLElement>(
                "[data-folder-queue-card]",
            ) ?? [],
        );

    const isTextInput = (target: EventTarget | null) =>
        target instanceof HTMLElement &&
        target.closest("input, textarea, [contenteditable]");

    const handleQueueNavigation = (event: KeyboardEvent) => {
        if (isTextInput(event.target)) return;

        const cards = getCards();
        const currentIndex = cards.findIndex(
            (card) =>
                card === document.activeElement ||
                card.contains(document.activeElement),
        );

        if (event.key === "Enter") {
            if (currentIndex !== -1 && event.target === cards[currentIndex]) {
                event.preventDefault();
                event.stopPropagation();
                cards[currentIndex]
                    .querySelector<HTMLButtonElement>("button")
                    ?.click();
            }
            return;
        }

        if (cards.length === 0) return;

        const grid = options.folderContainer()?.querySelector<HTMLElement>(
            "[data-folder-queue-grid]",
        );
        const columns = grid
            ? getComputedStyle(grid).gridTemplateColumns.split(" ").length
            : 1;
        const nextIndex =
            currentIndex === -1
                ? 0
                : event.key === "ArrowLeft"
                  ? currentIndex - 1
                  : event.key === "ArrowRight"
                    ? currentIndex + 1
                    : event.key === "ArrowUp"
                      ? currentIndex - columns
                      : currentIndex + columns;

        if (nextIndex < 0 || nextIndex >= cards.length) return;

        event.preventDefault();
        cards[nextIndex].focus();
    };

    const handleShortcut = (event: KeyboardEvent) => {
        if (tabStore.activeTab !== options.tabId()) return;

        if (event.key === "Escape") {
            event.preventDefault();
            options.onEscape();
            return;
        }

        if (event.metaKey || event.ctrlKey) {
            switch (event.key.toLowerCase()) {
                case "f":
                    event.preventDefault();
                    options.onSearch();
                    queueMicrotask(() => options.searchInput()?.focus());
                    return;
                case "e":
                    if (!options.isAllQueuesFolder()) {
                        event.preventDefault();
                        options.onEdit();
                    } else {
                      toast.warning("Cannot edit all Queue View!")
                    }
                    return;
                case "r":
                    if (!event.shiftKey) return;
                    event.preventDefault();
                    options.onRefresh();
                    return;
            }
        }

        if (
            [
                "Enter",
                "ArrowLeft",
                "ArrowRight",
                "ArrowUp",
                "ArrowDown",
            ].includes(event.key)
        ) {
            handleQueueNavigation(event);
        }
    };

    $effect(() => {
        window.addEventListener("keydown", handleShortcut);
        return () => window.removeEventListener("keydown", handleShortcut);
    });

    return {
        handleQueueCardKeydown: (event: KeyboardEvent) => {
            if (event.key !== "Enter" || event.target !== event.currentTarget) {
                return;
            }

            event.preventDefault();
            event.stopPropagation();
            (event.currentTarget as HTMLElement)
                .querySelector<HTMLButtonElement>("button")
                ?.click();
        },
    };
}
