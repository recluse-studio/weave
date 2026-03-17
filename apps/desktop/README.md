# Desktop Shell

The desktop shell is a thin Tauri wrapper around the existing local API and Svelte frontend.

Current shape:

- `package.json` runs the Tauri development and build commands
- `src-tauri/src/main.rs` starts the shared Axum API on `127.0.0.1:8787`
- the frontend keeps talking to that same local API whether it is opened in the browser or in the desktop shell

The shell stays intentionally thin so the local-first runtime remains in the shared Rust crates rather than being duplicated in Tauri-specific code.
