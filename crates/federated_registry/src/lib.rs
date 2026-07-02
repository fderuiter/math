use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex, OnceLock};

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

#[derive(Debug, Clone)]
pub struct TelemetryEvent {
    pub source: String,
    pub severity: Severity,
    pub message: String,
    pub metadata: HashMap<String, String>,
    pub thread_name: Option<String>,
}

pub struct FederatedRegistry {
    sender: Sender<TelemetryEvent>,
    receiver: Arc<Mutex<Receiver<TelemetryEvent>>>,
    sources: Arc<Mutex<Vec<String>>>,
}

impl FederatedRegistry {
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        Self {
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
            sources: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn register_source(&self, name: &str) {
        let mut sources = self.sources.lock().unwrap();
        if !sources.contains(&name.to_string()) {
            sources.push(name.to_string());
        }
    }

    pub fn known_sources(&self) -> Vec<String> {
        self.sources.lock().unwrap().clone()
    }

    pub fn emit(&self, event: TelemetryEvent) {
        let _ = self.sender.send(event);
    }

    pub fn try_recv_all(&self) -> Vec<TelemetryEvent> {
        let mut events = Vec::new();
        if let Ok(rx) = self.receiver.lock() {
            while let Ok(event) = rx.try_recv() {
                events.push(event);
            }
        }
        events
    }
}

impl Default for FederatedRegistry {
    fn default() -> Self {
        Self::new()
    }
}

pub fn global_registry() -> &'static FederatedRegistry {
    static REGISTRY: OnceLock<FederatedRegistry> = OnceLock::new();
    REGISTRY.get_or_init(FederatedRegistry::new)
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

        global_registry().emit(TelemetryEvent {
            source: "PanicHook".to_string(),
            severity: Severity::Fatal,
            message: format!("Thread panicked: {}", message),
            metadata,
            thread_name: std::thread::current().name().map(|s| s.to_string()),
        });

        default_hook(panic_info);
    }));
}
