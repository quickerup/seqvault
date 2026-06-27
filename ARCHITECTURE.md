```text
seqvault/
├── Cargo.toml
├── docker-compose.yml
├── Makefile
├── .env.example
├── core/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── emulator.rs
│       ├── merkle.rs
│       └── sequence.rs
├── api/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── server.rs
│       ├── config.rs
│       └── handlers/
│           ├── mod.rs
│           ├── simulate.rs
│           └── commit.rs
└── contracts/
    ├── package.json
    ├── tact.config.json
    ├── src/
    │   ├── seqvault_anchor.tact
    │   └── types.tact
    ├── scripts/
    │   └── deployAnchor.ts
    └── tests/
        └── SeqvaultAnchor.spec.ts

```
### Directory and File Descriptions
**Root Level**
 * **seqvault/**: The root directory containing the entire SeqVault workspace, unifying the Rust-based local API, the cryptographic core, and the TON smart contracts.
 * **Cargo.toml**: The Rust workspace manifest file that manages the api and core crates, ensuring unified dependency resolution and build processes.
 * **docker-compose.yml**: Defines the containerized environment for running the SeqVault API locally on target hardware (e.g., Raspberry Pi) alongside an optional lightweight local TON node or cache.
 * **Makefile**: Contains standard command aliases for building, testing, deploying contracts, and running the local API server.
 * **.env.example**: A template for environment variables required by the system, such as TON network endpoints, local host ports, and logging configurations.
**core/ Sub-project (Rust)**
 * **core/**: The standalone Rust library containing the heavy lifting for cryptography, state verification, and local simulation. Designed to be highly deterministic and hardware-agnostic.
 * **core/Cargo.toml**: The manifest for the core library, defining dependencies like ton_block, ton_types, and cryptographic primitives.
 * **core/src/**: The source code directory for the core library.
 * **core/src/lib.rs**: The primary entry point for the library, exporting modules for the API crate to consume.
 * **core/src/emulator.rs**: Wraps the local TON TVM (TON Virtual Machine) emulator to safely execute dry-runs of contract sequences in a locally sovereign sandbox without network broadcasts.
 * **core/src/merkle.rs**: Implements the logic to parse and verify TON Merkle proofs, ensuring the local sandbox state perfectly matches the on-chain reality without trusting a central RPC.
 * **core/src/sequence.rs**: Defines the data structures and serialization logic for multi-step transaction pipelines, managing the state transitions from step to step.
**api/ Sub-project (Rust)**
 * **api/**: The Rust-based (e.g., Axum or Actix) web server that exposes the local HTTP interface for developers to interact with the SeqVault engine.
 * **api/Cargo.toml**: The manifest for the API crate, defining dependencies like web frameworks, serialization (serde), and async runtimes (tokio).
 * **api/src/**: The source code directory for the API daemon.
 * **api/src/main.rs**: The binary entry point that bootstraps the async runtime, loads configurations, and starts the API listener.
 * **api/src/server.rs**: Configures the HTTP server, middleware (CORS, logging), and registers the routing table.
 * **api/src/config.rs**: Parses and validates the environment variables or configuration files needed for the API to run on local hardware.
 * **api/src/handlers/**: The directory containing the HTTP controller logic for the API endpoints.
 * **api/src/handlers/mod.rs**: The module declaration file linking all handler routes together.
 * **api/src/handlers/simulate.rs**: The endpoint controller that accepts a proposed sequence, invokes the core emulator, and returns the deterministic result (success or exact failure point).
 * **api/src/handlers/commit.rs**: The endpoint controller that packages a successfully simulated sequence, generates the necessary escrow payloads, and broadcasts it to the TON network via the anchor contract.
**contracts/ Sub-project (TON Blueprint / Tact)**
 * **contracts/**: The directory containing the on-chain components written in Tact, managed by the TON Blueprint framework.
 * **contracts/package.json**: Node.js package file containing Blueprint scripts, testing frameworks (Jest), and Tact compiler dependencies.
 * **contracts/tact.config.json**: The compiler configuration file specific to the Tact language, defining optimization and output settings.
 * **contracts/src/**: The source directory for the Tact smart contracts.
 * **contracts/src/seqvault_anchor.tact**: The core immutable on-chain smart contract. It utilizes Payment Channel escrow logic to hold funds and enforces all-or-nothing execution of the submitted sequence, guaranteeing a full revert if any step fails.
 * **contracts/src/types.tact**: Contains custom struct and message definitions (e.g., OpCodes, SequencePayloads) shared across the anchor contract logic.
 * **contracts/scripts/**: Directory for deployment and maintenance scripts.
 * **contracts/scripts/deployAnchor.ts**: A TypeScript script used to compile and deploy the seqvault_anchor contract to the TON mainnet or testnet.
 * **contracts/tests/**: Directory for local smart contract test suites.
 * **contracts/tests/SeqvaultAnchor.spec.ts**: The Jest test suite that uses Blueprint's local blockchain environment to rigorously verify the escrow, sequence execution, and failure-revert mechanics of the anchor contract.
 * 
