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

## 🛠️ Refinement

- [x] **Directory: `start_upload` field mapping**
  - `name` and `mime` are now correctly stored in the `UploadSession`.

## 🔐 Security Evolution

Tracked against [SECURITY_EVOLUTION.md](file:///Users/antonio.ventilii/projects/vault-core/SECURITY_EVOLUTION.md).

### 🛡️ Phase 2: Principal-Based Sharing (ACL)

- [x] **Directory: ACL Schema Implementation**
  - Add `readers` and `writers` (BTreeSet<Principal>) to `FileMeta`.
- [x] **Directory: Access Control Logic**
  - Update `get_file_meta`, `get_download_plan`, and `delete_file` to respect ACL.
- [x] **Directory: Management API**
  - `add_file_access(file_id, principal, role)`
  - `remove_file_access(file_id, principal)`

### 🔗 Phase 3: Link Sharing (Capability-Based)

- [x] **Directory: Link Token Generation**
  - Create secure, random 256-bit tokens for sharing.
- [x] **Directory: Link Management**
  - `create_share_link(file_id, ttl)`
  - `revoke_share_link(link_id)`
- [x] **Bucket: Capability Verification**
  - Logic to verify signed capability tokens (Phase 3 Option B).

### 🕵️ Phase 4: Privacy & Public Access

- [ ] **Directory/Bucket: Public Visibility**
  - Support for `visibility: Public` to allow unauthenticated access.
- [ ] **Client-Side Encryption Architecture**
  - Documentation/Tools for encrypting files before upload to ensure confidentiality.
