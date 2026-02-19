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

Initiate an upload session by specifying the file name, size, and payment method. The system will check if your account is expired.

```bash
# payment: opt variant { AttachedCycles }
dfx canister call directory start_upload '("my_file.dat", 1048576, opt variant { AttachedCycles })' --with-cycles 1000000
```

_Note: The `--with-cycles` matches the `SignerMethods::StartUpload` fee (1M cycles / $0.10)._

## 🔷 3. Get Upload Tokens

Retrieve a signed token that allows you to write to a specific bucket.

```bash
# Replace UPLOAD_ID_BLOB with the ID returned by start_upload
# chunks: vec { 0 }
dfx canister call directory get_upload_tokens '(blob "...", vec { 0 })'
```

## 🔷 4. Upload a Chunk

Send the data chunk directly to the bucket canister, attaching cycles or tokens for the storage cost.

```bash
# Replace TOKEN with the token returned in step 3
# chunk_index: 0
# bytes: blob "..."
# payment: opt variant { AttachedCycles }
dfx canister call bucket put_chunk '(record { ... }, 0, blob "...", opt variant { AttachedCycles })' --with-cycles 30000
```

_Note: The fee is 30,000 units ($0.03) per chunk._

## 🔷 5. Top Up Account (Rent Model)

If your account is close to expiring, or you've been "frozen" due to zero balance, you can top up your expiration date.

```bash
# amount: 1_000_000 (e.g., $1.00 in USDC)
# payment: variant { CallerPaysIcrc2Tokens = record { ledger = principal "..." } }
dfx canister call directory top_up_balance '(1000000, variant { CallerPaysIcrc2Tokens = record { ledger = principal "xevnm-gaaaa-aaar-qafqa-cai" } })'
```

## 🔷 6. Admin: Withdraw Earnings

As a controller of the canister, you can withdraw the accumulated fees.

```bash
# ledger: Principal of the token ledger (e.g., ICP or ckUSDC)
# amount: Nat
# to: Principal of the destination wallet
dfx canister call directory admin_withdraw '(principal "ryjl3-tyaaa-aaaaa-aaaba-cai", 5000000000, principal "aaaaa-aa")'
```

## 🔷 7. Finalize and Verify

Finalize the upload in the Directory.

```bash
dfx canister call directory commit_upload '(blob "...")'
dfx canister call directory list_files '()'
dfx canister call directory get_usage '(null)'
```

---

_Back to [Vault Core README](file:///Users/antonio.ventilii/projects/vault-core/README.md)_
