use std::cell::RefCell;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComplexityMetrics {
    pub arithmetic_ops: u64,
    pub memory_loads: u64,
    pub function_calls: u64,
}

impl ComplexityMetrics {
    pub const fn new() -> Self {
        Self {
            arithmetic_ops: 0,
            memory_loads: 0,
            function_calls: 0,
        }
    }
}

thread_local! {
    pub static METRICS: RefCell<ComplexityMetrics> = const { RefCell::new(ComplexityMetrics::new()) };
    pub static VERIFICATION_MODE: RefCell<bool> = const { RefCell::new(false) };
}

pub fn enable_verification() {
    VERIFICATION_MODE.with(|m| *m.borrow_mut() = true);
}

pub fn disable_verification() {
    VERIFICATION_MODE.with(|m| *m.borrow_mut() = false);
}

pub fn is_verification_enabled() -> bool {
    VERIFICATION_MODE.with(|m| *m.borrow())
}

pub fn reset_metrics() {
    METRICS.with(|m| *m.borrow_mut() = ComplexityMetrics::default());
}

pub fn get_metrics() -> ComplexityMetrics {
    METRICS.with(|m| m.borrow().clone())
}

#[inline]
pub fn increment_arithmetic() {
    if is_verification_enabled() {
        METRICS.with(|m| m.borrow_mut().arithmetic_ops += 1);
    }
}

#[inline]
pub fn increment_memory_loads() {
    if is_verification_enabled() {
        METRICS.with(|m| m.borrow_mut().memory_loads += 1);
    }
}

#[inline]
pub fn increment_calls() {
    if is_verification_enabled() {
        METRICS.with(|m| m.borrow_mut().function_calls += 1);
    }
}
