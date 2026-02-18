# 🔐 Security Evolution Roadmap

This document outlines the planned security evolution for Vault Core, moving from the initial V1 implementation to advanced sharing and privacy models.

---

## 🧱 V1: Owner-Only Access (Current)

The system currently enforces strict owner-only access.

- **Access Rule**: `caller == file.owner`
- **Security Logic**: Simple and safe starting point.
- **Verification**: Cleanly enforced at the canister level using IC cryptographic identity.

---

## 🔐 V2: Principal-Based Sharing (ACL)

Expansion of file metadata to include an Access Control List (ACL).

### Proposed Schema

```rust
FileAccess {
    owner: Principal,
    readers: BTreeSet<Principal>,
    writers: BTreeSet<Principal>,
}
```

### Access Check Logic

```rust
if caller == owner
   || readers.contains(caller)
   || writers.contains(caller)
{
    allow
}
else reject
```

- **Why it’s secure**: Identity on IC is cryptographic; no session cookies or bearer tokens. The caller principal cannot be forged.

---

## 🔗 V3: "Google Drive Style" Link Sharing

Transition from **Identity-based access** to **Capability-based access**.

### Concept

A link contains a unique, unguessable token:
`https://your-app/?file_id=abc123&token=xyz456`

### Implementation Options

#### Option A: Random Access Tokens (Simpler)

1. Generate a 256-bit random token on link creation.
2. Store `link_token → { file_id, permission, expires_at }` in the canister state.
3. Verify token existence, file matching, and expiry upon access.

#### Option B: Signed Capability Tokens (More Elegant)

1. Generate a signed capability certificate containing `file_id`, `permission`, and `expiry`.
2. Bucket verifies the signature without requiring a state lookup.
3. High scalability.

---

## 🧨 Security Requirements

To ensure link sharing is cryptographically safe:
✔ Use 128–256 bits of randomness.
✔ Support expiration and revocation.
✔ Rate-limit access.
✔ Avoid sequential IDs.

---

## ⚠ Important: Privacy vs. Access Control

**Access control ≠ confidentiality.**

On the IC, node providers could technically inspect stable memory. For true "Google Drive level" confidentiality, **Client-side encryption before upload** is required.

- Canister stores only encrypted bytes.
- Only the link holder with the decryption key can view the content.

---

## 🏗 Future Architecture

The Directory canister will manage the resolution order:

1. **Owner Check**: If `caller == owner` → allow.
2. **ACL Check**: If `caller` in ACL → allow.
3. **Link Check**: If valid link token → allow.
4. **Visibility Check**: If file is `Public` → allow.
5. **Else** → reject.

---

## 🎯 Bonus: Public Files

Support for `visibility: Private | Shared | Public`. If public, anyone can fetch without a permit or token.
