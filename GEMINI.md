# Bullastrator: BullMQ Desktop Manager

Bullastrator is a cross-platform desktop application built with **Tauri v2** and **Svelte 5**, designed for managing BullMQ queues with direct Redis connections.

## 🏗️ Core Architecture

- **Frontend:** Svelte 5 (Runes), SvelteKit (Static Adapter), TypeScript.
- **Backend:** Rust (Tauri v2), SeaORM (SQLite), `redis-rs`, `deadpool-redis`.
- **UI/UX:** Tailwind CSS v4, Shadcn-svelte (bits-ui), Lucide icons.
- **Persistence:** Local SQLite for app state (workspaces, folders, tabs), OS Keychain for Redis passwords.
- **Communication:** Bi-directional via Tauri commands (`invokeWrapper`) and events (`listen`).

## 📜 Technical Mandates

### 1. Svelte 5 Runes

Always use Svelte 5 runes for reactivity. Avoid old Svelte 4 syntax (e.g., `export let`, `$:`, `onMount` for simple reactivity).

- Use `$state`, `$derived`, `$effect`, `$props`, `$render`.
- Use `$bindable` for two-way bindings in components.
- Standardize on class-based stores with runes (e.g., `globalStore.svelte.ts`).

### 2. Tauri Command Pattern

All frontend-to-backend communication MUST go through `invokeWrapper`:

- Location: `src/lib/shared/helpers/invokeWrapper.ts`.
- Usage: `invokeWrapper<TReturn>(cmd, args, options)`.
- Backend: Ensure commands are registered in `src-tauri/src/lib.rs` and follow the `Result<T, E>` return pattern with `anyhow` or custom errors.

### 3. Feature-Based Organization

Maintain the established structure in `src/lib/features/`. Each feature should contain:

- `components/`: UI components specific to the feature.
- `hooks/`: Svelte Query hooks or custom state hooks.
- `stores/`: (Optional) Feature-specific rune stores.
- `interfaces/`: TypeScript definitions.

### 4. Styling & UI

- **Tailwind CSS v4:** Use modern Tailwind v4 features.
- **Shadcn-Svelte:** Reuse existing UI components in `src/lib/shared/components/ui/`.
- **Icons:** Use `lucide-svelte`.
- **Themes:** Supported via `mode-watcher`.

### 5. Backend (Rust)

- **SeaORM:** Use for SQLite interactions. Keep entities in `src-tauri/src/entity/` and repositories in `src-tauri/src/repository/`.
- **Redis Drivers:** Use `deadpool-redis` for connection pooling.
- **Error Handling:** Use `thiserror` for library errors and `anyhow` for commands.
- **Security:** Use the `keyring` crate for storing sensitive Redis credentials. NEVER log credentials.

### 6. Performance & UX

- **Virtualization:** Use `@tanstack/svelte-virtual` for rendering large job lists (>1000 items).
- **Latency:** Minimize UI blocking during Redis operations. Use async Tauri commands.
- **Tabs:** Use the existing tab system (`src/lib/features/tabs`) for main workspace views.

## 🚀 Workflows

### Adding a New Command

1. Define the command function in `src-tauri/src/commands/`.
2. Implement business logic in `src-tauri/src/service/`.
3. Register the command in `tauri::generate_handler!` within `src-tauri/src/lib.rs`.
4. Create a corresponding hook in the frontend using `invokeWrapper`.

### Bug Fixes

1. Reproduce the issue with a script or specific UI steps.
2. Check `tauri-plugin-log` output (available in the log directory).
3. Apply surgical fixes in the relevant layer (Frontend/Rust).
4. Verify with the reproduction steps.

## 📁 Key Directories

- `src/lib/features/`: Core business features.
- `src/lib/shared/`: Reusable utilities, components, and hooks.
- `src-tauri/src/commands/`: Tauri invoke handlers.
- `src-tauri/src/service/`: Backend business logic.
- `src-tauri/src/bull_drivers/`: Low-level Redis/BullMQ interactions.
