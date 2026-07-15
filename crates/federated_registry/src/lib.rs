//! Legacy crate.
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex, OnceLock};

pub use diagnostics::Severity;

#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub struct TelemetryEvent {
    #[allow(missing_docs)]
    pub source: String,
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
pub struct FederatedRegistry {
    sender: Sender<TelemetryEvent>,
    receiver: Arc<Mutex<Receiver<TelemetryEvent>>>,
    sources: Arc<Mutex<Vec<String>>>,
}

impl FederatedRegistry {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        Self {
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
            sources: Arc::new(Mutex::new(Vec::new())),
        }
    }

    #[allow(missing_docs)]
    pub fn register_source(&self, name: &str) {
        let mut sources = self.sources.lock().unwrap();
        if !sources.contains(&name.to_string()) {
            sources.push(name.to_string());
        }
    }

    #[allow(missing_docs)]
    pub fn known_sources(&self) -> Vec<String> {
        self.sources.lock().unwrap().clone()
    }

    #[allow(missing_docs)]
    pub fn emit(&self, event: TelemetryEvent) {
        let _ = self.sender.send(event);
    }

    #[allow(missing_docs)]
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

#[allow(missing_docs)]
pub fn global_registry() -> &'static FederatedRegistry {
    static REGISTRY: OnceLock<FederatedRegistry> = OnceLock::new();
    REGISTRY.get_or_init(|| {
        let registry = FederatedRegistry::new();
        let sender = registry.sender.clone();
        diagnostics::global_bus().register_listener(move |event| {
            let source = event.metadata.get("source")
                .cloned()
                .unwrap_or_else(|| "diagnostics".to_string());
            let _ = sender.send(TelemetryEvent {
                source,
                severity: event.severity.clone(),
                message: event.message.clone(),
                metadata: event.metadata.clone(),
                thread_name: event.thread_name.clone(),
            });
        });
        registry
    })
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
