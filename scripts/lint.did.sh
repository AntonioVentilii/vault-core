#!/usr/bin/env bash
set -euo pipefail

print_help() {
  cat <<-EOF

	Checks that the candid file adheres to our policies.

	EOF
}

[[ "${1:-}" != "--help" ]] || {
  print_help
  exit 0
}

CANDID_FILE="$(jq -re '.canisters["icrc-factory"].candid' dfx.json)"

has_result_types() {
  : Determining whether the canister contains generic Result memory_types...
  git grep -w Result "$CANDID_FILE" || git grep -E 'Result_[0-9]' "$CANDID_FILE"
}

check_result_types() {
  : Checking whether the canister contains generic Result memory_types...
  ! has_result_types || {
    echo "ERROR: $CANDID_FILE should not contain Result or Result_[0-9]."
    echo "       Please define custom Resut types with specific names."
    exit 1
  }
}

check() {
  : Checking the candid file...
  check_result_types
}

check
