use std::sync::LazyLock;

use ic_papi_api::PaymentType;
use ic_papi_guard::guards::any::{PaymentGuard, VendorPaymentConfig};

use crate::memory::{ckusdc_ledger, icp_ledger};

pub enum SignerMethods {
    PutChunk,
}

impl SignerMethods {
    pub fn fee(&self, payment: &PaymentType) -> u64 {
        let is_icp = match payment {
            PaymentType::CallerPaysIcrc2Tokens(ledger) => ledger.ledger == icp_ledger(),
            PaymentType::PatronPaysIcrc2Tokens(ledger) => ledger.ledger == icp_ledger(),
            _ => false,
        };

        match self {
            SignerMethods::PutChunk => {
                if is_icp {
                    // $0.03 = 300,000 e8s
                    300_000
                } else {
                    // $0.03 = 30,000 (6 decimals)
                    30_000
                }
            }
        }
    }
}

pub static PAYMENT_GUARD: LazyLock<PaymentGuard<5>> = LazyLock::new(|| PaymentGuard {
    supported: [
        VendorPaymentConfig::AttachedCycles,
        // Config for CKUSDC
        VendorPaymentConfig::CallerPaysIcrc2Tokens {
            ledger: ckusdc_ledger(),
        },
        VendorPaymentConfig::PatronPaysIcrc2Tokens {
            ledger: ckusdc_ledger(),
        },
        // Config for ICP
        VendorPaymentConfig::CallerPaysIcrc2Tokens {
            ledger: icp_ledger(),
        },
        VendorPaymentConfig::PatronPaysIcrc2Tokens {
            ledger: icp_ledger(),
        },
    ],
});
