# AGENTS

This repository is meant to be executable by long-running coding agents. Read this file before making changes. Then read `SPEC.md`. Treat `weave.md` as the concise product brief and `SPEC.md` as the detailed execution contract.

## Mission

Build WEAVE as a Google Workspace-native, local-first social intranet whose canonical truth lives in ordinary files inside Google Drive.

The repository should evolve in working vertical slices, not in disconnected scaffolding bursts. A good change leaves the app more real, more testable, and more legible to the next agent.

## Absolute Rules

- Canonical truth lives in Drive files, never in SQL tables, hosted databases, or Google-native document pointers.
- SQLite, indexes, embeddings, transcripts, thumbnails, and extracted text are rebuildable caches only.
- Google Docs, Sheets, and Slides are secondary surfaces, never canonical page objects.
- `gws` is the control plane for operator-visible Google actions.
- Hot paths must not shell out to `gws`.
- Keep page, event, recipe, and settings formats stable.
- Preserve the demo workspace and keep it runnable.
- Do not replace structured blocks with generic HTML blobs.
- Do not introduce a mandatory hosted backend.
- Do not hard-code secrets, tokens, or environment-specific values in tracked files.

## Read Order

1. Read this file.
2. Read `SPEC.md`.
3. Read the nearest nested `AGENTS.md` before editing a major subtree.
4. Inspect the relevant code before changing it.

If guidance conflicts:

- `SPEC.md` wins on implementation detail.
- the nearest nested `AGENTS.md` wins on local subtree practice.
- preserve the non-negotiables above in all cases.

## Working Style

- Do not stop at a plan when implementation is feasible.
- Prefer end-to-end vertical slices over isolated stubs.
- Make conservative assumptions and continue unless the choice is destructive or credential-bound.
- When a feature depends on unavailable credentials, finish the local architecture, mocks, fixtures, adapters, and activation hooks anyway.
- Keep diffs understandable. Small coherent steps beat giant opaque dumps.
- Update docs when a code change alters architecture, workflow, or operating rules.

## Repository Priorities

The default order of work is:

1. bootstrap and local runnability
2. canonical workspace engine and sync
3. pages, guides, and navigation
4. libraries, directories, and profiles
5. feed and notifications
6. search and answer flows
7. learning and video
8. Google Workspace coordination
9. privacy, connectors, relay, and governance

Do not polish late-stage surfaces while earlier milestones remain structurally broken.

## Storage And Sync Rules

- Use immutable revisions for published long-form content.
- Use user-scoped drafts for in-progress page edits.
- Use append-only JSONL segments for high-churn events such as feed activity and analytics.
- Preserve conflict copies; surface merge flows explicitly.
- Track Drive roots by durable identity, not guessed filesystem paths.
- Keep local caches outside the Drive workspace.
- Design for mirrored My Drive, streamed My Drive, and streamed shared-drive behavior.

If a design choice would make the product harder to reconstruct from canonical files alone, reject it.

## Google Integration Rules

- Use direct APIs or local state for latency-sensitive paths.
- Use `gws` for control-plane flows, command shadows, recipes, exports, delivery, and operator tooling.
- Every meaningful Google-side action needs a machine-readable shadow that an operator or agent can inspect.
- Request scopes progressively.
- Store credentials in OS-backed secure storage where possible.
- Keep Google satellites linked back to canonical WEAVE objects.

## Agent Behavior Inside The Repo

Long-running agents working here must:

- keep a running notion of the current milestone and next unblocked slice
- verify assumptions against the codebase before editing
- reread the relevant `SPEC.md` section when switching subsystems
- leave visible traces in docs, fixtures, tests, or scripts for non-trivial new behavior
- prefer additive migrations and stable file format changes

Long-running agents must not:

- leave half-migrated file formats without readers and repair paths
- silently break the demo workspace
- add bespoke one-off scripts when a reusable script or crate is the right abstraction
- widen permissions or scopes "just to make it work"

## Quality Gates

Run relevant checks before every commit. Minimum expectations:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features`
- `pnpm lint`
- `pnpm typecheck`
- `pnpm test`
- `pnpm test:e2e`

When the full matrix is too expensive for a tiny change, run the narrowest defensible subset first, but restore the full matrix regularly and before milestone completion.

Visual work also requires:

- screenshot or golden test updates
- dark-mode checks
- mobile breakpoint checks

## Security Gates

Run security checks regularly, especially before merging auth, relay, Google integration, export, or dependency changes.

Minimum expectations:

- `cargo deny check`
- `cargo audit`
- `pnpm audit --audit-level high`
- `gitleaks detect --source . --no-git`

Security defaults:

- never log secrets
- redact sensitive identifiers in debug output unless explicitly in a secure diagnostic mode
- enforce permission checks at tool and data boundaries, not only in the UI
- treat private-scope indexing and agent access as high-risk surfaces

## Commit Protocol

Commit frequently. A good cadence is one micro commit per coherent slice, refactor, or hardening pass.

Every commit should:

1. describe the technical scope clearly
2. use an Italo Calvino-inflected, mythopoetic image without becoming vague
3. correspond to passing checks for the touched surface

Preferred format:

`<scope>: <technical action> beneath a Calvino-inflected image`

Examples:

- `bootstrap: add Drive root picker beneath a winter traveler sky`
- `sync: reconcile shared-drive deltas through invisible cities`
- `search: wire citation anchors in a cosmicomic answer pane`
- `agents: trace Herald output in the castle of crossed tools`

Do not use poetic messages that hide what changed.

## Definition Of A Good Slice

A good implementation slice:

- changes one subsystem or one user-visible flow
- includes tests or fixtures
- keeps the app runnable
- updates docs when behavior changes
- can be explained in a short commit message

Bad slices:

- massive mixed refactors with no behavior boundary
- schema changes without migration or repair logic
- UI work with no tests or screenshots
- integration work with no dry-run or mock coverage

## Demo Workspace Rule

The demo workspace is not optional decoration. It is a living product fixture and acceptance surface.

- Do not break it casually.
- Extend it when new features land.
- Use it for screenshots, smoke tests, and acceptance checks.

## When Blocked

If blocked by missing credentials, unavailable APIs, or environment gaps:

- finish local contracts and mocks
- build dry-run flows
- leave activation notes in the right docs or scripts
- continue with the next unblocked slice

Only stop entirely when proceeding would require a destructive or irreversible guess.

## Handoff Standard

At the end of a work session, leave:

- committed code
- passing relevant checks
- updated docs if behavior changed
- a clear next slice implied by the repo state

The goal is continuity. Another strong engineer or agent should be able to resume work without reconstructing your intent from scratch.
