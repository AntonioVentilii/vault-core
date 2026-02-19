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
    TopUp(u64),
}

impl SignerMethods {
    pub fn fee(&self, payment: &PaymentType) -> u64 {
        let is_icp = match payment {
            PaymentType::CallerPaysIcrc2Tokens(ledger) => ledger.ledger == icp_ledger(),
            PaymentType::PatronPaysIcrc2Tokens(ledger) => ledger.ledger == icp_ledger(),
            _ => false,
        };

        match self {
            SignerMethods::StartUpload => {
                if is_icp {
                    1_000_000
                } else {
                    100_000
                }
            }
            SignerMethods::PutChunk => {
                if is_icp {
                    300_000
                } else {
                    30_000
                }
            }
            SignerMethods::TopUp(amount) => *amount,
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
