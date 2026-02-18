use std::sync::LazyLock;

use ic_papi_guard::guards::any::{PaymentGuard, VendorPaymentConfig};

pub enum SignerMethods {
    PutChunk,
}

impl SignerMethods {
    pub fn fee(&self) -> u64 {
        match self {
            SignerMethods::PutChunk => 50_000, // 50k cycles/tokens fee per chunk
        }
    }
}

pub static PAYMENT_GUARD: LazyLock<PaymentGuard<1>> = LazyLock::new(|| PaymentGuard {
    supported: [VendorPaymentConfig::AttachedCycles],
});
