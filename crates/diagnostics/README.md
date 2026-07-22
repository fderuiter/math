High-Integrity Diagnostic Suite

This module provides a unified diagnostic trait and centralized reporting bus
to eliminate silent thread failures and standardize error reporting across all modules.

# Developer Guide: Implementing Diagnostics

When creating a new error type in a library module, you must implement the
`Diagnostic` trait to ensure it integrates with the centralized reporting bus.

## Example
```rust
use diagnostics::{Diagnostic, Severity};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyModuleError {
    #[error("Numerical instability detected at state {state}")]
    Instability { state: String },
}

impl Diagnostic for MyModuleError {
    fn severity(&self) -> Severity {
        match self {
            Self::Instability { .. } => Severity::Warning,
        }
    }

    fn metadata(&self) -> HashMap<String, String> {
        let mut meta = HashMap::new();
        match self {
            Self::Instability { state } => {
                meta.insert("simulation_state".to_string(), state.clone());
            }
        }
        meta
    }
}
```

Emit the error to the bus using `diagnostics::emit_error(&my_error)`.
