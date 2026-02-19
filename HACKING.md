# HACKING: Developer Guide for Vault Core

This document provides technical details for developers who want to contribute to the Vault Core project.

## 🧪 Testing Infrastructure

Vault Core uses `pocket-ic` for robust integration testing of the canister system.

### Running Tests

To run the integration tests, use the provided automation script. This script handles the installation of the `pocket-ic` server and sets up the required environment variables.

```bash
./scripts/test-integration.sh
```

The script will:

1.  **Build** the canisters in release mode.
2.  **Download** the correct version of the `pocket-ic` server to `target/pocket-ic` (using `scripts/pic-install`).
3.  **Run** the integration tests using `cargo test -p it`.

### Test Structure

The integration tests are centralized in the `tests/it` directory.

#### Why not put tests inside each canister?

While Rust supports `tests/` folders inside each crate, we use a centralized approach for several reasons:

1.  **Cross-Canister Flows**: Many tests (like `test_full_upload_flow`) involve complex interactions between the **Directory** and **Bucket** canisters. A centralized crate allows testing these flows without creating circular dependencies between the individual canister crates.
2.  **Shared Infrastructure**: By centralizing the tests, we share the same `PicCanister` pattern, `TestSetup` orchestrated environment, and the `test_proxy` for handling cycle-attached calls.
3.  **Wasm Build Management**: Integration tests run against the compiled `.wasm` files. Centralizing the tests makes it easier to manage and locate build artifacts from the `target/` directory.

### Advanced PocketIC Pattern

We utilize a robust pattern inspired by the `dfinity/papi` repository:

- **`PicCanisterTrait`**: A trait that provides a standardized interface for sending `update` and `query` calls to canisters, handling Candid encoding and result mapping.
- **`PicCanisterBuilder`**: A flexible builder for deploying canisters with specific Wasm files, initialization arguments, and cycles.
- **`TestSetup`**: A struct that wraps the `PocketIc` instance and handles the deployment of the entire canister ecosystem (`directory`, `bucket`, and `proxy`).

### Handling Cycles in Tests

Ingress calls from a test harness cannot carry cycles on the IC. To verify methods protected by `PaymentGuard` (which requires attached cycles), we use a **Proxy Canister** (`tests/it/proxy`).

Calls made through `PicCanister::update_with_cycles` are routed through this proxy, which attaches the requested amount of cycles to the actual canister call.

## 🔷 Code Style & Quality

Before submitting changes, please ensure your code passes linting and formatting checks.

### Prerequisites

```bash
npm ci
```

### Formatting

To automatically format the code (both Rust and other files):

```bash
npm run format
```

### Linting

To check for formatting and common pitfalls:

```bash
npm run lint
```
