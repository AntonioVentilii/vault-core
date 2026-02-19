# 💰 Revenue Model: How to Earn with Vault Core

This document outlines how `Vault Core` functions as a "Storage-as-a-Service" business and how you can configure it to earn **ICP** or **ckUSDC**.

---

## 1. The Business Model

You act as a **Decentralized Cloud Provider**. Users pay you for:

1.  **Initiating Uploads**: A flat fee to start a session (prevents spam).
2.  **Uploading Data**: A fee per chunk (1MB) to cover storage and bandwidth, plus a profit margin.

The system uses the **PAPI (Paid API)** library to act as a gatekeeper. No one can call `start_upload` or `put_chunk` without attaching payment.

---

## 2. Configuration for ICP and ckUSDC

The directory canister is now configured to support **both** ICP and ckUSDC simultaneously.

### A. To Earn ICP

- **Ledger ID**: `ryjl3-tyaaa-aaaaa-aaaba-cai` (The main ICP Ledger)
- **Decimals**: 8 (1 ICP = 100,000,000 e8s)

### B. To Earn ckUSDC

- **Ledger ID**: `xevnm-gaaaa-aaar-qafqa-cai` (The main ckUSDC Ledger)
- **Decimals**: 6 (1 USDC = 1,000,000 units)

---

## 3. Deployment Steps

When you deploy your canister (or reinstall it), you pass the `InitArgs` struct. You must set both ledger IDs.

### Example

```bash
# Mainnet IDs
ICP_LEDGER="ryjl3-tyaaa-aaaaa-aaaba-cai"
CKUSDC_LEDGER="xevnm-gaaaa-aaar-qafqa-cai"

dfx deploy directory --argument "(record {
    icp_ledger = principal \"$ICP_LEDGER\";
    ckusdc_ledger = principal \"$CKUSDC_LEDGER\";
})"
```

---

## 4. Automatic Pricing Logic

The system automatically detects which token the user is paying with and charges the appropriate amount to maintain consistent value.

**Logic in `src/directory/src/payments.rs`:**

| Action           | Payment Method | Fee                       | Value (Approx)              |
| :--------------- | :------------- | :------------------------ | :-------------------------- |
| **Start Upload** | **ckUSDC**     | `100_000` (6 decimals)    | **$0.10**                   |
| **Start Upload** | **ICP**        | `1,000,000` (8 decimals)  | **0.01 ICP** (~$0.10)       |
| **Start Upload** | **Cycles**     | `100_000` (Treats as USD) | N/A                         |
|                  |                |                           |                             |
| **Put Chunk**    | **ckUSDC**     | `30_000` (6 decimals)     | **$0.03 / MB**              |
| **Put Chunk**    | **ICP**        | `300,000` (8 decimals)    | **0.003 ICP / MB** (~$0.03) |

> **Note**: ICP pricing assumes 1 ICP ~= $10.00 for roughly equivalent value. You can adjust this logic in `payments.rs` if the price of ICP shifts significantly.

### Scenario A: Charging in ckUSDC (6 decimals)

- `StartUpload`: `1_000_000` = **$1.00 USD**
- `PutChunk`: `30_000` = **$0.03 USD / MB** ($30/GB). This covers ~6 years of storage costs on the IC.

### Scenario B: Charging in ICP (8 decimals)

- Assume 1 ICP = $10.00 USD.
- `StartUpload`: `1_000_000` e8s = 0.01 ICP = **$0.10 USD**
- `PutChunk`: `100_000` e8s = 0.001 ICP = **$0.01 USD / MB**

---

## 5. Future Model: Pay-As-You-Go (Rent / Subscription)

Currently, users pay **once** to upload. To handle long-term "forever" storage, you might want to implement a **Rent Model**.

### Core Concept: "Time-to-Live" (TTL)

Instead of buying storage space, users buy **time**.

1.  **Storage Account Balance**: Each user has a `balance` in the Directory canister (deposited via ICP/ckUSDC).
2.  **Burn Rate**: The system calculates a daily cost based on the user's total stored data (e.g., `$0.01 per GB / Month`).
3.  **Expiration**:
    - Every file or user account has an `expires_at` timestamp.
    - **Payment = Extension**: When a user pays (e.g., $10), the system extends their `expires_at` date by `Amount / BurnRate`.

### The "Death" of a Document

If a user stops paying and their `balance` hits zero (or `expires_at` follows the current date):

1.  **Grace Period**: The system enters a "Frozen" state for 30 days. Files are readable but not writable.
2.  **Garbage Collection (The Reaper)**:
    - A periodic automated task (Heartbeat or Cron) scans for expired users.
    - **Deletion**: It permanently deletes the metadata and tells buckets to free the chunks.
    - **Result**: The user loses their documents to free up space for paying users.

### Implementation Sketch (No Code)

- Add `prepaid_balance: Nat` to `UserState`.
- Add `rate_per_gb_per_month: Nat` to `Config`.
- Add `top_up_balance()` method.
- Implement `canister_heartbeat` to check for expired accounts and delete them.

---

## 6. Withdrawing Your Earnings

All earnings accumulate in the **Directory Canister's Main Account** on the specified ledger.

To cash out:

1.  **Check Balance**:
    ```bash
    dfx canister call $LEDGER icrc1_balance_of "(record { owner = principal \"$(dfx canister id directory)\"; subaccount = null })"
    ```
2.  **Transfer to Admin**:
    - You will need to add an `admin_withdraw` method to the Directory canister (if not already present) that allows the controller to send these tokens to your personal wallet.
    - _Alternatively_, if the canister is controlled by your NNS neuron or wallet, you can use `dfx canister call` to execute a transfer if you have implemented a proxy transfer method.

---

## 7. Summary Checklist

1.  [ ] **Decide** on ICP vs ckUSDC.
2.  [ ] **Update** `src/directory/src/payments.rs` with appropriate fees for that token's decimals.
3.  [ ] **Deploy** `directory` with the correct Ledger Principal in the init args.
4.  [ ] **Profit!**
