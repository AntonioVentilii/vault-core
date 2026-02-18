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

## 🔷 Upload Sequence Diagram

```mermaid
sequenceDiagram
    participant U as User
    participant D as Directory
    participant B as Bucket

    U->>D: start_upload(size, payment)
    Note over D: PAPI: deduct(SignerMethods::StartUpload)
    D-->>U: upload_id + upload_token

    U->>D: get_upload_tokens(upload_id, chunks)
    D-->>U: upload_token (signed)

    loop For each chunk
        U->>B: put_chunk(token, chunk_index, bytes, payment)
        Note over B: PAPI: deduct(SignerMethods::PutChunk)
        B-->>U: size
        B-->>D: report_chunk_uploaded(upload_id, index)
    end

    U->>D: commit_upload(upload_id)
    D-->>U: FileMeta (Ready)
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

## 🔷 Visual Flow

```mermaid
sequenceDiagram
    participant User
    participant Directory
    participant Bucket
    participant Ledger

    Note over User, Ledger: 1. Payment Phase (ICRC-2 Tokens)
    User->>Ledger: icrc2_approve(Directory, amount)

    Note over User, Ledger: 2. Authorization Phase (Upload Tokens)
    User->>Directory: start_upload(file_info, payment_info)
    Directory->>Ledger: icrc2_transfer_from(User, amount)
    Directory-->>User: upload_session_id

    User->>Directory: get_upload_tokens(session_id)
    Directory-->>User: Signed UploadToken

    Note over User, Ledger: 3. Storage Phase
    User->>Bucket: put_chunk(UploadToken, data)
    Bucket->>Bucket: Verify Signature
    Bucket-->>User: Success
```
