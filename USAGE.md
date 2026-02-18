# 🚀 Usage Guide

This guide demonstrates how to interact with Vault Core using `dfx`. We will walk through a complete file upload flow, including PAPI payments.

## 🔷 1. Provision a Bucket

Before uploading, the Directory needs to know about a writable bucket.

```bash
# Get the principal of your bucket canister
BUCKET_ID=$(dfx canister id bucket)

# Register it in the directory
dfx canister call directory provision_bucket "(principal \"$BUCKET_ID\")"
```

## 🔷 2. Start an Upload

Initiate an upload session by specifying the file name, size, and payment method.

```bash
# payment: opt variant { AttachedCycles }
dfx canister call directory start_upload '("my_file.dat", 1048576, opt variant { AttachedCycles })' --with-cycles 1000000
```

_Note: The `--with-cycles` matches the `SignerMethods::StartUpload` fee (1M cycles)._

## 🔷 3. Get Upload Tokens

Retrieve a signed token that allows you to write to a specific bucket.

```bash
# Replace UPLOAD_ID_BLOB with the ID returned by start_upload
# chunks: vec { 0 }
dfx canister call directory get_upload_tokens '(blob "...", vec { 0 })'
```

## 🔷 4. Upload a Chunk

Send the data chunk directly to the bucket canister, attaching cycles for the storage cost.

```bash
# Replace TOKEN with the token returned in step 3
# chunk_index: 0
# bytes: blob "..."
# payment: opt variant { AttachedCycles }
dfx canister call bucket put_chunk '(record { ... }, 0, blob "...", opt variant { AttachedCycles })' --with-cycles 50000
```

_Note: The `--with-cycles` matches the `SignerMethods::PutChunk` fee (50k cycles)._

## 🔷 5. Commit the Upload

Finalize the upload in the Directory.

```bash
dfx canister call directory commit_upload '(blob "...")'
```

## 🔷 6. Verify

Check your file list and usage.

```bash
dfx canister call directory list_files '()'
dfx canister call directory get_usage '(null)'
```

---

_Back to [Vault Core README](file:///Users/antonio.ventilii/projects/vault-core/README.md)_
