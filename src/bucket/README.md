# 🪣 Bucket Canister (Data Plane)

The Bucket canister is a high-performance storage shard in the Vault Core ecosystem. It is designed to store and serve chunked file data efficiently.

## 🔷 Core Responsibilities

- **Data Persistence**: Stores file chunks in stable structures to survive canister upgrades.
- **Access Control**: Verifies `UploadToken` signatures before allowing data writes.
- **Resource Sustainability**: Uses PAPI to enforce that users attach cycles to storage-heavy operations (e.g., `put_chunk`), ensuring the canister remains fueled as data grows.
- **Reporting**: Reports successful chunk uploads back to the Directory canister for metadata synchronization.

## 🔷 Key Modules

- [`api.rs`](file:///Users/antonio.ventilii/projects/vault-core/src/bucket/src/api.rs): Methods for putting, getting, and deleting chunks.
- [`payments.rs`](file:///Users/antonio.ventilii/projects/vault-core/src/bucket/src/payments.rs): PAPI configuration for attached cycles.
- [`memory.rs`](file:///Users/antonio.ventilii/projects/vault-core/src/bucket/src/memory.rs): Stable storage for large file chunks.

## 🔷 Architecture

This canister represents the **Data Plane** of Vault Core. It is horizontally scalable; multiple bucket canisters can be deployed as the system grows. For more details on how buckets are provisioned and utilized, see [ARCHITECTURE.md](file:///Users/antonio.ventilii/projects/vault-core/ARCHITECTURE.md).

---

_Back to [Vault Core README](file:///Users/antonio.ventilii/projects/vault-core/README.md)_
