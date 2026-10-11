<script lang="ts">
  import * as Scroll from "ui/scroll-area/index";

  import { onDestroy, onMount } from "svelte";
  import {
    EditorView,
    keymap,
    lineNumbers,
    highlightActiveLineGutter,
  } from "@codemirror/view";
  import { EditorState, Compartment } from "@codemirror/state";
  import { json, jsonParseLinter } from "@codemirror/lang-json";
  import { defaultKeymap, history, historyKeymap } from "@codemirror/commands";
  import { closeBrackets, closeBracketsKeymap } from "@codemirror/autocomplete";
  import { linter, lintGutter } from "@codemirror/lint";
  import { oneDark } from "@codemirror/theme-one-dark";
  import {
    foldGutter,
    foldKeymap,
    foldAll,
    unfoldAll,
  } from "@codemirror/language";

  import {
    syntaxHighlighting,
    defaultHighlightStyle,
    indentUnit,
    bracketMatching,
  } from "@codemirror/language";
  import { cn } from "tailwind-variants";
  import Button from "../ui/button/button.svelte";
  import {
    Braces,
    Check,
    ChevronsDownUp,
    ChevronsUpDown,
    Copy,
  } from "@lucide/svelte";
  import CopyButton from "../actions/CopyButton.svelte";

  interface Props {
    value?: string;
    onChange?: ((value: string) => void) | null;
    theme?: string;
    readOnly?: boolean;
    class?: string;
  }

  let {
    value = $bindable(""),
    onChange = null,
    theme = "vs-dark",
    readOnly = false,
    class: className,
  }: Props = $props();

  let editorElement: HTMLDivElement;
  let view: EditorView | undefined = $state();
  let copied = $state(false);

  // Compartments allow us to dynamically reconfigure parts of the editor
  // without destroying the whole state.
  const readOnlyCompartment = new Compartment();
  const themeCompartment = new Compartment();
  const languageCompartment = new Compartment();

  const transparentTheme = EditorView.theme({
    "&": {
      backgroundColor: "transparent !important",
      height: "100%",
      fontWeight: 600,
    },
    // Apply font to content, gutters, and lines specifically to override defaults
    ".cm-content, .cm-gutter, .cm-line": {
      fontFamily:
        "'JetBrains Mono', 'Fira Code', Consolas, monospace !important",
      fontVariantLigatures: "common-ligatures",
      fontSize: "14px",
    },
    ".cm-gutters": {
      backgroundColor: "transparent !important",
      border: "none",
      color: (() => theme)().includes("dark") ? "#858585" : "#237893",
    },
    ".cm-content": {
      padding: "4px 0",
    },
    ".cm-line": {
      padding: "0 8px",
    },
    "&.cm-focused": {
      outline: "none",
    },
    ".cm-selectionLayer .cm-selectionBackground": {
      backgroundColor: "#3e4451 !important",
    },
    // This targets the selection when the editor is NOT focused
    "&.cm-focused .cm-selectionLayer .cm-selectionBackground": {
      backgroundColor: "#4a5261 !important",
    },
    // This targets the text color within the selection (if supported by the browser)
    ".cm-content ::selection": {
      backgroundColor: "skyblue !important",
    },

    ".cm-lintRange-error": {
      backgroundImage: `url('data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" width="6" height="3">%3Cpath d="M0 3 L3 0 L6 3" fill="none" stroke="red" stroke-width=".7"/%3E</svg>')`,
      backgroundRepeat: "repeat-x",
      backgroundPosition: "left bottom",
      paddingBottom: "1px",
    },
    // Customizes the gutter icon color
    ".cm-lint-marker-error": {
      color: "#ff5f56",
    },
    // Customizes the tooltip appearance
    ".cm-tooltip-lint": {
      backgroundColor: "#2d2d2d",
      color: "white",
      border: "1px solid #444",
    },
  });

  onMount(() => {
    const startState = EditorState.create({
      doc: value,
      extensions: [
        // 1. Basic Essentials (Stripped down version of basicSetup)
        lineNumbers(),
        lintGutter(),
        highlightActiveLineGutter(), // Optional: keep gutter highlight but not line highlight
        history(),
        bracketMatching(), // Disabled to match your Monaco config? Uncomment if needed.

        closeBrackets(),
        foldGutter(),

        linter(jsonParseLinter()),

        EditorState.tabSize.of(4), // Visual width of a tab
        indentUnit.of("    "),

        // 2. Keymaps
        keymap.of([
          ...closeBracketsKeymap,
          ...defaultKeymap,
          ...historyKeymap,
          ...foldKeymap,
        ]),

        // 3. Logic & Behavior
        EditorView.lineWrapping,
        syntaxHighlighting(defaultHighlightStyle, { fallback: true }),

        // 4. Compartments (Dynamic Settings)
        languageCompartment.of(json()),
        readOnlyCompartment.of(EditorState.readOnly.of(readOnly)),

        // 5. Theme Configuration
        // We apply the base theme (OneDark or Default) inside a compartment,
        // then apply our specific overrides (transparency/font) globally.
        themeCompartment.of(getThemeExtension(theme)),
        transparentTheme,

        // 6. Event Listener (Two-way binding)
        EditorView.updateListener.of((update) => {
          if (update.docChanged) {
            const newVal = update.state.doc.toString();
            value = newVal;
            onChange?.(newVal);
          }
        }),
      ],
    });

    view = new EditorView({
      state: startState,
      parent: editorElement,
    });
  });

  onDestroy(() => {
    view?.destroy();
  });

  // Helper to select base theme
  function getThemeExtension(themeName: string) {
    // If dark, use OneDark; if light, use empty array (default light theme)
    return themeName.includes("dark") ? oneDark : [];
  }

  // Effect: Handle external Value changes
  $effect(() => {
    if (view && value !== view.state.doc.toString()) {
      view.dispatch({
        changes: { from: 0, to: view.state.doc.length, insert: value },
      });
    }
  });

  // Effect: Handle ReadOnly changes
  $effect(() => {
    if (view) {
      view.dispatch({
        effects: readOnlyCompartment.reconfigure(
          EditorState.readOnly.of(readOnly),
        ),
      });
    }
  });

  // Effect: Handle Theme changes
  $effect(() => {
    if (view) {
      view.dispatch({
        effects: themeCompartment.reconfigure(getThemeExtension(theme)),
      });
    }
  });

  function formatJSON() {
    try {
      const parsed = JSON.parse(value);
      value = JSON.stringify(parsed, null, 2);
    } catch (e) {}
  }
</script>

<Scroll.ScrollArea class="space-y-4 bg-muted/10 flex min-w-0 relative">
  <div class="relative">
    <div class="flex gap-1.5 p-1 rounded-lg w-fit">
      <Button
        size="icon"
        variant="ghost"
        onclick={formatJSON}
        title="Format (Prettify)"
      >
        <Braces class="h-3.5 w-3.5" />
      </Button>

      <CopyButton {value} />

      <Button
        size="icon"
        variant="ghost"
        onclick={() => view && foldAll(view)}
        title="Collapse All"
      >
        <ChevronsDownUp size={14} />
      </Button>

      <Button
        size="icon"
        variant="ghost"
        onclick={() => view && unfoldAll(view)}
        title="Collapse All"
      >
        <ChevronsUpDown size={14} />
      </Button>
    </div>

    <div
      bind:this={editorElement}
      class={cn("no-scrollbar bg-black rounded p-2 min-w-0", className)}
    ></div>
  </div>
</Scroll.ScrollArea>
