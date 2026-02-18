## 🔷 High-Level Architecture Diagram

```mermaid
flowchart TB
    User[User / Frontend]

    subgraph ControlPlane["Directory Canister (Control Plane)"]
        DirAccounts[User Accounts & Credits]
        DirMeta[File Metadata & Index]
        DirUploads[Upload Sessions]
        DirRouting[Bucket Routing]
    end

    subgraph DataPlane["Bucket Canisters (Data Plane)"]
        Bucket1[(Bucket A)]
        Bucket2[(Bucket B)]
        BucketN[(Bucket N)]
    end

    subgraph Ops["Operations"]
        Billing[Credit / Cycles Billing]
        Admin[Monitoring & Admin]
    end

    User -->|start_upload| DirUploads
    User -->|get_pricing / usage| DirAccounts
    User -->|commit_upload| DirUploads
    User -->|list_files| DirMeta

    DirRouting --> Bucket1
    DirRouting --> Bucket2
    DirRouting --> BucketN

    User -->|put_chunk (token)| Bucket1
    User -->|get_chunk| Bucket1

    Bucket1 -->|stat / usage| DirRouting

    Billing --> DirAccounts
    Admin --> DirRouting
```

## 🔷 Upload Sequence Diagram

```mermaid
sequenceDiagram
    participant U as User
    participant D as Directory
    participant B as Bucket

    U->>D: start_upload(size, metadata)
    D-->>U: upload_id + chunk_size

    U->>D: get_upload_token(upload_id)
    D-->>U: upload_token

    loop For each chunk
        U->>B: put_chunk(token, index, bytes)
        B-->>U: ok
        U->>D: mark_chunk_uploaded(index)
    end

    U->>D: commit_upload(upload_id)
    D-->>U: file Ready
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
