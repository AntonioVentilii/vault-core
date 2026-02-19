# 🔐 Security Roadmap

This document outlines the security architecture of Vault Core and the planned evolution towards advanced privacy models.

---

## 🏗️ Current Security Architecture

Vault Core implements a multi-layered security model for file access:

1.  **Identity-Based Access (ACL)**:
    - Identity on the IC is cryptographic and non-forgeable.
    - Owners have full control.
    - `Reader` and `Writer` roles are managed via Access Control Lists (ACLs).
2.  **Capability-Based Access (Link Sharing)**:
    - **Shareable Links**: 256-bit unguessable random tokens for public/semi-public sharing.
    - **Signed Capability Tokens**: HMAC-SHA256 signed tokens containing `file_id`, `bucket_id`, and `expires_at`. Verified by Buckets in a decentralized manner.

---

## 🚀 Future Security Evolution

### 🕵️ Phase 1: Public Access & Asset Integration

Support for `visibility: Public`. If a file is marked public, the Directory will issue capability tokens to anonymous callers without permission checks. This will also include better integration with standard IC asset canister patterns.

### 🧨 Phase 2: Client-Side Encryption (True Privacy)

**Access control ≠ confidentiality.** On the IC, node providers could technically inspect stable memory. For "Google Drive level" confidentiality, Vault Core will support **Client-Side Encryption**:

- Files are encrypted in the browser/client BEFORE upload.
- The canister stores only encrypted blobs.
- Decryption keys are managed by the client or shared via out-of-band links.
- Only the holder of the decryption key can view the content, even if they have a valid access token.

### 🛡️ Phase 3: Advanced Abuse Controls

- **Fine-grained Rate Limiting**: Throttling based on user weight or history.
- **Proof-of-Work (PoW) Gauging**: Requiring minor PoW for anonymous link resolution to prevent crawler abuse.
- **DDoS Mitigation**: Advanced patterns for ensuring canister stability under high load.

---

## 🎯 Security Best Practices

- ✔ Use 256 bits of randomness for sharing tokens.
- ✔ Enforce short-lived capability tokens (5–15 minutes).
- ✔ Always check expiration and revocation status.
- ✔ Avoid sequential or guessable identifiers.
