use std::sync::LazyLock;

use candid::CandidType;
use ic_papi_guard::guards::any::{PaymentGuard, VendorPaymentConfig};
use serde::{Deserialize, Serialize};

use crate::memory::payment_ledger;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum SignerMethods {
    StartUpload,
    PutChunk,
}

impl SignerMethods {
    pub fn fee(&self) -> u64 {
        match self {
            SignerMethods::StartUpload => 1_000_000, // 1M cycles/tokens fee
            SignerMethods::PutChunk => 100_000,
        }
    }
}

pub static PAYMENT_GUARD: LazyLock<PaymentGuard<5>> = LazyLock::new(|| PaymentGuard {
    supported: [
        VendorPaymentConfig::AttachedCycles,
        VendorPaymentConfig::CallerPaysIcrc2Cycles,
        VendorPaymentConfig::PatronPaysIcrc2Cycles,
        VendorPaymentConfig::CallerPaysIcrc2Tokens {
            ledger: payment_ledger(),
        },
        VendorPaymentConfig::PatronPaysIcrc2Tokens {
            ledger: payment_ledger(),
        },
    ],
});
