use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, Ordering};

use verified_engine_macros::runtime_violation;

pub struct VerifiedAllocator;

pub static MEMORY_LOCK: AtomicBool = AtomicBool::new(false);

pub fn lock_allocations() {
    MEMORY_LOCK.store(true, Ordering::SeqCst);
}

pub fn unlock_allocations() {
    MEMORY_LOCK.store(false, Ordering::SeqCst);
}

unsafe impl GlobalAlloc for VerifiedAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if MEMORY_LOCK.load(Ordering::SeqCst) && crate::metrics::is_verification_enabled() {
            runtime_violation!("VIOLATION: Dynamic memory allocation after initialization.");
        }
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        if MEMORY_LOCK.load(Ordering::SeqCst) && crate::metrics::is_verification_enabled() {
            runtime_violation!("VIOLATION: Dynamic memory allocation after initialization.");
        }
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        if MEMORY_LOCK.load(Ordering::SeqCst) && crate::metrics::is_verification_enabled() {
            runtime_violation!("VIOLATION: Dynamic memory reallocation after initialization.");
        }
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}
