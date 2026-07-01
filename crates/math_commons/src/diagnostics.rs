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
//! use math_commons::diagnostics::{Diagnostic, Severity};
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
use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicUsize, AtomicU8, Ordering};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Severity {
    Info,
    Warning,
    Error,
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

impl Severity {
    pub fn from_u8(val: u8) -> Self {
        match val {
            0 => Severity::Info,
            1 => Severity::Warning,
            2 => Severity::Error,
            _ => Severity::Fatal,
        }
    }
    
    pub fn to_u8(&self) -> u8 {
        match self {
            Severity::Info => 0,
            Severity::Warning => 1,
            Severity::Error => 2,
            Severity::Fatal => 3,
        }
    }
}

pub trait Diagnostic: std::error::Error + Send + Sync + 'static {
    fn severity(&self) -> Severity {
        Severity::Error
    }
    fn metadata(&self) -> HashMap<String, String> {
        HashMap::new()
    }
    fn error_code(&self) -> u32 {
        0
    }
    fn static_message(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

#[derive(Debug, Clone)]
pub struct DiagnosticEvent {
    pub severity: Severity,
    pub message: String,
    pub metadata: HashMap<String, String>,
    pub thread_name: Option<String>,
}

#[derive(Clone, Copy)]
pub struct BridgeEvent {
    pub severity: u8,
    pub error_code: u32,
    pub message: &'static str,
}

const QUEUE_SIZE: usize = 1024;

pub struct NoAllocBridge {
    head: AtomicUsize,
    tail: AtomicUsize,
    buffer: [UnsafeCell<BridgeEvent>; QUEUE_SIZE],
    ready: [AtomicU8; QUEUE_SIZE],
}

unsafe impl Send for NoAllocBridge {}
unsafe impl Sync for NoAllocBridge {}

impl NoAllocBridge {
    pub const fn new() -> Self {
        Self {
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
            buffer: [const { UnsafeCell::new(BridgeEvent { severity: 0, error_code: 0, message: "" }) }; QUEUE_SIZE],
            ready: [const { AtomicU8::new(0) }; QUEUE_SIZE],
        }
    }

    pub fn push(&self, event: BridgeEvent) {
        let mut idx = self.head.load(Ordering::Relaxed);
        loop {
            let next = (idx + 1) % QUEUE_SIZE;
            if next == self.tail.load(Ordering::Acquire) {
                return;
            }
            if self.head.compare_exchange_weak(idx, next, Ordering::AcqRel, Ordering::Relaxed).is_ok() {
                while self.ready[idx].load(Ordering::Acquire) != 0 {
                    std::hint::spin_loop();
                }
                unsafe {
                    *self.buffer[idx].get() = event;
                }
                self.ready[idx].store(1, Ordering::Release);
                break;
            } else {
                idx = self.head.load(Ordering::Relaxed);
            }
        }
    }

    pub fn pop_all(&self) -> Vec<BridgeEvent> {
        let mut events = Vec::new();
        let tail = self.tail.load(Ordering::Relaxed);
        let head = self.head.load(Ordering::Acquire);
        
        let mut idx = tail;
        while idx != head {
            if self.ready[idx].load(Ordering::Acquire) == 1 {
                let event = unsafe { *self.buffer[idx].get() };
                events.push(event);
                self.ready[idx].store(0, Ordering::Release);
                idx = (idx + 1) % QUEUE_SIZE;
            } else {
                break;
            }
        }
        self.tail.store(idx, Ordering::Release);
        events
    }
}

pub fn global_bridge() -> &'static NoAllocBridge {
    static BRIDGE: NoAllocBridge = NoAllocBridge::new();
    &BRIDGE
}


pub struct DiagnosticBus {
    sender: Sender<DiagnosticEvent>,
    receiver: Arc<Mutex<Receiver<DiagnosticEvent>>>,
}

impl DiagnosticBus {
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        Self {
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }

    pub fn emit(&self, event: DiagnosticEvent) {
        let _ = self.sender.send(event);
    }

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

    pub fn try_recv_all(&self) -> Vec<DiagnosticEvent> {
        let mut events = Vec::new();
        
        // Hydrate from bridge
        let bridge_events = global_bridge().pop_all();
        for be in bridge_events {
            let mut metadata = HashMap::new();
            metadata.insert("error_code".to_string(), be.error_code.to_string());
            events.push(DiagnosticEvent {
                severity: Severity::from_u8(be.severity),
                message: be.message.to_string(),
                metadata,
                thread_name: Some("verified_thread".to_string()),
            });
        }
        
        if let Ok(rx) = self.receiver.lock() {
            while let Ok(event) = rx.try_recv() {
                events.push(event);
            }
        }
        events
    }
}

impl Default for DiagnosticBus {
    fn default() -> Self {
        Self::new()
    }
}

pub fn global_bus() -> &'static DiagnosticBus {
    static BUS: OnceLock<DiagnosticBus> = OnceLock::new();
    BUS.get_or_init(DiagnosticBus::new)
}

pub fn emit(event: DiagnosticEvent) {
    global_bus().emit(event);
}

pub fn emit_error<E: Diagnostic>(err: &E) {
    global_bus().emit_error(err);
}

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
