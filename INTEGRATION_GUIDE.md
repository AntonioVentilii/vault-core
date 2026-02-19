# 🚀 Vault Core: Ultimate Integration Guide

This document is a technical blueprint designed to allow a developer (or an AI assistant) to fully implement the frontend integration for Vault Core.

---

## 1. Authentication & Principal

The frontend must provide an **Identity** to the `@dfinity/agent`.

- **Key Idea**: All calls are signed. The `Principal` of the identity is used as the `owner` of files and the `UserId` for access control.

---

## 2. Payment Models (ICRC-2)

Vault Core uses `ic_papi` for payments. Before any paid call, you must perform an `icrc2_approve`.

### Pricing Reference (Human Readable)

| Action           | Amount (ckUSDC) | Amount (ICP) | atomic units (ckUSDC) | atomic units (ICP) |
| :--------------- | :-------------- | :----------- | :-------------------- | :----------------- |
| **Start Upload** | 0.1             | 0.01         | 100,000               | 1,000,000          |
| **Put Chunk**    | 0.03            | 0.003        | 30,000                | 300,000            |

### PaymentType Variant

When calling methods that require payment, use this structure:

```typescript
type PaymentType =
	| { CallerPaysIcrc2Tokens: { ledger: Principal } }
	| { AttachedCycles: null }
	| { CallerPaysIcrc2Cycles: null };
```

---

## 3. The Upload Protocol (Step-by-Step)

### Phase 1: Initiation

1.  **Select Ledger**: Decide if user pays in ICP or ckUSDC.
2.  **Approve Fee**: `ledger.icrc2_approve({ spender: DIRECTORY_ID, amount: 100_000n })`.
3.  **Call**: `directory.start_upload(filename, mime, total_bytes, { CallerPaysIcrc2Tokens: { ledger: LEDGER_ID } })`.
4.  **Save**: The returned `UploadSession` contains `upload_id` and `file_id`.

### Phase 2: Concurrent Chunking

1.  **Divide**: Slice file into 1MB chunks (1,048,576 bytes).
2.  **Request Tokens**: `directory.get_upload_tokens(upload_id, [0, 1, 2...])`. Large files should request tokens in batches of 10-20.
3.  **Upload to Bucket**:
    - For each chunk index `i`:
    - **Approve**: `ledger.icrc2_approve({ spender: bucket_id, amount: 30_000n })`.
    - **Call**: `bucket.put_chunk(upload_token, i, data_blob, payment_type)`.
    - _Retry on failure_: Chunks are idempotent.

### Phase 3: Completion

1.  **Call**: `directory.commit_upload(upload_id)`.
2.  **Verification**: The call fails if any chunk is missing.

---

## 4. Download Protocol

1.  **Call**: `directory.get_download_plan(file_id)`.
2.  **Parse**: The `DownloadPlan` gives you a list of `ChunkLocation` (which bucket has which chunk) and signed `DownloadToken`.
3.  **Fetch**: Call `bucket.get_chunk(token, chunk_index)` for each chunk.
4.  **Reassemble**: Concatenate blobs in order to recreate the file.

---

## 5. Sharing & Permissions

### Public Links

- `create_share_link(file_id, ttl)` -> returns a `token` (blob).
- Distribute the URL as `https://<frontend-id>.icp0.io/#/share/<base64-token>`.
- Recipient calls `resolve_share_link(token)` to get a `DownloadPlan`.

### Principal-based Sharing

- `add_file_access(file_id, principal, Role)` where Role is `{ Reader: null }` or `{ Writer: null }`.

---

## 6. Technical Type Reference (Candid)

### FileId

```typescript
type FileId = { id: Uint8Array; owner: Principal };
```

### FileMeta

```typescript
type FileMeta = {
	file_id: FileId;
	name: string;
	size_bytes: bigint;
	chunk_count: number;
	status: { Ready: null } | { Pending: null } | { Deleted: null };
	// ... (readers, writers, timestamps)
};
```

---

## 7. Error Handling

Common `DirectoryError` variants to catch:

- `AccountExpired`: User needs to `top_up_balance`.
- `QuotaExceeded`: User is trying to upload more than their limit.
- `PaymentFailed(string)`: Usually means `icrc2_transfer_from` failed (insufficient allowance or funds).
- `Unauthorized`: Caller doesn't own the file or have reader/writer access.
