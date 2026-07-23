use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use syn::visit::Visit;
use walkdir::WalkDir;

#[derive(Serialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ApiItem {
    pub signature: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_expires: Option<String>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ApiItemShim {
    Simple(String),
    Detailed {
        signature: String,
        deprecation_expires: Option<String>,
    },
}

impl From<ApiItemShim> for ApiItem {
    fn from(shim: ApiItemShim) -> Self {
        match shim {
            ApiItemShim::Simple(sig) => ApiItem {
                signature: sig,
                deprecation_expires: None,
            },
            ApiItemShim::Detailed {
                signature,
                deprecation_expires,
            } => ApiItem {
                signature,
                deprecation_expires,
            },
        }
    }
}

impl<'de> Deserialize<'de> for ApiItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let shim = ApiItemShim::deserialize(deserializer)?;
        Ok(ApiItem::from(shim))
    }
}

fn extract_expiration_date(attrs: &[syn::Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("deprecated") {
            let attr_str = quote::quote!(#attr).to_string();
            if let Some(pos) = attr_str.find("expires") {
                if let Some(date) = find_date_pattern(&attr_str[pos..]) {
                    return Some(date);
                }
            }
            if let Some(pos) = attr_str.find("expiry") {
                if let Some(date) = find_date_pattern(&attr_str[pos..]) {
                    return Some(date);
                }
            }
            if let Some(date) = find_date_pattern(&attr_str) {
                return Some(date);
            }
        }
    }
    None
}

fn find_date_pattern(s: &str) -> Option<String> {
    let bytes = s.as_bytes();
    if bytes.len() < 10 {
        return None;
    }
    for i in 0..=(bytes.len() - 10) {
        let is_digit = |b: u8| b.is_ascii_digit();
        if is_digit(bytes[i])
            && is_digit(bytes[i + 1])
            && is_digit(bytes[i + 2])
            && is_digit(bytes[i + 3])
            && bytes[i + 4] == b'-'
            && is_digit(bytes[i + 5])
            && is_digit(bytes[i + 6])
            && bytes[i + 7] == b'-'
            && is_digit(bytes[i + 8])
            && is_digit(bytes[i + 9])
        {
            return Some(s[i..i + 10].to_string());
        }
    }
    None
}

#[derive(Clone, Debug)]
pub struct ParsedModule {
    pub file_path: PathBuf,
    pub is_pub: bool,
    pub pub_items: Vec<String>,
    pub re_exports: Vec<ReExport>,
}

#[derive(Clone, Debug)]
pub struct ReExport {
    pub source_path: String,
    pub is_glob: bool,
}

fn find_module_file(parent_file: &std::path::Path, mod_name: &str) -> Option<PathBuf> {
    let parent_dir = parent_file.parent()?;
    let stem = parent_file.file_stem()?.to_str()?;
    
    let mut candidates = Vec::new();
    if stem == "lib" || stem == "main" || parent_file.ends_with("mod.rs") {
        candidates.push(parent_dir.join(format!("{}.rs", mod_name)));
        candidates.push(parent_dir.join(mod_name).join("mod.rs"));
    } else {
        candidates.push(parent_dir.join(stem).join(format!("{}.rs", mod_name)));
        candidates.push(parent_dir.join(stem).join(mod_name).join("mod.rs"));
    }
    
    for cb in candidates {
        if cb.exists() {
            return Some(cb);
        }
    }
    None
}

fn resolve_path<F>(current_module: &str, path: &str, module_exists: F) -> String
where
    F: Fn(&str) -> bool,
{
    if path == "crate" {
        return "crate".to_string();
    }
    if path.starts_with("crate::") {
        return path.to_string();
    }
    if path.starts_with("self::") {
        let suffix = &path[6..];
        return if current_module == "crate" {
            format!("crate::{}", suffix)
        } else {
            format!("{}::{}", current_module, suffix)
        };
    }
    if path.starts_with("super::") {
        let suffix = &path[7..];
        if let Some(pos) = current_module.rfind("::") {
            let parent = &current_module[..pos];
            return format!("{}::{}", parent, suffix);
        } else {
            return format!("crate::{}", suffix);
        }
    }
    
    let first_segment = path.split("::").next().unwrap_or("");
    
    let candidate = if current_module == "crate" {
        format!("crate::{}", first_segment)
    } else {
        format!("{}::{}", current_module, first_segment)
    };
    if module_exists(&candidate) {
        return if current_module == "crate" {
            format!("crate::{}", path)
        } else {
            format!("{}::{}", current_module, path)
        };
    }
    
    let candidate_root = format!("crate::{}", first_segment);
    if module_exists(&candidate_root) {
        return format!("crate::{}", path);
    }
    
    if current_module == "crate" {
        format!("crate::{}", path)
    } else {
        format!("{}::{}", current_module, path)
    }
}

fn flatten_use_tree(prefix: &str, tree: &syn::UseTree, out: &mut Vec<(String, Option<String>, bool)>) {
    match tree {
        syn::UseTree::Name(name) => {
            let full_path = if prefix.is_empty() {
                name.ident.to_string()
            } else {
                format!("{}::{}", prefix, name.ident)
            };
            out.push((full_path, None, false));
        }
        syn::UseTree::Rename(rename) => {
            let full_path = if prefix.is_empty() {
                rename.ident.to_string()
            } else {
                format!("{}::{}", prefix, rename.ident)
            };
            out.push((full_path, Some(rename.rename.to_string()), false));
        }
        syn::UseTree::Glob(_) => {
            out.push((prefix.to_string(), None, true));
        }
        syn::UseTree::Path(path) => {
            let new_prefix = if prefix.is_empty() {
                path.ident.to_string()
            } else {
                format!("{}::{}", prefix, path.ident)
            };
            flatten_use_tree(&new_prefix, &path.tree, out);
        }
        syn::UseTree::Group(group) => {
            for item in &group.items {
                flatten_use_tree(prefix, item, out);
            }
        }
    }
}

fn has_cfg_test(attrs: &[syn::Attribute]) -> bool {
    for attr in attrs {
        if attr.path().is_ident("cfg") {
            let attr_str = quote::quote!(#attr).to_string();
            if attr_str.contains("test") {
                return true;
            }
        }
    }
    false
}

fn normalize_path_for_comparison(p: &std::path::Path) -> String {
    let s = p.to_string_lossy().replace("\\", "/");
    if s.starts_with("./") {
        s[2..].to_string()
    } else {
        s
    }
}

fn parse_module_recursive(
    module_path: String,
    file_path: PathBuf,
    is_pub: bool,
    module_map: &mut BTreeMap<String, ParsedModule>,
) {
    if !file_path.exists() {
        return;
    }
    let Ok(content) = fs::read_to_string(&file_path) else {
        return;
    };
    let Ok(ast) = syn::parse_file(&content) else {
        return;
    };

    let mut pub_items = Vec::new();
    let mut re_exports = Vec::new();
    let mut submodules_to_parse = Vec::new();

    for item in &ast.items {
        match item {
            syn::Item::Fn(i) => {
                if has_cfg_test(&i.attrs) {
                    continue;
                }
                if matches!(i.vis, syn::Visibility::Public(_)) {
                    pub_items.push(i.sig.ident.to_string());
                }
            }
            syn::Item::Struct(i) => {
                if has_cfg_test(&i.attrs) {
                    continue;
                }
                if matches!(i.vis, syn::Visibility::Public(_)) {
                    pub_items.push(i.ident.to_string());
                }
            }
            syn::Item::Enum(i) => {
                if has_cfg_test(&i.attrs) {
                    continue;
                }
                if matches!(i.vis, syn::Visibility::Public(_)) {
                    pub_items.push(i.ident.to_string());
                }
            }
            syn::Item::Trait(i) => {
                if has_cfg_test(&i.attrs) {
                    continue;
                }
                if matches!(i.vis, syn::Visibility::Public(_)) {
                    pub_items.push(i.ident.to_string());
                }
            }
            syn::Item::Type(i) => {
                if has_cfg_test(&i.attrs) {
                    continue;
                }
                if matches!(i.vis, syn::Visibility::Public(_)) {
                    pub_items.push(i.ident.to_string());
                }
            }
            syn::Item::Mod(i) => {
                let mod_name = i.ident.to_string();
                if mod_name == "tests" || has_cfg_test(&i.attrs) {
                    continue;
                }
                let is_mod_pub = matches!(i.vis, syn::Visibility::Public(_));
                let sub_module_path = format!("{}::{}", module_path, mod_name);

                if let Some((_, ref content)) = i.content {
                    parse_inline_module(
                        sub_module_path,
                        file_path.clone(),
                        is_mod_pub,
                        content,
                        module_map,
                    );
                } else {
                    if let Some(sub_file) = find_module_file(&file_path, &mod_name) {
                        submodules_to_parse.push((sub_module_path, sub_file, is_mod_pub));
                    }
                }
            }
            syn::Item::Use(i) => {
                if has_cfg_test(&i.attrs) {
                    continue;
                }
                if matches!(i.vis, syn::Visibility::Public(_)) {
                    let mut flattened = Vec::new();
                    flatten_use_tree("", &i.tree, &mut flattened);
                    for (path_str, _rename, is_glob) in flattened {
                        re_exports.push(ReExport {
                            source_path: path_str,
                            is_glob,
                        });
                    }
                }
            }
            _ => {}
        }
    }

    module_map.insert(
        module_path.clone(),
        ParsedModule {
            file_path,
            is_pub,
            pub_items,
            re_exports,
        },
    );

    for (sub_path, sub_file, sub_pub) in submodules_to_parse {
        parse_module_recursive(sub_path, sub_file, sub_pub, module_map);
    }
}

fn parse_inline_module(
    module_path: String,
    file_path: PathBuf,
    is_pub: bool,
    items: &[syn::Item],
    module_map: &mut BTreeMap<String, ParsedModule>,
) {
    let mut pub_items = Vec::new();
    let mut re_exports = Vec::new();
    let mut submodules_to_parse = Vec::new();

    for item in items {
        match item {
            syn::Item::Fn(i) => {
                if has_cfg_test(&i.attrs) {
                    continue;
                }
                if matches!(i.vis, syn::Visibility::Public(_)) {
                    pub_items.push(i.sig.ident.to_string());
                }
            }
            syn::Item::Struct(i) => {
                if has_cfg_test(&i.attrs) {
                    continue;
                }
                if matches!(i.vis, syn::Visibility::Public(_)) {
                    pub_items.push(i.ident.to_string());
                }
            }
            syn::Item::Enum(i) => {
                if has_cfg_test(&i.attrs) {
                    continue;
                }
                if matches!(i.vis, syn::Visibility::Public(_)) {
                    pub_items.push(i.ident.to_string());
                }
            }
            syn::Item::Trait(i) => {
                if has_cfg_test(&i.attrs) {
                    continue;
                }
                if matches!(i.vis, syn::Visibility::Public(_)) {
                    pub_items.push(i.ident.to_string());
                }
            }
            syn::Item::Type(i) => {
                if has_cfg_test(&i.attrs) {
                    continue;
                }
                if matches!(i.vis, syn::Visibility::Public(_)) {
                    pub_items.push(i.ident.to_string());
                }
            }
            syn::Item::Mod(i) => {
                let mod_name = i.ident.to_string();
                if mod_name == "tests" || has_cfg_test(&i.attrs) {
                    continue;
                }
                let is_mod_pub = matches!(i.vis, syn::Visibility::Public(_));
                let sub_module_path = format!("{}::{}", module_path, mod_name);

                if let Some((_, ref content)) = i.content {
                    parse_inline_module(
                        sub_module_path,
                        file_path.clone(),
                        is_mod_pub,
                        content,
                        module_map,
                    );
                } else {
                    if let Some(sub_file) = find_module_file(&file_path, &mod_name) {
                        submodules_to_parse.push((sub_module_path, sub_file, is_mod_pub));
                    }
                }
            }
            syn::Item::Use(i) => {
                if has_cfg_test(&i.attrs) {
                    continue;
                }
                if matches!(i.vis, syn::Visibility::Public(_)) {
                    let mut flattened = Vec::new();
                    flatten_use_tree("", &i.tree, &mut flattened);
                    for (path_str, _rename, is_glob) in flattened {
                        re_exports.push(ReExport {
                            source_path: path_str,
                            is_glob,
                        });
                    }
                }
            }
            _ => {}
        }
    }

    module_map.insert(
        module_path.clone(),
        ParsedModule {
            file_path,
            is_pub,
            pub_items,
            re_exports,
        },
    );

    for (sub_path, sub_file, sub_pub) in submodules_to_parse {
        parse_module_recursive(sub_path, sub_file, sub_pub, module_map);
    }
}

fn is_direct_child(parent: &str, child: &str) -> bool {
    if !child.starts_with(parent) {
        return false;
    }
    let suffix = &child[parent.len()..];
    suffix.starts_with("::") && !suffix[2..].contains("::")
}

struct FilePubVisitor {
    pub_items: Vec<(String, usize, String, Option<String>, Vec<String>)>,
    current_inline_path: Vec<String>,
}

impl<'ast> Visit<'ast> for FilePubVisitor {
    fn visit_item_mod(&mut self, i: &'ast syn::ItemMod) {
        let mod_name = i.ident.to_string();
        if mod_name == "tests" || has_cfg_test(&i.attrs) {
            return;
        }
        self.current_inline_path.push(mod_name);
        syn::visit::visit_item_mod(self, i);
        self.current_inline_path.pop();
    }

    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        if has_cfg_test(&i.attrs) {
            return;
        }
        if matches!(i.vis, syn::Visibility::Public(_)) {
            use syn::spanned::Spanned;
            let sig = &i.sig;
            self.pub_items.push((
                i.sig.ident.to_string(),
                i.span().start().line,
                quote::quote!(pub #sig).to_string(),
                extract_expiration_date(&i.attrs),
                self.current_inline_path.clone(),
            ));
        }
        syn::visit::visit_item_fn(self, i);
    }

    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        if has_cfg_test(&i.attrs) {
            return;
        }
        if matches!(i.vis, syn::Visibility::Public(_)) {
            use syn::spanned::Spanned;
            let ident = &i.ident;
            let generics = &i.generics;
            self.pub_items.push((
                ident.to_string(),
                i.span().start().line,
                quote::quote!(pub struct #ident #generics).to_string(),
                extract_expiration_date(&i.attrs),
                self.current_inline_path.clone(),
            ));
        }
        syn::visit::visit_item_struct(self, i);
    }

    fn visit_item_enum(&mut self, i: &'ast syn::ItemEnum) {
        if has_cfg_test(&i.attrs) {
            return;
        }
        if matches!(i.vis, syn::Visibility::Public(_)) {
            use syn::spanned::Spanned;
            let ident = &i.ident;
            let generics = &i.generics;
            self.pub_items.push((
                ident.to_string(),
                i.span().start().line,
                quote::quote!(pub enum #ident #generics).to_string(),
                extract_expiration_date(&i.attrs),
                self.current_inline_path.clone(),
            ));
        }
        syn::visit::visit_item_enum(self, i);
    }

    fn visit_item_trait(&mut self, i: &'ast syn::ItemTrait) {
        if has_cfg_test(&i.attrs) {
            return;
        }
        if matches!(i.vis, syn::Visibility::Public(_)) {
            use syn::spanned::Spanned;
            let ident = &i.ident;
            let generics = &i.generics;
            self.pub_items.push((
                ident.to_string(),
                i.span().start().line,
                quote::quote!(pub trait #ident #generics).to_string(),
                extract_expiration_date(&i.attrs),
                self.current_inline_path.clone(),
            ));
        }
        syn::visit::visit_item_trait(self, i);
    }

    fn visit_item_type(&mut self, i: &'ast syn::ItemType) {
        if has_cfg_test(&i.attrs) {
            return;
        }
        if matches!(i.vis, syn::Visibility::Public(_)) {
            use syn::spanned::Spanned;
            let ident = &i.ident;
            let generics = &i.generics;
            let ty = &i.ty;
            self.pub_items.push((
                ident.to_string(),
                i.span().start().line,
                quote::quote!(pub type #ident #generics = #ty).to_string(),
                extract_expiration_date(&i.attrs),
                self.current_inline_path.clone(),
            ));
        }
        syn::visit::visit_item_type(self, i);
    }
}

pub fn scan_workspace_apis() -> (BTreeMap<String, Vec<ApiItem>>, Vec<(String, usize, String)>) {
    let mut reachable_apis = BTreeMap::new();
    let mut unreachable_apis = Vec::new();

    let mut package_src_dirs = vec![PathBuf::from("math_explorer/src")];

    if let Ok(entries) = fs::read_dir("crates") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = path.file_name().unwrap_or_default().to_string_lossy();
                if name != "unified_verification" {
                    package_src_dirs.push(path.join("src"));
                }
            }
        }
    }

    for src_dir in package_src_dirs {
        if !src_dir.exists() {
            continue;
        }

        let mut root_file = src_dir.join("lib.rs");
        if !root_file.exists() {
            root_file = src_dir.join("main.rs");
        }
        if !root_file.exists() {
            continue;
        }

        let mut module_map = BTreeMap::new();
        parse_module_recursive(
            "crate".to_string(),
            root_file.clone(),
            true,
            &mut module_map,
        );

        let mut reachable_modules = std::collections::BTreeSet::new();
        reachable_modules.insert("crate".to_string());

        let mut reachable_items = std::collections::BTreeSet::new();

        let mut changed = true;
        while changed {
            changed = false;
            let current_modules: Vec<String> = reachable_modules.iter().cloned().collect();

            for m in &current_modules {
                if let Some(parsed_mod) = module_map.get(m) {
                    for (key, sub_mod) in &module_map {
                        if is_direct_child(m, key) && sub_mod.is_pub {
                            if reachable_modules.insert(key.clone()) {
                                changed = true;
                            }
                        }
                    }

                    for item in &parsed_mod.pub_items {
                        let item_path = if m == "crate" {
                            format!("crate::{}", item)
                        } else {
                            format!("{}::{}", m, item)
                        };
                        if reachable_items.insert(item_path) {
                            changed = true;
                        }
                    }

                    for re_exp in &parsed_mod.re_exports {
                        let resolved = resolve_path(m, &re_exp.source_path, |path| module_map.contains_key(path));

                        if re_exp.is_glob {
                            if module_map.contains_key(&resolved) {
                                if reachable_modules.insert(resolved.clone()) {
                                    changed = true;
                                }
                            }
                        } else {
                            if module_map.contains_key(&resolved) {
                                if reachable_modules.insert(resolved.clone()) {
                                    changed = true;
                                }
                            } else {
                                if reachable_items.insert(resolved.clone()) {
                                    changed = true;
                                }
                                if let Some(pos) = resolved.rfind("::") {
                                    let parent_mod = &resolved[..pos];
                                    let item_name = &resolved[pos + 2..];
                                    if let Some(parent_parsed) = module_map.get(parent_mod) {
                                        if parent_parsed.pub_items.iter().any(|i| i == item_name) {
                                            if reachable_items.insert(resolved.clone()) {
                                                changed = true;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        for entry in WalkDir::new(&src_dir).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file()
                && entry.path().extension().and_then(|s| s.to_str()) == Some("rs")
            {
                let norm_path = entry.path().to_string_lossy().replace("\\", "/");
                
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    if let Ok(ast) = syn::parse_file(&content) {
                        let mut visitor = FilePubVisitor {
                            pub_items: Vec::new(),
                            current_inline_path: Vec::new(),
                        };
                        visitor.visit_file(&ast);

                        let mut matching_modules = Vec::new();
                        let norm_entry_path = normalize_path_for_comparison(entry.path());
                        for (m_path, parsed_m) in &module_map {
                            if normalize_path_for_comparison(&parsed_m.file_path) == norm_entry_path {
                                matching_modules.push(m_path.clone());
                            }
                        }

                        let mut file_reachable_items = Vec::new();

                        for (name, line, signature, deprecation_expires, inline_segs) in visitor.pub_items {
                            let mut is_reachable = false;
                            for m in &matching_modules {
                                let mut item_m = m.clone();
                                for seg in &inline_segs {
                                    item_m = format!("{}::{}", item_m, seg);
                                }
                                let item_path = if item_m == "crate" {
                                    format!("crate::{}", name)
                                } else {
                                    format!("{}::{}", item_m, name)
                                };
                                if reachable_items.contains(&item_path) {
                                    is_reachable = true;
                                    break;
                                }
                            }

                            if is_reachable {
                                file_reachable_items.push(ApiItem {
                                    signature,
                                    deprecation_expires,
                                });
                            } else {
                                unreachable_apis.push((norm_path.clone(), line, signature));
                            }
                        }

                        if !file_reachable_items.is_empty() {
                            file_reachable_items.sort();
                            reachable_apis.insert(norm_path, file_reachable_items);
                        }
                    }
                }
            }
        }
    }

    (reachable_apis, unreachable_apis)
}

pub fn extract_apis() -> BTreeMap<String, Vec<ApiItem>> {
    let (reachable, _) = scan_workspace_apis();
    reachable
}

const BASELINE_PATH: &str = "crates/unified_verification/api_baseline.json";

pub fn regenerate_baseline() {
    let apis = extract_apis();
    let json = serde_json::to_string_pretty(&apis).unwrap();
    fs::write(BASELINE_PATH, json).expect("Failed to write baseline");
    println!("API baseline regenerated successfully.");
}

pub fn check_api_drift() -> bool {
    let (current_apis, unreachable_apis) = scan_workspace_apis();
    if !unreachable_apis.is_empty() {
        println!("Error: Found unreachable public declarations in the workspace!");
        println!("Public items must be reachable from their package entry points, or have restricted visibility (e.g. pub(crate)).");
        for (file_path, line, signature) in &unreachable_apis {
            println!("  -> {} (at {}:{})", signature, file_path, line);
        }
        println!("Please restrict visibility of these items or make them reachable.");
        return false;
    }

    let baseline_content = fs::read_to_string(BASELINE_PATH).unwrap_or_else(|_| "{}".to_string());
    let baseline_apis: BTreeMap<String, Vec<ApiItem>> =
        serde_json::from_str(&baseline_content).unwrap_or_default();

    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let mut changed = false;

    // Check current APIs against baseline
    for (path, items) in &current_apis {
        // Find if any currently existing API is past its expiration
        for item in items {
            if let Some(ref date) = item.deprecation_expires {
                if date.as_str() <= today.as_str() {
                    println!(
                        "API Expired: {} in {} (Expired: {})",
                        item.signature, path, date
                    );
                    println!(
                        "  This deprecated API has passed its expiration target and must be removed."
                    );
                    changed = true;
                }
            }
        }

        if let Some(base_items) = baseline_apis.get(path) {
            // Check for added/removed
            let mut local_changed = false;
            for item in items {
                if !base_items.iter().any(|b| b.signature == item.signature) {
                    println!("  + {}", item.signature);
                    local_changed = true;
                }
            }
            for item in base_items {
                if !items.iter().any(|c| c.signature == item.signature) {
                    // Item was deleted. Check if it's a valid bypass.
                    if let Some(ref date) = item.deprecation_expires {
                        if date.as_str() <= today.as_str() {
                            println!(
                                "  ~ {} (Deleted, bypassed drift check due to expired deprecation)",
                                item.signature
                            );
                            continue; // Skip flagging as changed!
                        }
                    }
                    println!("  - {}", item.signature);
                    local_changed = true;
                }
            }
            if local_changed {
                println!("API Drift detected in {}:", path);
                changed = true;
            }
        } else {
            println!("API Drift: New file with public API: {}", path);
            changed = true;
        }
    }

    // Check files that were completely removed
    for (path, base_items) in &baseline_apis {
        if !current_apis.contains_key(path) {
            let mut all_bypassed = true;
            for item in base_items {
                if let Some(ref date) = item.deprecation_expires {
                    if date.as_str() <= today.as_str() {
                        // bypassed
                        println!(
                            "  ~ {} in {} (File deleted, bypassed due to expired deprecation)",
                            item.signature, path
                        );
                        continue;
                    }
                }
                all_bypassed = false;
            }
            if !all_bypassed {
                println!("API Drift: File with public API removed: {}", path);
                changed = true;
            }
        }
    }

    if changed {
        println!("Public API drift detected! Please regenerate the baseline and write an ADR.");
        return false;
    }

    println!("Public API drift check passed.");
    true
}
