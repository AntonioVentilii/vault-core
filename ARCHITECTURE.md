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

- **Directory (Control Plane)**: Enforces fees for metadata operations (e.g., starting an upload). Supports Cycles (direct or via Ledger) and ICRC-2 Tokens.
- **Bucket (Data Plane)**: Enforces "attached cycles" for data-heavy operations (`put_chunk`). This ensures that bucket canisters are refueled directly by the users, preventing resource exhaustion during large uploads.

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
