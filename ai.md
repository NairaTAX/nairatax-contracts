# AI Agent Guide — NairaTax Organization

Cross-repo map for AI coding agents working across the NairaTax organization. Each repo's own README covers its repo-local conventions; this file is the source of truth for how the repos fit together.

## Repositories

| Repo | Role | URL |
|---|---|---|
| `nairatax-xyz` | Org landing, roadmap, project board | https://github.com/nairatax-xyz/nairatax-xyz |
| `nairatax-engine` | Ledger ingestion, event classification, FIFO cost basis, report generation | https://github.com/nairatax-xyz/nairatax-engine |
| `nairatax-web` | Frontend — dashboard, review/tag UI, reports, export | https://github.com/nairatax-xyz/nairatax-web |
| `nairatax-rules` | Jurisdiction rule packs as auditable data (e.g. `nigeria-nta-2025`) | https://github.com/nairatax-xyz/nairatax-rules |
| `nairatax-contracts` | On-chain attestation layer for report hashes (this repo) | https://github.com/nairatax-xyz/nairatax-contracts |
| `nairatax-docs` | Methodology, filing guides, developer + user docs | https://github.com/nairatax-xyz/nairatax-docs |

## Data flow

```
Stellar Horizon/RPC ──▶ nairatax-engine ──▶ report + report_hash
                                                   │
                              nairatax-rules ──────┤ (jurisdiction rules)
                                                   │
                                    ┌──────────────┴──────────────┐
                                    ▼                              ▼
                            nairatax-web                 nairatax-contracts
                          (renders report)         (anchors report_hash on-chain)
```

## Shared contracts (must stay in sync across repos)

1. **Report hash format** — defined by `nairatax-engine`'s report serialization. Whatever canonical form `nairatax-engine` hashes must match exactly what `submit_attestation` receives in `nairatax-contracts`, or verification is meaningless.
2. **`Attestation` schema** — defined in `nairatax-contracts/contracts/attestation/src/types.rs`. If it changes, update `nairatax-web` client code that reads `get_attestation` results.
3. **Environment variables / config keys** — a deployed contract ID and a service account authorised to call `submit_attestation`, mirrored between `nairatax-engine`'s config and this repo's deployment.

## Conventions for AI agents

- Treat this file as the source of truth for **cross-repo** contracts; each repo's own README covers repo-local conventions.
- If a change in one repo would affect a shared contract above, call it out explicitly so the other repo(s) can be updated to match — don't assume the other repo's maintainer will notice on their own.
- Don't scaffold speculative cross-repo integration code (e.g. a `nairatax-engine` client for `submit_attestation`) without confirming the target repo is ready for it.
