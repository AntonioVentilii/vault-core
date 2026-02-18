# TODO: SPECS_V2 Implementation Gaps

This file tracks the missing features and improvements required to fully satisfy the `SPECS_V2.md` specification.

## 🔍 Missing API Methods

- [x] **Directory: `get_download_plan(file_id)`**
- [ ] **Directory: `estimate_upload_cost(bytes)`**
- [x] **Directory: `get_file_meta(file_id)`**
- [x] **Directory: `abort_upload(upload_id)`**
- [ ] **Directory: `get_pricing()`**
  - Needs to return structured pricing data instead of a static string.

## ⚙️ Administrative & Management

- [ ] **Directory: Pagination for `list_files`**
  - Implement `cursor` and `limit` support.
- [ ] **Directory: `admin_set_pricing(...)`**
- [ ] **Directory: `admin_set_quota(user, ...)`**
- [ ] **Directory: `reap_expired_uploads()`**
  - Automatic cleanup of stale `Pending` uploads.
- [ ] **Bucket: `set_read_only(bool)`**
  - Administrative control over bucket writability.

## 🛡️ Validation & Reliability

- [ ] **Directory/Bucket: Capacity Guarding**
  - Enforce `soft_limit` and `hard_limit` on buckets during upload.
- [ ] **Directory: Automated Provisioning**
  - Logic to automatically create new buckets when capacity is reached.
- [ ] **Directory: Integrity Checks**
  - Verify `sha256` hashes during `commit_upload`.

## 🛠️ Refinement

- [ ] **Directory: `start_upload` field mapping**
  - Store the `name` and `mime` type in the `UploadSession` to correctly populate `FileMeta` on commit.
