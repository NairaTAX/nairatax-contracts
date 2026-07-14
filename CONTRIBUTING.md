# Contributing

## Setup

This crate targets the Rust stable toolchain with the `wasm32v1-none` target, pinned in `rust-toolchain.toml`. `rustup` will install both automatically the first time you run `cargo build` in this repo.

## Workflow

```bash
make test    # cargo test
make check   # fmt-check + clippy + test — what CI runs
make build   # optimized wasm via scripts/build.sh
```

Run `make check` before pushing; it's the same sequence `.github/workflows/ci.yml` runs, so a clean local run means CI should pass too.

## Code style

- Formatting is enforced by `rustfmt.toml`; run `make fmt` to apply it.
- Clippy warnings are treated as errors in CI (`-D warnings`).
- Keep doc comments (`///`) on public contract functions, types, and errors — they're part of the contract's interface documentation.

## Contract changes

`submit_attestation` and `get_attestation` are consumed by `nairatax-engine` and `nairatax-web` in sibling repos. If a change alters the `Attestation` struct, the `AttestationSubmitted` event shape, or an error code, call that out in the PR description so the matching client code can be updated.

New contract logic should ship with unit tests in `contracts/attestation/src/test.rs` covering both the success path and the failure modes it introduces.

## Commit messages

This repo uses Conventional Commits (`feat:`, `fix:`, `test:`, `docs:`, `chore:`, `ci:`, `refactor:`) with a short imperative summary line. Keep commits atomic — one logical change per commit.
