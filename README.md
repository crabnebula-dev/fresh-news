# fresh-news

Tauri v2 boilerplate — Rust backend, Vite + vanilla TypeScript frontend, no UI framework.

## Prerequisites

- Rust (stable) and Cargo
- Node.js 18+ and pnpm
- macOS: Xcode Command Line Tools. Linux/Windows: see the
  [Tauri prerequisites](https://tauri.app/start/prerequisites/).

## Commands

```bash
pnpm install        # install frontend dependencies
pnpm tauri:dev      # run the desktop app with hot reload
pnpm tauri:build    # produce a release bundle in src-tauri/target/release/bundle
pnpm dev            # frontend only, in a browser at localhost:1430 (Tauri commands unavailable)
pnpm build          # typecheck + build the frontend into dist/
pnpm typecheck      # tsc --noEmit
```

`cargo check` / `cargo clippy` run from `src-tauri/`.

The dev server runs on **1430**, not Tauri's default 1420, so this app can run
alongside another Tauri project. The port appears twice — `server.port` in
`vite.config.ts` and `build.devUrl` in `src-tauri/tauri.conf.json` — and both
must agree, since `strictPort` makes a mismatch fail rather than silently
fall back.

`.prototools` pins node and pnpm for [proto](https://moonrepo.dev/proto). Without
it, proto's shims fall through to a global pnpm and abort with a "recursive
execution loop" the moment Tauri spawns `beforeDevCommand`. Delete the file if
you don't use proto.

## Layout

```
index.html            # entry document
src/
  main.ts             # DOM wiring
  bindings.ts         # typed wrappers around invoke() — the only file that calls it
  styles.css
src-tauri/
  src/main.rs         # binary entry point, defers to lib.rs
  src/lib.rs          # Builder setup: plugins, managed state, command registration
  src/commands.rs     # #[tauri::command] functions
  capabilities/       # per-window permission grants
  tauri.conf.json     # app metadata, window config, bundle targets
```

## Adding a command

Three places, always:

1. Write the `#[tauri::command] pub fn` in `src-tauri/src/commands.rs`.
2. Add it to `tauri::generate_handler![...]` in `src-tauri/src/lib.rs` — forgetting
   this compiles fine and fails at runtime with "command not found".
3. Add a typed wrapper in `src/bindings.ts`. Nothing outside that file should call
   `invoke` directly, so the TS types stay the single description of the IPC surface.

The four commands already there cover the patterns you'll reuse: a plain
string round-trip, a `Result` with a serializable error enum, a struct argument,
shared `AppState` behind a `Mutex`, and an `async` command that doesn't block the
UI thread.

Note that argument names cross the boundary as **camelCase** by default. A Rust
parameter `some_value` is `someValue` in the `invoke` payload; struct fields need
`#[serde(rename_all = "camelCase")]` to match.

## Permissions

Tauri v2 denies everything not explicitly granted. Plugin APIs called from the
frontend need their permission listed in `src-tauri/capabilities/default.json`
(`opener:default` is there as an example). Your own `#[tauri::command]`
functions do not — registering them in `generate_handler!` is the grant.

## Bundle identifier

Set to `dev.crabnebula.fresh-news` in `src-tauri/tauri.conf.json`. Change it
before shipping anything real; it keys code signing and OS-level app identity.
