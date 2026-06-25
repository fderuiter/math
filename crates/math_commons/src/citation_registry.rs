use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

pub struct CitationRegistry {
    citations: HashMap<String, String>,
}

impl CitationRegistry {
    pub fn global() -> &'static RwLock<CitationRegistry> {
        static REGISTRY: OnceLock<RwLock<CitationRegistry>> = OnceLock::new();
        REGISTRY.get_or_init(|| RwLock::new(CitationRegistry { citations: HashMap::new() }))
    }

    pub fn register(id: String, citation: String) {
        Self::global().write().unwrap().citations.insert(id, citation);
    }

    pub fn get(id: &str) -> Option<String> {
        Self::global().read().unwrap().citations.get(id).cloned()
    }
}
