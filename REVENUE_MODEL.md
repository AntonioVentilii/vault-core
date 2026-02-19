# 💰 Revenue Model: How to Earn with Vault Core

This document explains how `Vault Core` functions as a "Storage-as-a-Service" business and how it is configured to earn **ICP** or **ckUSDC**.

---

## 1. The Business Model

Vault Core acts as a **Decentralized Cloud Provider**. It generates revenue through:

1.  **Action Fees (Pay-per-Action)**:
    - **Initiating Uploads**: A flat fee to start a session ($0.10).
    - **Uploading Data**: A fee per chunk (1MB) to cover storage and bandwidth ($0.03/MB).
2.  **Rent Model (Pay-As-You-Go)**:
    - Users purchase "Time-to-Live" (TTL) for their account.
    - Data is charged a monthly rate (e.g., $0.01 per GB / Month).
    - If an account expires, data is frozen and eventually deleted to free up space.

---

## 2. Configuration for ICP and ckUSDC

The system supports both ICP and ckUSDC. Fees are automatically adjusted based on the token's decimals.

| Action           | ckUSDC (6 decimals) | ICP (8 decimals) | Approx. Value  |
| :--------------- | :------------------ | :--------------- | :------------- |
| **Start Upload** | 100,000             | 1,000,000        | **$0.10**      |
| **Put Chunk**    | 30,000              | 300,000          | **$0.03 / MB** |

---

## 3. The Rent Model (TTL)

Instead of a one-time payment for "forever" storage, users top up their account balance which extends their **Expiration Date**.

### How it works:

- **Top-Up**: Use the `top_up_balance(amount, payment)` method.
- **Extension**: The system calculates the extension based on `Amount / BurnRate`.
- **Burn Rate**: Depends on the `used_bytes` and the `rate_per_gb_per_month` configured in the canister.

### The "Death" of a Document:

If a user's `expires_at` date passes:

1.  **Grace Period**: The system allows a 30-day grace period.
2.  **Garbage Collection**: An automated heartbeat task periodically scans for expired users and deletes their metadata and files.

---

## 4. Administrative Features

### Withdrawal

Canister controllers can withdraw accumulated earnings using:

- `admin_withdraw(ledger, amount, to)`

### Configuration

Administrators set the rates during deployment or via upgrades:

- `icp_ledger`: Principal
- `ckusdc_ledger`: Principal
- `rate_per_gb_per_month`: Amount in token units (e.g., 10,000 for $0.01).

---

## 5. Deployment Example

```bash
# Set rates and ledgers
dfx deploy directory --argument "(record {
    icp_ledger = principal \"ryjl3-tyaaa-aaaaa-aaaba-cai\";
    ckusdc_ledger = principal \"xevnm-gaaaa-aaar-qafqa-cai\";
    rate_per_gb_per_month = 10000; # $0.01 per GB
})"
```
