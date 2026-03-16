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

## Current Status

This repository is being set up as a specification-first project. The immediate goal is to define the product, the architecture, the implementation order, and the operating rules clearly enough that a long-running coding agent can build the system end to end without reinterpreting the fundamentals midstream.
