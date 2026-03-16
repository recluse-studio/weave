# Desktop Shell Agents

This subtree is for the future Tauri desktop shell.

- Keep the desktop shell thin.
- Treat the local API and canonical file engine as the real runtime core.
- Do not duplicate business logic here that belongs in Rust workspace crates.
- Desktop work should strengthen the local-first experience rather than introduce hosted dependencies.
