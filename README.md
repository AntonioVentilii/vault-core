# 🛡️ Vault Core

Vault Core is a high-performance, sharded file storage system built for the **Internet Computer**. It provides a secure and scalable foundation for decentralized file storage, featuring a modular architecture and an integrated billing system.

## 🔷 Key Features

- **Sharded Architecture**: Separates the control plane (**Directory**) from the data plane (**Buckets**) for massive horizontal scalability.
- **PAPI Billing Integration**: Native support for **Paid APIs (PAPI)**, allowing canisters to be self-sustaining via attached Cycles and support profit collection through ICRC-2 tokens.
- **Quota Management**: User-specific storage limits and usage tracking.
- **Rent/TTL Model**: Automated "Pay-As-You-Go" storage with a prepaid balance and transparent expiration tracking.
- **Garbage Collection**: Periodic automated cleanup of data from expired accounts via `canister_heartbeat`.
- **Modular Design**: Extensible codebase with clearly defined modules for configuration, memory management, and payments.
- **Stable Memory**: Leverages `ic-stable-structures` for robust data persistence across canister upgrades.

## 🔷 High-Level Architecture

Vault Core consists of two main canister types:

1.  [**Directory Canister**](file:///Users/antonio.ventilii/projects/vault-core/src/directory/README.md): The orchestrator. Manages user accounts, quotas, file metadata, expiration (TTL), and bucket routing.
2.  [**Bucket Canisters**](file:///Users/antonio.ventilii/projects/vault-core/src/bucket/README.md): The storage workers. Handle chunked file data and enforce payment for storage operations.

For a detailed look at the system design and interaction diagrams, see [ARCHITECTURE.md](file:///Users/antonio.ventilii/projects/vault-core/ARCHITECTURE.md).
For a deep dive into authorization, see [UPLOAD_TOKENS.md](file:///Users/antonio.ventilii/projects/vault-core/UPLOAD_TOKENS.md).
For the economic strategy, see [REVENUE_MODEL.md](file:///Users/antonio.ventilii/projects/vault-core/REVENUE_MODEL.md).

## 🔷 Getting Started

### Prerequisites

- [DFX](https://internetcomputer.org/docs/current/developer-docs/setup/install) (The IC SDK)
- [Rust](https://www.rust-lang.org/tools/install) (with `wasm32-unknown-unknown` target)

### Build

To compile all canisters in the project:

```bash
cargo build --target wasm32-unknown-unknown
```

### Deployment

Configure your canisters in `dfx.json` and deploy:

```bash
dfx deploy
```

> [!NOTE]
> During deployment, both the Directory and Bucket canisters require `InitArgs` to configure the ledger principals for PAPI billing.

## 🔷 Project Structure

```text
vault-core/
├── src/
│   ├── directory/    # Control Plane logic
│   ├── bucket/       # Data Plane storage logic
│   └── shared/       # Common types and utilities
├── ARCHITECTURE.md   # System design and diagrams
├── SPECS_V2.md       # Detailed PAPI integration specs
├── SECURITY_EVOLUTION.md # Future security roadmap
└── Cargo.toml        # Workspace configuration
```

## 🔷 Usage & Examples

For a step-by-step guide on how to use Vault Core with `dfx`, including payment details, see [USAGE.md](file:///Users/antonio.ventilii/projects/vault-core/USAGE.md).

### Quick Demo

You can run a basic upload flow demo using the provided script:

```bash
./scripts/demo_upload.sh
```

### Real File Upload

To upload an actual file from your local filesystem:

```bash
./scripts/upload_file.sh <path_to_file>
```

## 🔷 Testing

Vault Core features a robust integration test suite powered by `pocket-ic`.

```bash
./scripts/test-integration.sh
```

For more details on the testing infrastructure, cross-canister flows, and how to contribute, see [HACKING.md](file:///Users/antonio.ventilii/projects/vault-core/HACKING.md).

## 🔷 Billing Model (PAPI)

Vault Core uses the **PAPI** library to manage sustaining costs and service fees.

- **Control Plane Fees**: Managed in `src/directory/src/payments.rs`.
- **Data Plane Fees**: Managed in `src/bucket/src/payments.rs`.

Users can pay using direct **Attached Cycles** or via **ICRC-2 Tokens** (e.g., ckBTC, ckETH).

---

_Built with ❤️ on the Internet Computer._
