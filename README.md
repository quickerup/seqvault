# SeqVault Implementation Plan

This document outlines the roadmap to realize the vision of SeqVault as described in `ARCHITECTURE.md`.

## Phase 1: Foundation & Scaffolding (Current)
- [x] Establish Rust workspace with `core` and `api` crates.
- [x] Set up TON Blueprint project for Tact smart contracts.
- [x] Configure Docker environment and root-level automation (Makefile).
- [x] Basic API server structure with Axum.

## Phase 2: Core Cryptography & Logic
- [ ] **Sequence Logic (`core/src/sequence.rs`)**: Implement data structures for multi-step transactions and their serialization.
- [ ] **Merkle Verification (`core/src/merkle.rs`)**: Integrate `ton-types` or similar to parse and verify Merkle proofs from the TON network.
- [ ] **TVM Emulator (`core/src/emulator.rs`)**: Wrap a local TVM implementation to allow for local, sovereign simulation of transaction sequences.

## Phase 3: Smart Contract Development
- [ ] **Anchor Contract (`contracts/src/seqvault_anchor.tact`)**: Implement the escrow and all-or-nothing execution logic.
- [ ] **Type Definitions (`contracts/src/types.tact`)**: Finalize OpCodes and Payload structures.
- [ ] **Security Audit & Testing**: Rigorous testing using Blueprint's emulator and Jest.

## Phase 4: API & Integration
- [ ] **Simulation Handler (`api/src/handlers/simulate.rs`)**: Connect the API to the `core` emulator.
- [ ] **Commit Handler (`api/src/handlers/commit.rs`)**: Logic for packaging sequences and broadcasting to the TON network.
- [ ] **Configuration & Security**: Implement robust error handling and secure configuration management.

## Phase 5: Deployment & Hardware Optimization
- [ ] Optimize the `core` library for target hardware (e.g., Raspberry Pi).
- [ ] Deploy the `SeqvaultAnchor` to TON Testnet/Mainnet.
- [ ] Final end-to-end integration testing.
