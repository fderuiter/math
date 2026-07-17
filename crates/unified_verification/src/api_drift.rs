use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use syn::visit::Visit;
use walkdir::WalkDir;

#[derive(Default)]
struct ApiVisitor {
    pub items: Vec<String>,
}

impl<'ast> Visit<'ast> for ApiVisitor {
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        if matches!(i.vis, syn::Visibility::Public(_)) {
            let sig = &i.sig;
            self.items.push(quote::quote!(pub #sig).to_string());
        }
        syn::visit::visit_item_fn(self, i);
    }

    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        if matches!(i.vis, syn::Visibility::Public(_)) {
            let ident = &i.ident;
            let generics = &i.generics;
            self.items
                .push(quote::quote!(pub struct #ident #generics).to_string());
        }
        syn::visit::visit_item_struct(self, i);
    }

    fn visit_item_enum(&mut self, i: &'ast syn::ItemEnum) {
        if matches!(i.vis, syn::Visibility::Public(_)) {
            let ident = &i.ident;
            let generics = &i.generics;
            self.items
                .push(quote::quote!(pub enum #ident #generics).to_string());
        }
        syn::visit::visit_item_enum(self, i);
    }

    fn visit_item_trait(&mut self, i: &'ast syn::ItemTrait) {
        if matches!(i.vis, syn::Visibility::Public(_)) {
            let ident = &i.ident;
            let generics = &i.generics;
            self.items
                .push(quote::quote!(pub trait #ident #generics).to_string());
        }
        syn::visit::visit_item_trait(self, i);
    }

    fn visit_item_type(&mut self, i: &'ast syn::ItemType) {
        if matches!(i.vis, syn::Visibility::Public(_)) {
            let ident = &i.ident;
            let generics = &i.generics;
            let ty = &i.ty;
            self.items
                .push(quote::quote!(pub type #ident #generics = #ty).to_string());
        }
        syn::visit::visit_item_type(self, i);
    }
}

pub fn extract_apis() -> BTreeMap<String, Vec<String>> {
    let mut apis = BTreeMap::new();
    let mut dirs_to_scan = vec![PathBuf::from("math_explorer/src")];

    if let Ok(entries) = fs::read_dir("crates") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = path.file_name().unwrap_or_default().to_string_lossy();
                if name != "unified_verification" {
                    dirs_to_scan.push(path.join("src"));
                }
            }
        }
    }

    for dir in dirs_to_scan {
        if !dir.exists() {
            continue;
        }
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file()
                && entry.path().extension().and_then(|s| s.to_str()) == Some("rs")
            {
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    if let Ok(ast) = syn::parse_file(&content) {
                        let mut visitor = ApiVisitor::default();
                        visitor.visit_file(&ast);
                        if !visitor.items.is_empty() {
                            let mut items = visitor.items;
                            items.sort();
                            let norm_path = entry.path().to_string_lossy().replace("\\", "/");
                            apis.insert(norm_path, items);
                        }
                    }
                }
            }
        }
    }
    apis
}

const BASELINE_PATH: &str = "crates/unified_verification/api_baseline.json";

pub fn regenerate_baseline() {
    let apis = extract_apis();
    let json = serde_json::to_string_pretty(&apis).unwrap();
    fs::write(BASELINE_PATH, json).expect("Failed to write baseline");
    println!("API baseline regenerated successfully.");
}

pub fn check_api_drift() -> bool {
    let current_apis = extract_apis();
    let baseline_content = fs::read_to_string(BASELINE_PATH).unwrap_or_else(|_| "{}".to_string());
    let baseline_apis: BTreeMap<String, Vec<String>> =
        serde_json::from_str(&baseline_content).unwrap_or_default();

    let mut changed = false;

    for (path, items) in &current_apis {
        if let Some(base_items) = baseline_apis.get(path) {
            if items != base_items {
                println!("API Drift detected in {}:", path);
                for item in items {
                    if !base_items.contains(item) {
                        println!("  + {}", item);
                    }
                }
                for item in base_items {
                    if !items.contains(item) {
                        println!("  - {}", item);
                    }
                }
                changed = true;
            }
        } else {
            println!("API Drift: New file with public API: {}", path);
            changed = true;
        }
    }

    for (path, _) in &baseline_apis {
        if !current_apis.contains_key(path) {
            println!("API Drift: File with public API removed: {}", path);
            changed = true;
        }
    }

    if changed {
        println!("Public API drift detected! Please regenerate the baseline and write an ADR.");
        return false;
    }

    println!("Public API drift check passed.");
    true
}
