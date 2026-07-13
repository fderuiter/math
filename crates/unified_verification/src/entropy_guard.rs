use std::collections::HashMap;
use std::fs;
use syn::visit::Visit;
use walkdir::WalkDir;

struct EntropyVisitor {
    aliases: HashMap<String, String>,
    violations: Vec<(String, String)>,
}

fn flatten_use_tree(tree: &syn::UseTree, prefix: &mut Vec<String>, mappings: &mut HashMap<String, String>) {
    match tree {
        syn::UseTree::Path(p) => {
            prefix.push(p.ident.to_string());
            flatten_use_tree(&p.tree, prefix, mappings);
            prefix.pop();
        }
        syn::UseTree::Name(n) => {
            let mut full_path = prefix.clone();
            full_path.push(n.ident.to_string());
            mappings.insert(n.ident.to_string(), full_path.join("::"));
        }
        syn::UseTree::Rename(r) => {
            let mut full_path = prefix.clone();
            full_path.push(r.ident.to_string());
            mappings.insert(r.rename.to_string(), full_path.join("::"));
        }
        syn::UseTree::Group(g) => {
            for item in &g.items {
                flatten_use_tree(item, prefix, mappings);
            }
        }
        syn::UseTree::Glob(_) => {}
    }
}

fn resolve_path(p: &syn::Path, mappings: &HashMap<String, String>) -> String {
    let mut parts = Vec::new();
    for (i, seg) in p.segments.iter().enumerate() {
        if i == 0 {
            let first = seg.ident.to_string();
            if let Some(mapped) = mappings.get(&first) {
                parts.push(mapped.clone());
            } else {
                parts.push(first);
            }
        } else {
            parts.push(seg.ident.to_string());
        }
    }
    parts.join("::")
}

impl<'ast> Visit<'ast> for EntropyVisitor {
    fn visit_item_use(&mut self, i: &'ast syn::ItemUse) {
        let mut prefix = Vec::new();
        flatten_use_tree(&i.tree, &mut prefix, &mut self.aliases);
        syn::visit::visit_item_use(self, i);
    }

    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        if let syn::Expr::Path(p) = &*i.func {
            let resolved = resolve_path(&p.path, &self.aliases);
            let ident_str = p.path.segments.last().unwrap().ident.to_string();
            if resolved.contains("thread_rng") || resolved.ends_with("::thread_rng") || resolved == "thread_rng" {
                self.violations.push((ident_str.clone(), "thread_rng()".to_string()));
            }
            if resolved.contains("random") || resolved.ends_with("::random") || resolved == "random" {
                self.violations.push((ident_str.clone(), "random()".to_string()));
            }
            if resolved.contains("SystemTime::now") || resolved.ends_with("::now") && resolved.contains("SystemTime") {
                self.violations.push((ident_str, "SystemTime::now()".to_string()));
            }
        }
        syn::visit::visit_expr_call(self, i);
    }
}

pub fn check_entropy(members: &[String]) -> Vec<String> {
    println!("Running Entropy Guard...");
    let mut all_violations = Vec::new();

    for dir in members {
        if !std::path::Path::new(dir).exists() {
            continue;
        }
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            let path_str = entry.path().to_string_lossy();
            if path_str.contains("/target/") || path_str.contains("\\target\\") {
                continue;
            }
            if entry.path().extension().and_then(|s| s.to_str()) == Some("rs") {
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    if let Ok(ast) = syn::parse_file(&content) {
                        let mut visitor = EntropyVisitor {
                            aliases: HashMap::new(),
                            violations: Vec::new(),
                        };
                        visitor.visit_file(&ast);
                        
                        let lines: Vec<&str> = content.lines().collect();
                        
                        for (ident_str, name) in visitor.violations {
                            // Find line manually
                            let mut found_line = 0;
                            for (i, line) in lines.iter().enumerate() {
                                if line.contains(&ident_str) {
                                    // Make sure it's not the `allow(entropy_guard)` line
                                    if !line.contains("allow(entropy_guard)") {
                                        found_line = i + 1;
                                        break;
                                    }
                                }
                            }
                            if found_line == 0 { found_line = 1; }
                            
                            let mut ignored = false;
                            if found_line > 0 && found_line <= lines.len() {
                                let target_line = lines[found_line - 1];
                                if target_line.contains("allow(entropy_guard)") {
                                    ignored = true;
                                }
                                if found_line > 1 && lines[found_line - 2].contains("allow(entropy_guard)") {
                                    ignored = true;
                                }
                            }
                            if !ignored {
                                all_violations.push(format!(
                                    "Entropy Guard Violation: Prohibited pattern '{}' found in {} at line {}",
                                    name,
                                    entry.path().display(),
                                    found_line
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    all_violations
}
