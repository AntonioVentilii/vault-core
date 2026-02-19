#!/bin/bash
set -e

# Define test file path
TEST_FILE="tests/test_upload.txt"

# Ensure tests directory exists
mkdir -p tests

# Create a dummy file
echo "Hello, World! Timestamp: $(date)" >"$TEST_FILE"
echo "📝 Created dummy file at $TEST_FILE"

# Run the upload script
echo "🚀 Running upload script..."
./scripts/upload_file.sh "$TEST_FILE"

# Verify the file exists in the directory
echo "🔍 Verifying file in directory..."
dfx canister call directory list_files

echo "✅ Test flow completed successfully!"
