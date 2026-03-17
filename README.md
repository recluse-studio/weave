# WEAVE

WEAVE is a Google Workspace-native, local-first social intranet. It keeps canonical truth in ordinary files inside Google Drive while using Gmail, Calendar, Chat, Docs, Sheets, Slides, Groups, and Gemini as coordinated surfaces around that core.

The product is designed to feel like one coherent system rather than a loose bundle of integrations: structured pages and guides, metadata-rich libraries, people and project directories, a real social feed, local-first search with citations, and an integrated learning layer. The desktop app stays fast and file-based; Google Workspace handles identity, sharing, communication, scheduling, and collaboration.

## Core Principles

- Google-native from the start
- file-based canonical storage
- local-first rendering, indexing, and retrieval
- structured block content instead of generic HTML blobs
- direct APIs for hot paths and `gws`-backed control-plane actions for operator workflows
- one canonical object with optional exported or mirrored satellite surfaces

## Product Shape

WEAVE is intended to support:

- pages, guides, templates, and editorial workflows
- document, video, and image libraries
- employee, project, company, contact, opportunity, and course directories
- profiles with rich metadata and related content
- feed posts, comments, hashtags, promoted updates, and weekly digests
- local-first search with grounded, citation-rich answers
- Gemini-backed agents with visible traceability
- learning catalogs, courses, assignments, transcripts, live sessions, and analytics
- Docs, Sheets, Slides, Gmail, Calendar, Meet, Chat, Groups, and Admin coordination where enabled

## Architecture

- Rust workspace for storage, sync, Google clients, job orchestration, search, and the Gemini broker
- Tauri 2 desktop shell
- Axum local service boundary
- Svelte 5 frontend
- SQLite for rebuildable local cache
- Tantivy plus a local vector index for retrieval
- Google Drive as canonical storage and sharing layer
- `gws` as the operator-facing control plane for Google-side actions

## Repository Docs

- [weave.md](/Users/drewwiberg/Weave/weave.md): concise product brief
- [SPEC.md](/Users/drewwiberg/Weave/SPEC.md): exhaustive build specification for an autonomous implementation agent
- [AGENTS.md](/Users/drewwiberg/Weave/AGENTS.md): repo-specific guidance for long-running coding agents
- [support_docs/spec-prd.json](/Users/drewwiberg/Weave/support_docs/spec-prd.json): machine-readable gated checklist of current spec coverage

## Current Status

The repository now includes:

- a Rust workspace with file-backed feature crates
- an Axum relay that reads canonical fixture content from a Drive-shaped workspace
- sync audit and cache rebuild tooling for the workspace engine
- a Svelte frontend for dashboard, search, feed, editorial publishing, directories, libraries, learning, and automation previews
- a demo workspace fixture under `fixtures/demo-workspace/WEAVE`

The current implementation is a strong local-first baseline rather than a fully finished product. It is already useful as a working demo and development harness for the broader build.

## Quick Start

1. Install Rust with `rustup` if `cargo` is not already available.
2. Run `npm install` in the repo root.
3. Run `npm --prefix frontend install`.
4. Run `bash scripts/dev/doctor.sh` to verify the local toolchain.
5. Run `npm run dev`.
6. Open the Vite URL shown in the terminal, usually [http://127.0.0.1:5173](http://127.0.0.1:5173).

The relay serves the local API on [http://127.0.0.1:8787](http://127.0.0.1:8787).

## Useful Commands

- `npm run dev`: run the relay and frontend together
- `npm run build`: build the relay and frontend
- `npm run check`: run Rust and frontend typechecks
- `npm run test`: run Rust and frontend tests
- `npm run lint`: run formatting and lint checks where configured
- `npm run workspace:audit`: emit a machine-readable workspace integrity report
- `npm run workspace:rebuild`: rebuild the disposable local SQLite cache from canonical files
- `bash scripts/ci/security-audit.sh`: run local security-oriented checks
