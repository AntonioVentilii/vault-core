use candid::Principal;
use ic_cdk::api::call::CallResult;
use ic_cdk_macros::update;

#[update]
async fn proxy_call(
    canister_id: Principal,
    method: String,
    args: Vec<u8>,
    cycles: u128,
) -> Vec<u8> {
    let res: CallResult<Vec<u8>> =
        ic_cdk::api::call::call_raw128(canister_id, &method, args, cycles).await;

    match res {
        Ok(bytes) => bytes,
        Err((code, msg)) => ic_cdk::trap(&format!("Proxy call failed: {:?} {}", code, msg)),
    }
}
