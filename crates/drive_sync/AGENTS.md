# Drive Sync Agents

This subtree owns local watch handling, Drive reconciliation, conflict surfacing, and sync health.

- Preserve idempotence in replay and local watch application.
- Prefer explicit repair paths over silent data loss.
- Never make cache state canonical.
- Shared-drive and mirrored My Drive behavior both matter here.
- Any change that affects merge, replay, or conflict handling must add or update tests.
