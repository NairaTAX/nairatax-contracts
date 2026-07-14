# Security Policy

## Scope

This repo contains a Soroban smart contract that anchors report hashes on-chain. It holds no funds and no PII, but it does control a single admin-gated write path (`submit_attestation`); a bug that lets a non-admin write, overwrite, or forge an attestation is a security issue.

## Reporting a vulnerability

Please **do not** open a public GitHub issue for a suspected vulnerability. Instead, email the maintainers at the address listed in the NairaTax org profile, or reach out privately via the `nairatax-xyz` org contacts.

Include:

- A description of the issue and its potential impact
- Steps to reproduce, ideally as a failing test against `contracts/attestation`
- Any relevant network/deployment details, if the issue is Testnet/Mainnet-specific

We'll acknowledge reports within a few business days and follow up with a plan once the issue is confirmed.

## Supported versions

This contract has not yet been deployed to Testnet or Mainnet. Until a first version is deployed, only the code on `main` is in scope.
