# 📂 Directory Canister (Control Plane)

The Directory canister acts as the orchestrator of the Vault Core system. It is responsible for managing user state, file indexing, and coordinating bucket assignments.

## 🔷 Core Responsibilities

- **User Accounts**: Manages storage quotas, prepaid balances, and account expiration.
- **Rent Management**: Tracks the TTL (Time-to-Live) of user accounts and implements the **Rent Model**.
- **Garbage Collection**: Automatically cleans up data from accounts that remain expired beyond the grace period.
- **File Metadata**: Stores an index of all files, including their size, status, and associated buckets.
- **Upload Coordination**: Manages upload sessions and issues signed `UploadToken`s to the frontend.
- **Bucket Routing**: Maps files to specific bucket canisters and handles shard growth.
- **PAPI Implementation**: Enforces payment for metadata operations and top-ups via the `PAYMENT_GUARD`.

## 🔷 Key Modules

- [`api.rs`](file:///Users/antonio.ventilii/projects/vault-core/src/directory/src/api.rs): Public canister methods.
- [`payments.rs`](file:///Users/antonio.ventilii/projects/vault-core/src/directory/src/payments.rs): PAPI configuration and method fees.
- [`memory.rs`](file:///Users/antonio.ventilii/projects/vault-core/src/directory/src/memory.rs): Stable storage definitions and helper functions.
- [`config.rs`](file:///Users/antonio.ventilii/projects/vault-core/src/directory/src/config.rs): Canister initialization and upgrade arguments.

## 🔷 Architecture

This canister sits in the **Control Plane** of Vault Core. For a high-level view of how it interacts with the Buckets and the IC ecosystem, see the [High-Level Architecture](file:///Users/antonio.ventilii/projects/vault-core/ARCHITECTURE.md).

---

_Back to [Vault Core README](file:///Users/antonio.ventilii/projects/vault-core/README.md)_
