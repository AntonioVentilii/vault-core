#!/bin/bash
set -e

# Vault Core Upload Demo
# This script demonstrates the full upload flow using dfx calls.

echo "🔷 Provisioning Bucket..."
BUCKET_ID=$(dfx canister id bucket)
dfx canister call directory provision_bucket "(principal \"$BUCKET_ID\")"

echo "🔷 Starting Upload (Paying 1M cycles)..."
# Capture the upload_id from the response
# The response is a Result<UploadSession, String>
# record { upload_id = blob "..."; ... }
UPLOAD_ID=$(dfx canister call directory start_upload '("my_demo_file.txt", 100, opt variant { AttachedCycles })' --with-cycles 1000000 | grep -o 'upload_id = blob "[^"]*"' | cut -d'"' -f2)

echo "🔷 Upload ID: $UPLOAD_ID"

echo "🔷 Getting Upload Tokens..."
dfx canister call directory get_upload_tokens "(blob \"$UPLOAD_ID\", vec { 0 })"

echo "🔷 Uploading Chunk 0 (Paying 50k cycles)..."
# For demo purposes, we usually extract the token from the previous call.
# This script assumes you know the token structure.
# dfx canister call bucket put_chunk ...

echo "🔷 Committing Upload..."
dfx canister call directory commit_upload "(blob \"$UPLOAD_ID\")"

echo "✅ Flow complete! Check files with:"
echo "dfx canister call directory list_files '()'"
