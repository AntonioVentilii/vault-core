# TODO: SPECS_V2 Implementation Gaps

This file tracks the missing features and improvements required to fully satisfy the `SPECS_V2.md` specification.

## 🔍 Missing API Methods

- [x] **Directory: `get_download_plan(file_id)`**
- [x] **Directory: `estimate_upload_cost(bytes)`**
- [x] **Directory: `get_file_meta(file_id)`**
- [x] **Directory: `abort_upload(upload_id)`**
- [x] **Directory: `get_pricing()`**
  - Updated to return structured `PricingConfig`.

## ⚙️ Administrative & Management

- [x] **Directory: `admin_withdraw(ledger, amount, to)`**
- [x] **Bucket: `admin_withdraw(ledger, amount, to)`**
- [ ] **Directory: Pagination for `list_files`**
  - Implement `cursor` and `limit` support.
- [x] **Directory: `admin_set_pricing(...)`**
- [x] **Directory: `admin_set_quota(user, ...)`**
- [x] **Directory: `garbage_collect()`**
  - Implemented for user account expiration.
- [x] **Directory: `reap_expired_uploads()`**
  - Specific cleanup for stale `Pending` uploads (sessions).
- [x] **Bucket: `set_read_only(bool)`**
  - Administrative control over bucket writability.

## 🛡️ Validation & Reliability

- [ ] **Directory/Bucket: Capacity Guarding**
  - Enforce `soft_limit` and `hard_limit` on buckets during upload.
- [ ] **Directory: Automated Provisioning**
  - Logic to automatically create new buckets when capacity is reached.
- [ ] **Directory: Integrity Checks**
  - Verify `sha256` hashes during `commit_upload`.
- [ ] **Bucket: Idempotency**
  - Ensure `put_chunk` is idempotent (return success if chunk already exists and matches).

## 🛡️ Security & Abuse Controls

- [ ] **Rate Limiting**
  - Implement limits for `start_upload` (e.g., 5/min per user).
  - Implement token issuance limits to prevent DDoS on buckets.
- [ ] **DDoS Protection**
  - Basic request throttling for expensive operations.

## 🛠️ Refinement & Pagination

- [ ] **Directory: Pagination for `list_files`**
  - Implement `cursor` and `limit` support for efficient listing of large file sets.
- [ ] **Directory/Bucket: Resumable Uploads**
  - Improve session recovery and partial upload tracking.

## 🕵️ Future Evolution: Privacy & Public Access

Tracked against [SECURITY_EVOLUTION.md](file:///Users/antonio.ventilii/projects/vault-core/SECURITY_EVOLUTION.md).

- [ ] **Directory/Bucket: Public Visibility**
  - Support for `visibility: Public` to allow unauthenticated access.
- [ ] **Client-Side Encryption Architecture**
  - Documentation/Tools for encrypting files before upload to ensure confidentiality.
- [ ] **Asset Canister Integration**
  - Compatibility with standard IC asset canister patterns for frontend integration.
