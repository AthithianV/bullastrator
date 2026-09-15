import { writable } from "svelte/store";

const storedTheme = localStorage.getItem("theme") || "light";
export const theme = writable(storedTheme);

theme.subscribe((value) => {
    localStorage.setItem("theme", value);
    document.documentElement.classList.toggle("dark", value === "dark");
});
