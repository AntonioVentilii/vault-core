use std::sync::LazyLock;

use candid::CandidType;
use ic_papi_api::PaymentType;
use ic_papi_guard::guards::any::{PaymentGuard, VendorPaymentConfig};
use serde::{Deserialize, Serialize};

use crate::memory::{ckusdc_ledger, icp_ledger};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum SignerMethods {
    StartUpload,
    PutChunk,
}

impl SignerMethods {
    pub fn fee(&self, payment: &PaymentType) -> u64 {
        // ICP has 8 decimals (10^8)
        // ckUSDC and most others have 6 decimals (10^6)
        // We check if the payment is targeting the ICP ledger.
        let is_icp = match payment {
            PaymentType::CallerPaysIcrc2Tokens(ledger) => (*ledger).ledger == icp_ledger(),
            PaymentType::PatronPaysIcrc2Tokens(ledger) => (*ledger).ledger == icp_ledger(),
            // For cycles/other, we treat as USD equivalent (6 decimals)
            _ => false,
        };

        match self {
            SignerMethods::StartUpload => {
                if is_icp {
                    // $0.10 ~= 0.01 ICP = 1,000,000 e8s (assuming $10/ICP)
                    // Let's stick to the previous high value if user uses ICP
                    1_000_000
                } else {
                    // $0.10 = 100,000 (6 decimals)
                    100_000
                }
            }
            SignerMethods::PutChunk => {
                if is_icp {
                    // $0.03 ~= 0.003 ICP = 300,000 e8s
                    300_000
                } else {
                    // $0.03 = 30,000 (6 decimals)
                    30_000
                }
            }
        }
    }
}

pub static PAYMENT_GUARD: LazyLock<PaymentGuard<7>> = LazyLock::new(|| PaymentGuard {
    supported: [
        VendorPaymentConfig::AttachedCycles,
        VendorPaymentConfig::CallerPaysIcrc2Cycles,
        VendorPaymentConfig::PatronPaysIcrc2Cycles,
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
