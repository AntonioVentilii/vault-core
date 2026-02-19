#!/bin/bash
set -euo pipefail

# Usage: ./scripts/upload_file.sh <file_path>
FILE_PATH="$1"

if [ -z "$FILE_PATH" ]; then
  echo "Usage: $0 <file_path>"
  exit 1
fi

if [ ! -f "$FILE_PATH" ]; then
  echo "Error: File '$FILE_PATH' not found."
  exit 1
fi

# Helper to convert hex string to Candid blob format
hex_to_candid_blob() {
  local hex="$1"
  # turns "a1b2" into "\a1\b2"
  echo "blob \"$(echo "$hex" | sed 's/../\\&/g')\""
}

# 1. Preparation
NETWORK="${DFX_NETWORK:-local}"
WALLET="$(dfx identity get-wallet --network "$NETWORK")"
FILENAME=$(basename "$FILE_PATH")
OUT_DIR="out/upload"
mkdir -p "$OUT_DIR"

# Cleanup trap
cleanup() {
  if [ "${KEEP_TMP:-0}" = "1" ] || [ "${EXIT_CODE:-0}" != "0" ]; then
    echo "🧪 Keeping outputs in $OUT_DIR (EXIT_CODE=$EXIT_CODE)."
    return
  fi
  rm -rf "$OUT_DIR"
}
EXIT_CODE=0
trap 'EXIT_CODE=$?; cleanup' EXIT

FILE_SIZE=$(wc -c <"$FILE_PATH" | tr -d ' ')
MIME_TYPE=$(file --mime-type -b "$FILE_PATH")

echo "📂 Preparing to upload '$FILENAME'"
echo "   Size: $FILE_SIZE bytes"
echo "   Type: $MIME_TYPE"

# 2. Start Upload
echo "🚀 Starting upload session..."

# Start upload session
echo "   Calls directory.start_upload..."
dfx canister call directory start_upload "(\"$FILENAME\", \"$MIME_TYPE\", $FILE_SIZE, opt variant { AttachedCycles })" \
  --network "$NETWORK" \
  --wallet "$WALLET" \
  --with-cycles 100000000 \
  >"$OUT_DIR/start_upload_out.txt" 2>&1

START_RES=$(cat "$OUT_DIR/start_upload_out.txt" | idl2json)

# Check for error in output
if grep -q "Err" "$OUT_DIR/start_upload_out.txt"; then
  echo "❌ Start upload returned failure:"
  cat "$OUT_DIR/start_upload_out.txt"
  exit 1
fi

# Extract session details
CHUNK_SIZE=$(echo "$START_RES" | jq -r '.Ok.chunk_size')
EXPECTED_CHUNKS=$(echo "$START_RES" | jq -r '.Ok.expected_chunk_count')

# Format UPLOAD_ID for Candid (upload_id is vec nat8)
UPLOAD_ID_HEX=$(echo "$START_RES" | jq -r '.Ok.upload_id[]' | xargs printf "%02x")

if [ -z "$UPLOAD_ID_HEX" ] || [ "$UPLOAD_ID_HEX" = "null" ]; then
  echo "❌ Failed to parse upload session (upload_id). Raw response:"
  echo "$START_RES"
  exit 1
fi

# Candid expects blob "\xx\yy..." (not a raw hex string)
UPLOAD_ID_ARG=$(hex_to_candid_blob "$UPLOAD_ID_HEX")

echo "🆔 Upload ID: $UPLOAD_ID_HEX"
echo "📦 Chunk Size: $CHUNK_SIZE"
echo "🔢 Expected Chunks: $EXPECTED_CHUNKS"

# 3. Get Upload Tokens
echo "🎟️  Getting upload tokens..."

# Build indices as: vec { 0; 1; 2 }
CHUNK_INDICES_VEC="vec {"
for ((i = 0; i < EXPECTED_CHUNKS; i++)); do
  if [ $i -gt 0 ]; then
    CHUNK_INDICES_VEC="$CHUNK_INDICES_VEC; "
  fi
  CHUNK_INDICES_VEC="$CHUNK_INDICES_VEC$i"
done
CHUNK_INDICES_VEC="$CHUNK_INDICES_VEC }"

dfx canister call directory get_upload_tokens "($UPLOAD_ID_ARG, $CHUNK_INDICES_VEC)" \
  --network "$NETWORK" \
  --wallet "$WALLET" \
  >"$OUT_DIR/tokens_out.txt" 2>&1

# Hard fail if the canister returned Err (or call failed)
if grep -q "Err" "$OUT_DIR/tokens_out.txt"; then
  echo "❌ get_upload_tokens returned failure:"
  cat "$OUT_DIR/tokens_out.txt"
  exit 1
fi

TOKENS_RES=$(cat "$OUT_DIR/tokens_out.txt" | idl2json)

# Ensure we actually have an Ok array
TOKENS_LEN=$(echo "$TOKENS_RES" | jq -r '.Ok | length // 0')

if [ "$TOKENS_LEN" -eq 0 ]; then
  echo "❌ No tokens received. Parsed response:"
  echo "$TOKENS_RES" | jq .
  echo "Raw candid response:"
  cat "$OUT_DIR/tokens_out.txt"
  exit 1
fi

# 4. Upload Chunks
echo "📤 Uploading $EXPECTED_CHUNKS chunks..."

for ((i = 0; i < EXPECTED_CHUNKS; i++)); do
  echo "   Processing Chunk $i..."

  # Extract token logic
  TOKEN_JSON=$(echo "$TOKENS_RES" | jq -r ".Ok[] | select(.allowed_chunks | index($i))")

  if [ -z "$TOKEN_JSON" ]; then
    echo "❌ No token found for chunk $i"
    exit 1
  fi

  # Construct token record
  SIG_HEX=$(echo "$TOKEN_JSON" | jq -r '.sig[]' | xargs printf "%02x")
  BUCKET_ID=$(echo "$TOKEN_JSON" | jq -r '.bucket_id')
  DIR_ID=$(echo "$TOKEN_JSON" | jq -r '.directory_id')
  EXPIRES_AT=$(echo "$TOKEN_JSON" | jq -r '.expires_at')
  ALLOWED_CHUNKS=$(echo "$TOKEN_JSON" | jq -r '.allowed_chunks | join("; ")')
  FILE_ID_BLOB_HEX=$(echo "$TOKEN_JSON" | jq -r '.file_id.id[]' | xargs printf "%02x")
  FILE_ID_OWNER=$(echo "$TOKEN_JSON" | jq -r '.file_id.owner')

  SIG_BLOB=$(hex_to_candid_blob "$SIG_HEX")
  UPLOAD_ID_BLOB=$(hex_to_candid_blob "$UPLOAD_ID_HEX")
  FILE_ID_BLOB=$(hex_to_candid_blob "$FILE_ID_BLOB_HEX")

  TOKEN_ARG="record {
        sig = $SIG_BLOB;
        bucket_id = principal \"$BUCKET_ID\";
        upload_id = $UPLOAD_ID_BLOB;
        directory_id = principal \"$DIR_ID\";
        expires_at = $EXPIRES_AT;
        allowed_chunks = vec { $ALLOWED_CHUNKS };
        file_id = record { id = $FILE_ID_BLOB; owner = principal \"$FILE_ID_OWNER\" }
    }"

  # Read Chunk Data
  CHUNK_FILE="$OUT_DIR/chunk_${UPLOAD_ID_HEX}_${i}.bin"
  dd if="$FILE_PATH" of="$CHUNK_FILE" bs=$CHUNK_SIZE skip=$i count=1 2>/dev/null

  CHUNK_HEX=$(xxd -p -c 100000000 "$CHUNK_FILE")
  CHUNK_BLOB=$(hex_to_candid_blob "$CHUNK_HEX")

  # Call put_chunk
  echo "   Sending Chunk $i to bucket $BUCKET_ID..."
  dfx canister call bucket put_chunk "($TOKEN_ARG, $i, $CHUNK_BLOB, null)" \
    --network "$NETWORK" \
    --wallet "$WALLET" \
    --with-cycles 50000 \
    >"$OUT_DIR/put_chunk_out.txt" 2>&1

  if grep -q "Err" "$OUT_DIR/put_chunk_out.txt"; then
    echo "❌ Failed to put chunk $i"
    cat "$OUT_DIR/put_chunk_out.txt"
    rm "$CHUNK_FILE"
    exit 1
  fi

  rm "$CHUNK_FILE"
done

# 5. Commit Upload
echo "💾 Committing upload..."
dfx canister call directory commit_upload "($UPLOAD_ID_ARG)" \
  --network "$NETWORK" \
  --wallet "$WALLET" \
  >"$OUT_DIR/commit_out.txt" 2>&1
COMMIT_RES=$(cat "$OUT_DIR/commit_out.txt" | idl2json)

if [ "$(echo "$COMMIT_RES" | jq -r '.Err // empty')" != "" ]; then
  echo "❌ Failed to commit upload"
  echo "$COMMIT_RES"
  exit 1
fi

FILE_ID_RES=$(echo "$COMMIT_RES" | jq -r '.Ok.file_id.id[]' | xargs printf "%02x")
echo "✅ Upload Complete!"
echo "📄 File ID: $FILE_ID_RES"

# Verify
echo "🔍 Verifying file presence..."
dfx canister call directory list_files '()' \
  --network "$NETWORK" \
  --wallet "$WALLET" \
  >"$OUT_DIR/list_out.txt" 2>&1
LIST_RES=$(cat "$OUT_DIR/list_out.txt" | idl2json)

IDS=$(echo "$LIST_RES" | jq -r '.[].file_id.id[]' | xargs printf "%02x")
if [[ "$IDS" == *"$FILE_ID_RES"* ]]; then
  echo "🎉 Verification Successful: File found in directory listing."
else
  echo "⚠️  Verification Failed: File not found in directory listing."
fi
