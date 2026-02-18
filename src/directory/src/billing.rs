// Basic pricing constants for v1
// These could be made configurable via admin methods later.

pub const CYCLES_PER_GIB_MONTH: u128 = 100_000_000_000; // 100B cycles per GiB/month as an example
pub const MIN_CREDIT_TO_START_UPLOAD: u128 = 10_000_000; // 10M cycles minimum to attempt an upload
pub const GIB: u64 = 1024 * 1024 * 1024;

/// Calculates the estimated cost of storage for a given size and duration.
/// bytes * (CYCLES_PER_GIB_MONTH / GIB) * (days / 30)
pub fn estimate_storage_cost(size_bytes: u64, days: u32) -> u128 {
    let cycles_per_byte_month = CYCLES_PER_GIB_MONTH / (GIB as u128);
    let total_for_month = (size_bytes as u128) * cycles_per_byte_month;
    
    // cost per day = total_for_month / 30
    // total cost = cost_per_day * days
    (total_for_month * (days as u128)) / 30
}

/// Default storage duration for reservation (e.g. 30 days)
pub const DEFAULT_RETENTION_DAYS: u32 = 30;

pub fn calculate_reservation_cost(size_bytes: u64) -> u128 {
    estimate_storage_cost(size_bytes, DEFAULT_RETENTION_DAYS)
}
