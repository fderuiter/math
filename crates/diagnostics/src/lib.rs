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
//! use diagnostics::{Diagnostic, Severity};
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
#[allow(missing_docs)]
pub enum Severity {
    #[allow(missing_docs)]
    Info,
    #[allow(missing_docs)]
    Warning,
    #[allow(missing_docs)]
    Error,
    #[allow(missing_docs)]
    Fatal,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Info => write!(f, "INFO"),
            Severity::Warning => write!(f, "WARNING"),
            Severity::Error => write!(f, "ERROR"),
            Severity::Fatal => write!(f, "FATAL"),
        }
    }
}

#[allow(missing_docs)]
pub trait Diagnostic: std::error::Error + Send + Sync + 'static {
    #[allow(missing_docs)]
    fn severity(&self) -> Severity {
        Severity::Error
    }
    #[allow(missing_docs)]
    fn metadata(&self) -> HashMap<String, String> {
        HashMap::new()
    }
}

#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub struct DiagnosticEvent {
    #[allow(missing_docs)]
    pub severity: Severity,
    #[allow(missing_docs)]
    pub message: String,
    #[allow(missing_docs)]
    pub metadata: HashMap<String, String>,
    #[allow(missing_docs)]
    pub thread_name: Option<String>,
}

#[allow(missing_docs)]
pub struct DiagnosticBus {
    sender: Sender<DiagnosticEvent>,
    receiver: Arc<Mutex<Receiver<DiagnosticEvent>>>,
    listeners: Arc<Mutex<Vec<Arc<dyn Fn(&DiagnosticEvent) + Send + Sync + 'static>>>>,
}

impl DiagnosticBus {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        Self {
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
            listeners: Arc::new(Mutex::new(Vec::new())),
        }
    }

    #[allow(missing_docs)]
    pub fn register_listener<F>(&self, listener: F)
    where
        F: Fn(&DiagnosticEvent) + Send + Sync + 'static,
    {
        let mut listeners = match self.listeners.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        listeners.push(Arc::new(listener));
    }

    #[allow(missing_docs)]
    pub fn emit(&self, event: DiagnosticEvent) {
        if let Err(_) = self.sender.send(event.clone()) {
            eprintln!("[{}] {} - Metadata: {:?}", event.severity, event.message, event.metadata);
        }
        let listeners = match self.listeners.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        for listener in listeners.iter() {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                listener(&event);
            }));
            if let Err(_) = result {
                eprintln!("Diagnostic listener panicked while processing event: {}", event.message);
            }
        }
    }

    #[allow(missing_docs)]
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

    #[allow(missing_docs)]
    pub fn try_recv_all(&self) -> Vec<DiagnosticEvent> {
        let mut events = Vec::new();
        let rx = match self.receiver.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        while let Ok(event) = rx.try_recv() {
            events.push(event);
        }
        events
    }
}

impl Default for DiagnosticBus {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(missing_docs)]
pub fn global_bus() -> &'static DiagnosticBus {
    static BUS: OnceLock<DiagnosticBus> = OnceLock::new();
    BUS.get_or_init(DiagnosticBus::new)
}

#[allow(missing_docs)]
pub fn emit(event: DiagnosticEvent) {
    global_bus().emit(event);
}

#[allow(missing_docs)]
pub fn emit_error<E: Diagnostic>(err: &E) {
    global_bus().emit_error(err);
}

#[allow(missing_docs)]
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
