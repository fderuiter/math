//! High-Integrity Diagnostic Suite
//!
//! This module re-exports the unified diagnostic trait and interacts
//! directly with the centralized reporting bus in math_commons using
//! a lock-free bridge to avoid dynamic memory allocation.

pub use math_commons::diagnostics::{Diagnostic, DiagnosticEvent, Severity};

#[verified_engine::verified]
pub fn emit(event: DiagnosticEvent) {
    // If dynamic events are emitted, we route them to the main bus directly if possible.
    // However, calling this in a verified context will abort if it allocates.
    // Ideally, domains should only use `emit_error`
    math_commons::diagnostics::global_bus().emit(event);
}

#[verified_engine::verified]
pub fn emit_error<E: Diagnostic>(err: &E) {
    let severity = err.severity();
    let message = err.static_message();
    let error_code = err.error_code();

    math_commons::diagnostics::global_bridge().push(math_commons::diagnostics::BridgeEvent {
        severity: severity.to_u8(),
        error_code,
        message,
    });
}

#[verified_engine::verified]
pub fn init_panic_hook() {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        // We push a fatal event to the bridge. We cannot easily extract the panic message as a static string,
        // so we use a generic string.
        math_commons::diagnostics::global_bridge().push(math_commons::diagnostics::BridgeEvent {
            severity: 3, // Fatal
            error_code: 999,
            message: "Thread panicked",
        });

        default_hook(panic_info);
    }));
}
