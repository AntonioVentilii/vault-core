## 🔷 High-Level Architecture Diagram

```mermaid
flowchart TB
    User[User / Frontend]

    subgraph ControlPlane["Directory Canister (Control Plane)"]
        DirAccounts[User Accounts & Quotas]
        DirMeta[File Metadata & Index]
        DirUploads[Upload Sessions]
        DirRouting[Bucket Routing]
        DirPAPI[PAPI PaymentGuard]
    end

    subgraph DataPlane["Bucket Canisters (Data Plane)"]
        Bucket1[(Bucket A)]
        Bucket2[(Bucket B)]
        BucketPAPI[PAPI PaymentGuard]
    end

    subgraph External["External Ecosystem"]
        CyclesLedger[Cycles Ledger]
        TokenLedger[ICRC-2 Token Ledgers]
    end

    User -->|start_upload + payment| DirPAPI
    DirPAPI --> DirUploads
    User -->|get_pricing| DirPAPI
    User -->|commit_upload| DirUploads
    User -->|list_files| DirMeta

    DirRouting --> Bucket1
    DirRouting --> Bucket2

    User -->|put_chunk + payment| BucketPAPI
    BucketPAPI --> Bucket1
    User -->|get_chunk| Bucket1

    Bucket1 -->|report_chunk| DirUploads

    DirPAPI --> CyclesLedger
    DirPAPI --> TokenLedger
    BucketPAPI -->|Attached Cycles| Bucket1
```

## 🔷 Payment Logic (PAPI)

Payment logic is modularised into `payments.rs` in each canister, using the **PAPI (Paid APIs)** library.

- **Directory (Control Plane)**: Enforces fees for metadata operations and manages the **Rent Model**. It tracks a user's `expires_at_ns` and `prepaid_balance`.
- **Bucket (Data Plane)**: Enforces storage fees for `put_chunk`. Now supports both **Attached Cycles** and **ICRC-2 Tokens** (ICP/ckUSDC), ensuring that bucket canisters are refueled directly by the users.

### Rent Model & Garbage Collection

The Directory canister uses a `canister_heartbeat` to periodically run a `garbage_collect` task.

- **Expiry**: When `time() > expires_at_ns + 30 days`, the user's account and all associated files are deleted.
- **Top-Up**: Users can extend their expiration by calling `top_up_balance`.

## 🔷 Upload Sequence Diagram (Complete Flow)

```mermaid
sequenceDiagram
    participant U as User
    participant L as Ledger (ICRC-2)
    participant D as Directory (Control)
    participant B as Bucket (Data)

    Note over U, L: 1. Payment Phase
    U->>L: icrc2_approve(Spender: Directory, Amount)
    L-->>U: Allowance Created

    Note over U, D: 2. Authorization Phase
    U->>D: start_upload(Size, PaymentInfo)
    Note right of D: Check if Account Expired
    D->>L: icrc2_transfer_from(User, Amount)
    D-->>U: upload_id

    U->>D: get_upload_tokens(upload_id, [chunk_indices])
    Note right of D: SignerMethods::IssueToken
    D-->>U: Vec<UploadToken> (Signed)

    Note over U, B: 3. Storage Phase
    loop For each chunk
        U->>B: put_chunk(token, index, bytes, cycles)
        Note right of B: Verify HMAC Signature
        B-->>U: success
        B-->>D: report_chunk_uploaded(upload_id, index)
    end

    Note over U, D: 4. Finalization
    U->>D: commit_upload(upload_id)
    D-->>U: FileMeta (Success)
```

## 🔷 Bucket Provisioning Logic (Shard Growth)

```mermaid
flowchart LR
    A[Active Bucket]
    B{Used > Soft Limit?}
    C[Mark Bucket Draining]
    D[Create New Bucket]
    E[Route New Files to New Bucket]

    A --> B
    B -- No --> A
    B -- Yes --> C --> D --> E
```

## 🔷 Testing Infrastructure

The system's integrity is verified via a centralized integration test suite using `PocketIC`. These tests simulate the entire sharded environment, including the inter-canister interactions between the Directory and Buckets, as well as the cycle-based billing logic.

For technical details on the test architecture, see [HACKING.md](file:///Users/antonio.ventilii/projects/vault-core/HACKING.md).
