# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `init(admin)` — sets the sole account authorised to submit attestations; callable once.
- `submit_attestation(submitter, report_hash, account, period)` — anchors a report hash on-chain, admin-only, append-only (rejects duplicate hashes).
- `get_attestation(report_hash)` — public read-only lookup of an anchored attestation.
- `AttestationSubmitted` event, topic-indexed by `report_hash`, published on every successful submission.
- Unit test coverage for `init`, `submit_attestation`, and `get_attestation`, including auth, duplicate, and pre-init failure modes.
- `contractmeta!` description for on-chain contract discoverability.
- CI (GitHub Actions): build/test, fmt-check, clippy, and a wasm build artifact job.
- `scripts/build.sh` and `scripts/deploy.sh` for local and CI wasm builds and Testnet/Mainnet deploys.

### Not yet done

- Not deployed to Testnet or Mainnet.
- Not yet wired up as a `nairatax-engine` client.
