Below is a **v1 spec** for a scalable IC file-storage system using a **Directory + Bucket canister** design. It’s written so you can implement in Rust (ic-cdk + stable structures) or Motoko, and so a frontend can talk to it cleanly.

---

# IC File Storage Specs (Directory + Bucket)

## 1) Goals

- Store arbitrary user files on the IC with **chunked upload** (≤2 MiB per call).
- Scale storage by adding **bucket canisters** over time (no “canister explosion” per user).
- Make storage **paid by users** via PAPI (Cycles or ICRC-2 tokens).
- Keep bytes in **stable memory**, keep metadata/indexing small and efficient.
- Support resumable uploads, deletes, and basic sharing later.

Non-goals for v1:

- Content dedup across users, fancy folder semantics, end-to-end encryption (can be added).

---

## 2) Actors & Canisters

### A) Directory Canister (control plane)

Responsibilities:

- User accounts: quota, usage tracking.
- File metadata: name, mime, size, timestamps, status.
- Upload sessions: resumable state, chunk count, commit/abort.
- Chunk routing: which bucket stores each chunk.
- Bucket registry & capacity tracking.
- Pricing and billing (PAPI gatekeeper, fee calculation).

### B) Bucket Canisters (data plane)

Responsibilities:

- Store bytes for `(file_id, chunk_index)` in stable memory.
- Provide `put_chunk/get_chunk/delete_file` primitives.
- Track per-file byte usage for fast accounting.
- Optional lazy-GC / tombstones.

---

## 3) Core Constraints

- Upload/download must be **chunked** due to **ingress limits** (choose chunk size ≤ 1 MiB for safety).
- Bucket canisters keep bytes in **stable memory**; directory stores mostly metadata.
- Never depend on keeping whole files in heap.

---

## 4) Identifiers & Types

### 4.1 Identifiers

- `UserId = Principal`
- `FileId = record { owner: UserId; id: blob }`
  - `id` is random 16 bytes UUID-like, or hash-salted. Random is fine for v1.

- `UploadId = blob` (random 16–32 bytes)
- `BucketId = Principal`

### 4.2 File status

- `Pending` (upload in progress)
- `Ready` (committed)
- `Deleted` (tombstoned; bytes may be GC’d later)

### 4.3 Chunk size

- Directory provides `chunk_size` (default: 512 KiB or 1 MiB).
- Client must upload exactly `chunk_size` per chunk except last chunk.

### 4.4 Candid-like type sketch (informal)

```candid
type FileId = record { owner: principal; id: blob };

type FileStatus = variant { Pending; Ready; Deleted };

type FileMeta = record {
  file_id: FileId;
  name: text;
  mime: text;
  size_bytes: nat64;
  chunk_size: nat32;
  chunk_count: nat32;
  created_at_ns: nat64;
  updated_at_ns: nat64;
  status: FileStatus;
  // optional:
  sha256: opt blob; // 32 bytes
};

type UploadSession = record {
  upload_id: blob;
  file_id: FileId;
  chunk_size: nat32;
  expected_size_bytes: nat64;
  expected_chunk_count: nat32;
  uploaded_chunks: vec nat32; // or bitmap
  expires_at_ns: nat64;
  payment_info: opt PaymentInfo; // PAPI payment record
};
```

---

## 5) Billing Model (PAPI)

### 5.1 PAPI Integration

- The Directory canister uses the [PAPI library](https://github.com/dfinity/papi) to guard paid API methods.
- Users pay for operations (like `start_upload` or `put_chunk`) using:
  - **Attached Cycles**: To directly sustain the canister's operational costs.
  - **ICRC-2 Allowance**: To pay in tokens (ckBTC, ckUSDC, etc.) which can include a profit margin for the service provider.

### 5.2 Pricing Structure

Directory defines:

- `base_storage_fee`: Price in tokens per GiB/month.
- `cycle_fee_per_call`: Minimum cycles required to be attached to cover compute/storage overhead.
- `min_deposit`: Minimal amount to open an account or start a large upload.

**Sustainability Pattern:**

- Compute-heavy calls (like `put_chunk`) should require enough cycles to keep the canister "fuel tank" full.
- Value-added fees (profit) are collected in ICRC-2 tokens and stored in the canister's ledger account.

### 5.3 Transparency

Directory exposes:

- `get_pricing() -> Pricing`: Current fees and supported tokens.
- `estimate_upload_cost(bytes) -> { tokens: nat; cycles: nat }`: Predicted cost using PAPI fee structure.

---

## 6) Directory Canister API

### 6.1 Auth

- All methods are authenticated by `caller`.
- Owner-only for v1:
  - `caller == file_id.owner` required for upload, download pointers, delete.

- (Later) add ACL for sharing.

### 6.2 Public query methods

#### `get_pricing() -> Pricing`

Returns current pricing parameters and chunk size policy.

#### `get_usage(user: opt principal) -> Usage`

If `user` omitted, returns caller’s usage.
Includes `used_bytes`, `quota_bytes`, `files_count`, etc.

#### `list_files(cursor, limit) -> { files: vec FileMeta; next: opt blob }`

List caller’s files (Ready + optionally Pending).

#### `get_file_meta(file_id) -> opt FileMeta`

Owner-only.

#### `get_download_plan(file_id) -> DownloadPlan`

Owner-only; returns where to fetch each chunk (bucket + indices).
Alternative: return a compact plan for ranges.

`DownloadPlan` includes:

- `chunk_count`
- `chunk_size`
- `locations: vec record { chunk_index: nat32; bucket: principal }`
  - optionally compressed via ranges.

### 6.3 Update methods (uploads)

#### `start_upload(req: StartUploadReq) -> StartUploadRes`

Request:

- `name`, `mime`
- `size_bytes`
- `sha256: opt blob` (optional)
- `retention_days: nat32` (optional; default)
  Response:
- `upload_id`
- `file_id`
- `chunk_size`
- `expected_chunk_count`
- `expires_at_ns`
- optional `initial_routes` (see routing below)

Rules:

- Enforce quota & PAPI payment guard (Cycles/Tokens).
- Create `FileMeta(status=Pending)` and `UploadSession`.

Errors:

- `InsufficientCredit`
- `QuotaExceeded`
- `InvalidSize`
- `TooManyInProgressUploads`

#### `route_chunks(upload_id, chunk_indices: vec nat32) -> RouteChunksRes`

Returns where to upload those chunk indices:

- `routes: vec record { chunk_index; bucket: principal }`

Rules:

- Deterministic mapping allowed (hash-based), but must respect bucket capacity.
- Directory must ensure chosen bucket is “writable”.

(You can also return all routes in `start_upload` if you prefer.)

#### `commit_upload(req: CommitUploadReq) -> CommitUploadRes`

Request:

- `upload_id`
- `uploaded_chunk_count` (optional; or infer from session)
- `final_sha256` (optional)
  Rules:
- Verify all chunks present (directory’s session state).
- Mark file `Ready`, settle PAPI payment if needed, add to user `used_bytes`.

Errors:

- `UploadNotFound`
- `UploadIncomplete { missing: vec nat32 }`
- `HashMismatch` (if used)

#### `abort_upload(upload_id) -> ()`

- Marks file `Deleted` or removes Pending file entirely.
- Aborts PAPI payment session if applicable.

#### `delete_file(file_id) -> DeleteRes`

- Owner-only.
- Marks file `Deleted` in directory immediately.
- Schedules/initiates deletion in all relevant buckets (best-effort).
- Updates user usage either:
  - immediately (optimistic), or
  - after bucket confirms deletion (conservative).
    For v1, optimistic is fine with a “reconcile” job.

---

## 7) Bucket Canister API

Buckets should be **dumb** and fast.

### 7.1 Auth

- Only allow calls from:
  - Directory canister, **or**
  - Users presenting a short-lived **upload token** minted by Directory.

For v1, simplest is: **bucket trusts Directory only**.
Frontend never calls bucket directly; it always calls directory, and directory proxies chunks.
But proxying increases directory load.

Recommended v1.5: allow user -> bucket direct upload with directory-issued token.
For now, I’ll spec the token approach (it scales better).

### 7.2 Upload token (capability)

Directory mints:

- `UploadToken = record { upload_id; file_id; bucket_id; expires_at; allowed_chunks: vec nat32; sig: blob }`
  Bucket validates signature (or validates by calling Directory to verify token).
  Pick:
- **Signature verification** = faster, no cross-canister call.
- **Call Directory** = simpler but slower.

### 7.3 Bucket methods

#### `put_chunk(req: PutChunkReq) -> PutChunkRes`

Request:

- `token: UploadToken`
- `chunk_index: nat32`
- `bytes: blob`
  Rules:
- Validate token and chunk_index in allowed set.
- Enforce size constraints (chunk_size except last chunk if provided).
- Write bytes to stable memory keyed by `(file_id, chunk_index)`.
- Idempotent: if same chunk already exists, return success (or require exact same length/hash).

Response:

- `stored_bytes: nat32`
- `etag: opt blob` (optional hash for chunk)

Errors:

- `Unauthorised`
- `TokenExpired`
- `ChunkTooLarge`
- `BucketReadOnly`
- `OutOfSpace`

#### `get_chunk(req: GetChunkReq) -> blob`

Request:

- `file_id`
- `chunk_index`
- auth: owner token or directory authorisation (same pattern as upload).
  Rules:
- Return bytes.

#### `delete_file(req: DeleteFileReq) -> DeleteFileRes`

- Called by Directory for cleanup/GC.
- Deletes all chunks for file (if you track per-file chunk list) or marks tombstone.

#### `stat() -> BucketStat`

- Used bytes, free bytes estimate, writable flag, version.

---

## 8) Routing & Sharding Policy

Directory maintains:

- `buckets: vec BucketInfo { id, writable, used_bytes, soft_limit_bytes, hard_limit_bytes }`

Routing rules:

- Prefer a bucket with `used_bytes + incoming <= soft_limit`.
- If none, create or activate a new bucket (see “Provisioning”).
- Optionally keep a file entirely in one bucket for simplicity.
  - This is recommended for v1: **one file → one bucket**.

So:

- `bucket_for_file(file_id) = assigned bucket`
- all chunks go to that bucket.

This keeps the directory’s chunk map tiny:

- store only `file_id -> bucket_id` (instead of per-chunk mapping).

---

## 9) Bucket Provisioning & “Exploding Canisters” Prevention

### 9.1 When to add a new bucket

- If active bucket reaches `soft_limit` (e.g. 80–90% target), directory marks it “draining” and assigns new files to a new bucket.
- If `hard_limit` reached, set bucket `read_only`.

### 9.2 How to create a bucket

- Directory can call the management canister to create/install a new bucket canister (needs cycles).
- Maintain a “bucket pool” (pre-created buckets) if you want to avoid user-facing latency.

### 9.3 Failure modes

- If directory cannot provision a bucket due to low cycles, return:
  - `ServiceTemporarilyUnavailable` with explanation.

- Never accept an upload you cannot store.

---

## 10) State & Storage Layout

### 10.1 Directory stable state

- `users: StableBTreeMap<UserId, UserState>`
- `files: StableBTreeMap<FileId, FileMeta>`
- `uploads: StableBTreeMap<UploadId, UploadSession>`
- `file_to_bucket: StableBTreeMap<FileId, BucketId>`
- `buckets: StableVec<BucketInfo>` (or map)

### 10.2 Bucket stable state (two approaches)

**A) Key-value chunks**

- `chunks: StableBTreeMap<(FileId, chunk_index), blob_ref>`
- `blob_ref` points to stable-memory region / segmented allocator.
- Maintain `file_index: StableBTreeMap<FileId, vec chunk_index>` for delete speed.

**B) Append-only log + index**

- Append chunk bytes to stable memory.
- Store offset/len in map.
- Tombstone on delete; periodic compaction later.

For v1, A is conceptually simpler if you have a stable allocator strategy.

---

## 11) Upload Lifecycle (exact behaviour)

1. `start_upload(size_bytes, ...)`
   - directory reserves credit
   - creates `file_id`, `upload_id`, status Pending
   - assigns bucket for file

2. Client requests token(s):
   - `get_upload_token(upload_id, allowed_chunks_range)`

3. Client calls bucket `put_chunk(token, idx, bytes)` for each chunk
4. Client notifies directory:
   - either per chunk: `mark_chunk_uploaded(upload_id, idx, etag?)`
   - or at end: directory queries bucket for chunk presence (more expensive)
   - **Recommended:** client calls `mark_chunk_uploaded` after each success.

5. `commit_upload(upload_id)`
   - directory verifies all chunks marked uploaded
   - optionally verifies hashes
   - status Ready, settle billing, update usage

Expiry:

- Directory has an `expires_at_ns` on upload sessions.
- A periodic maintenance call (manual cron or external) runs `reap_expired_uploads()`:
  - abort session
  - ask bucket to delete partial chunks (best-effort)

---

## 12) Errors (common variants)

Directory errors:

- `Unauthorised`
- `NotFound`
- `QuotaExceeded { requested; available }`
- `InsufficientCredit { required; available }`
- `InvalidRequest { reason }`
- `UploadExpired`
- `UploadIncomplete { missing: vec nat32 }`
- `InternalError { code }`

Bucket errors:

- `Unauthorised`
- `TokenExpired`
- `ChunkTooLarge`
- `OutOfSpace`
- `ReadOnly`
- `NotFound`

---

## 13) Security & Abuse Controls (v1 must-haves)

- Rate limit:
  - `start_upload` per user (e.g. 5/min)
  - `put_chunk` via token issuance limits

- Require PAPI payment to start upload (prevents free partial uploads).
- Token expiry short (e.g. 5–15 minutes).
- Cap in-progress uploads per user.
- Avoid directory proxying large blobs if possible (token direct to bucket).

---

## 14) Observability & Admin

Directory:

- `admin_get_stats()`: total bytes, total files, buckets count, active uploads
- `admin_set_pricing()`
- `admin_set_quota(user, ...)` (optional)

Buckets:

- `stat()` returns usage + read-only flag
- `set_read_only(bool)` admin-only (directory)

---

## 15) Minimal “Frontend Contract”

Frontend only needs:

- `get_pricing/get_balance/get_usage`
- `start_upload`
- `get_upload_token` (or `route_chunks` + `get_upload_token`)
- `mark_chunk_uploaded` (if using it)
- `commit_upload`
- `get_download_plan`
- `delete_file`

Everything else can be hidden.
