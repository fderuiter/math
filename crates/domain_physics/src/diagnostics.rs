//! High-Integrity Diagnostic Suite
//!
//! This module provides a unified diagnostic trait and centralized reporting bus
//! to eliminate silent thread failures and standardize error reporting across all modules.
//!
//! # Developer Guide: Implementing Diagnostics
//!
//! When creating a new error type in a library module, you must implement the
//! `Diagnostic` trait to ensure it integrates with the centralized reporting bus.
//!
//! ## Example
//! ```rust
//! use domain_physics::diagnostics::{Diagnostic, Severity};
//! use std::collections::HashMap;
//! use thiserror::Error;
//!
//! #[derive(Debug, Error)]
//! pub enum MyModuleError {
//!     #[error("Numerical instability detected at state {state}")]
//!     Instability { state: String },
//! }
//!
//! impl Diagnostic for MyModuleError {
//!     fn severity(&self) -> Severity {
//!         match self {
//!             Self::Instability { .. } => Severity::Warning,
//!         }
//!     }
//!
//!     fn metadata(&self) -> HashMap<String, String> {
//!         let mut meta = HashMap::new();
//!         match self {
//!             Self::Instability { state } => {
//!                 meta.insert("simulation_state".to_string(), state.clone());
//!             }
//!         }
//!         meta
//!     }
//! }
//! ```
//!
//! Emit the error to the bus using `diagnostics::emit_error(&my_error)`.

use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex, OnceLock};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Severity {
    Info,
    Warning,
    Error,
    Fatal,
}

impl std::fmt::Display for Severity {
    #[verified_engine::verified]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Info => write!(f, "INFO"),
            Severity::Warning => write!(f, "WARNING"),
            Severity::Error => write!(f, "ERROR"),
            Severity::Fatal => write!(f, "FATAL"),
        }
    }
}

pub trait Diagnostic: std::error::Error + Send + Sync + 'static {
    #[verified_engine::verified]
    fn severity(&self) -> Severity {
        Severity::Error
    }
    #[verified_engine::verified]
    fn metadata(&self) -> HashMap<String, String> {
        HashMap::new()
    }
}

#[derive(Debug, Clone)]
pub struct DiagnosticEvent {
    pub severity: Severity,
    pub message: String,
    pub metadata: HashMap<String, String>,
    pub thread_name: Option<String>,
}

pub struct DiagnosticBus {
    sender: Sender<DiagnosticEvent>,
    receiver: Arc<Mutex<Receiver<DiagnosticEvent>>>,
}

impl DiagnosticBus {
    #[verified_engine::verified]
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        Self {
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }

    #[verified_engine::verified]
    pub fn emit(&self, event: DiagnosticEvent) {
        let _ = self.sender.send(event);
    }

    #[verified_engine::verified]
    pub fn emit_error<E: Diagnostic>(&self, err: &E) {
        let metadata = err.metadata();
        let thread = std::thread::current();
        let thread_name = thread.name().map(|s| s.to_string());
        self.emit(DiagnosticEvent {
            severity: err.severity(),
            message: err.to_string(),
            metadata,
            thread_name,
        });
    }

    #[verified_engine::verified]
    pub fn try_recv_all(&self) -> Vec<DiagnosticEvent> {
        let mut events = Vec::new();
        if let Ok(rx) = self.receiver.lock() {
            while let Ok(event) = rx.try_recv() {
                events.push(event);
            }
        }
        events
    }
}

impl Default for DiagnosticBus {
    #[verified_engine::verified]
    fn default() -> Self {
        Self::new()
    }
}

#[verified_engine::verified]
pub fn global_bus() -> &'static DiagnosticBus {
    static BUS: OnceLock<DiagnosticBus> = OnceLock::new();
    BUS.get_or_init(DiagnosticBus::new)
}

#[verified_engine::verified]
pub fn emit(event: DiagnosticEvent) {
    global_bus().emit(event);
}

#[verified_engine::verified]
pub fn emit_error<E: Diagnostic>(err: &E) {
    global_bus().emit_error(err);
}

#[verified_engine::verified]
pub fn init_panic_hook() {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let message = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            (*s).to_string()
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "Unknown panic".to_string()
        };

        let location = panic_info
            .location()
            .map(|l| format!("{}:{}", l.file(), l.line()))
            .unwrap_or_default();
        let mut metadata = HashMap::new();
        metadata.insert("location".to_string(), location);

        global_bus().emit(DiagnosticEvent {
            severity: Severity::Fatal,
            message: format!("Thread panicked: {}", message),
            metadata,
            thread_name: std::thread::current().name().map(|s| s.to_string()),
        });

        default_hook(panic_info);
    }));
}
