export function useDebounce<T>(getter: () => T, delay: number = 300) {
  let debounced = $state(getter());

  $effect(() => {
    const value = getter();
    const handler = setTimeout(() => {
      debounced = value;
    }, delay);

    return () => clearTimeout(handler);
  });

  return () => debounced;
}
