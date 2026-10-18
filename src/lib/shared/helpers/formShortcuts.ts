export function submitOnModEnter(event: KeyboardEvent) {
    if (!(event.metaKey || event.ctrlKey) || event.key !== "Enter") return;

    event.preventDefault();
    (event.currentTarget as HTMLFormElement).requestSubmit();
}
