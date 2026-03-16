# WEAVE

WEAVE is a Google Workspace-native, local-first social intranet for individuals, studios, departments, and distributed teams.

It keeps canonical truth in ordinary files inside Google Drive. The desktop app handles fast local rendering, indexing, editing, and search. Google Workspace provides the surrounding collaboration plane: identity, sharing, mail, calendar, chat, docs, sheets, slides, and model access.

## North Star

WEAVE should feel like three systems folded into one calm, coherent object:

- an editorial workspace for policies, handbooks, standards, guides, and project knowledge
- a social workspace for posts, hashtags, discussion, recognition, and weekly rhythm
- a Google Workspace cockpit that can mirror, export, route, schedule, and govern work across the rest of the suite

The product should open like a bright, orderly studio: live but not noisy, structured but not rigid, and useful before the user clicks anything.

## Product Principles

- Google-native, not Google-adjacent
- file-based, not row-based
- local-first, cloud-coordinated
- structured content, not HTML blobs
- direct APIs on hot paths, scripted control-plane flows on cold paths
- one canonical object with many optional satellite surfaces

## Hard Rules

- Canonical truth lives in ordinary files inside Google Drive.
- SQLite is cache only.
- Search indexes, embeddings, transcripts, extracted text, and thumbnails are cache only.
- Google Docs, Sheets, and Slides are secondary surfaces, never canonical page objects.
- No mandatory WEAVE-owned backend is allowed.
- Gmail is the outbound mail surface.
- Google Calendar is the event surface.
- Google Chat is the alert and mirror surface, not the primary feed.
- `gws` is the official operator shell for Google-side actions.
- Gemini is the runtime model layer.

## Core Surfaces

WEAVE must support:

- block-based pages and guides
- navigation-rich home pages and community pages
- document, video, and image libraries
- employee, project, company, contact, opportunity, and course directories
- profiles with rich metadata and related content
- feed posts, comments, likes, mentions, hashtags, promoted posts, and digests
- local-first search with citation-rich answers
- agents that can draft, post, summarize, schedule, and export with visible traces
- an integrated learning layer with catalogs, courses, lessons, assignments, transcripts, analytics, and live sessions
- Docs, Sheets, Slides, Gmail, Calendar, Meet, Chat, Groups, and Admin coordination where enabled

## Runtime Modes

- Personal Google account: My Drive workspace with a first-class solo experience
- Google Workspace user: My Drive or shared drive workspace with richer collaboration features
- Google Workspace admin: team-owned shared drive mode with admin-backed governance, group sync, and label mirroring
- Optional relay: background automation, browser mode, webhooks, exports, and scheduled jobs without becoming the source of truth

## Build Priorities

1. Bootstrap the desktop shell, Drive root selection, Google auth, Gemini setup, and demo workspace.
2. Build the canonical file engine, local cache rebuilds, and sync health.
3. Deliver pages, guides, templates, navigation, libraries, directories, and profiles.
4. Deliver feed, search, agents, learning, and Google Workspace coordination.
5. Add privacy, governance, relay mode, and production hardening without breaking the file-based model.

## Implementation Posture

Use Rust for the core runtime, Tauri 2 for the desktop shell, Axum for local APIs and services, and Svelte 5 for the frontend.

Everything important must remain reconstructable from the Drive workspace alone. If caches are deleted, a machine is replaced, or the relay disappears, the product should recover by reconnecting to the chosen Drive root and replaying canonical files.
