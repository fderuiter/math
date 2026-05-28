use crate::metrics::{enable_verification, disable_verification, reset_metrics, get_metrics, ComplexityMetrics};
use crate::allocator::{lock_allocations, unlock_allocations};

pub struct VerifiedEngine;

impl VerifiedEngine {
    pub fn run_verified<F, R>(f: F) -> (R, ComplexityMetrics)
    where
        F: FnOnce() -> R,
    {
        enable_verification();
        reset_metrics();
        // NASA Power of 10 Rule 3: No dynamic memory allocation after initialization.
        // We assume initialization is done outside of `run_verified`, so we lock here.
        lock_allocations();
        
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
        
        unlock_allocations();
        let metrics = get_metrics();
        disable_verification();
        
        match result {
            Ok(r) => (r, metrics),
            Err(e) => std::panic::resume_unwind(e),
        }
    }
}
