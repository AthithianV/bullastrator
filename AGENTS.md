<!--VITE PLUS START-->

# Using Vite+, the Unified Toolchain for the Web

This project is using Vite+, a unified toolchain built on top of Vite, Rolldown, Vitest, tsdown, Oxlint, Oxfmt, and Vite Task. Vite+ wraps runtime management, package management, and frontend tooling in a single global CLI called `vp`. Vite+ is distinct from Vite, and it invokes Vite through `vp dev` and `vp build`. Run `vp help` to print a list of commands and `vp <command> --help` for information about a specific command.

Docs are local at `node_modules/vite-plus/docs` or online at https://viteplus.dev/guide/.

## Built-in Commands vs Scripts

`vp <name>` runs a built-in command. `vp run <name>` runs a `package.json` script or a `vite.config.ts` task. Scripts cannot overwrite built-ins, so `vp dev` and `vp run dev` may do different things. Check `package.json` and `vite.config.ts` first, and run `vp run <name>` when the project defines a script or task with that name.

## Tool Versions

Run `vp toolchain` to show versions and relationships in the active Vite+
release. Add a tool name to select part of the graph. For example, run
`vp toolchain vite`. Use `--global` to ignore the local `vite-plus` package. Use
`vp why <package>` to show the package-manager dependency graph.

## Review Checklist

- [ ] Run `vp install` after pulling remote changes and before getting started.
- [ ] Run `vp check` and `vp test` to format, lint, type check and test changes.
- [ ] Check if there are `vite.config.ts` tasks or `package.json` scripts necessary for validation, run via `vp run <script>`.
- [ ] If setup, runtime, or package-manager behavior looks wrong, run `vp env doctor` and include its output when asking for help.

<!--VITE PLUS END-->

# Agent Guide for Bullastrator

## Project overview

Bullastrator is a desktop application for inspecting and managing Redis/BullMQ queues. It uses:

- Svelte 5, SvelteKit, TypeScript, Vite, and Tailwind CSS for the frontend.
- Tauri 2 for the native desktop shell.
- Rust for queue access, persistence, web/API behavior, CLI behavior, and desktop commands.
- Redis and BullMQ as the primary external data sources.

The project is under active development. Preserve existing behavior and keep changes focused.

## Repository layout

- `src/`: SvelteKit frontend routes, layouts, components, shared utilities, and styles.
- `src/routes/`: application pages, including the main page, login, and registration flows.
- `src/lib/`: reusable UI layouts, components, utilities, and global styles.
- `crates/`: Rust workspace.
  - `bullastrator_core`: domain logic, models, services, BullMQ drivers, Redis access, and shared state.
  - `bullastrator_storage`: local persistence/storage layer.
  - `bullastrator_web`: web/API server and middleware.
  - `bullastrator_cli`: CLI binary.
  - `bullastrator_desktop`: Tauri application, native commands, capabilities, and desktop configuration.
- `docs/`: setup notes, roadmap, and screenshots.
- `.github/workflows/`: frontend, Rust, and release CI workflows.

## Local development

Prerequisites: Node.js, npm, stable Rust/Cargo, Redis, and the Tauri prerequisites for the host OS.

Install frontend dependencies:

```bash
npm install
```

Run the desktop application:

```bash
npm run tauri dev
```

Run the frontend only:

```bash
npm run dev
```

Run the Rust web service from the workspace:

```bash
cd crates
cargo watch -c -x check -x 'run -p bullastrator-web'
```

Use `.env.example` as the template for local environment variables. Never commit `.env` files or credentials.

## Validation commands

Run the checks relevant to the files changed:

```bash
# Frontend type and Svelte checks
npm run check

# Frontend production build
npm run build

# Rust formatting, compilation, and tests
cd crates
cargo fmt --check
cargo check
cargo test
```

The CI workflow runs `npm ci` followed by `npm run build`, and runs `cargo check` and `cargo test` from `crates/`.

When changing Tauri configuration or native commands, validate the desktop app with `npm run tauri dev` when the required OS dependencies are available.

## Implementation conventions

- Keep frontend changes in `src/` and Rust changes in the appropriate crate; avoid crossing layers unnecessarily.
- Reuse existing Svelte components, layouts, utilities, and styling patterns before adding new abstractions.
- Keep Rust domain behavior in `bullastrator_core`; keep Tauri-specific wiring and commands in `bullastrator_desktop`.
- Prefer small, focused functions, clear names, explicit error handling, and reusable components.
- Follow existing TypeScript/Svelte and Rust formatting conventions. Run the relevant formatter/checker after edits.
- Use the existing workspace dependencies where possible; avoid adding dependencies for small utilities.
- Treat Redis connection details, passwords, API keys, tokens, and production URLs as secrets.
- For UI changes, preserve the existing desktop experience and include screenshots or recordings in pull requests when useful.

## Change workflow

1. Inspect nearby code and existing patterns before editing.
2. Make the smallest coherent change that addresses the request.
3. Run focused checks first, then the broader validation commands when practical.
4. Review the diff for unrelated changes, generated files, secrets, and accidental formatting churn.
5. Summarize changed files and validation results.

Do not modify generated build output such as `node_modules/`, `.svelte-kit/`, or `crates/target/` unless the task explicitly requires it.

## Security and data safety

Never commit credentials, Redis passwords, private keys, `.env` files containing secrets, or production connection information. For a suspected vulnerability, follow `SECURITY.md` rather than opening a public issue.

## Further guidance

- User-facing setup: `README.md`
- Contribution and style expectations: `CONTRIBUTING.md`
- Current known TODOs: `WHERE_I_LEFT.md`
- Development notes and roadmap: `docs/`
