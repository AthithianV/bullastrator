import { writable } from "svelte/store";
import { browser } from "$app/environment";

const systemTheme =
    browser && window.matchMedia("(prefers-color-scheme: dark)").matches
        ? "dark"
        : "light";
const storedTheme = browser ? localStorage.getItem("theme") : null;
const initialTheme = storedTheme === "dark" || storedTheme === "light"
    ? storedTheme
    : systemTheme;

export const theme = writable(initialTheme);

if (browser) {
    theme.subscribe((value) => {
        localStorage.setItem("theme", value);
        document.documentElement.classList.toggle("dark", value === "dark");
    });
}
