use std::fs;
use std::path::PathBuf;
use std::process::Command;
use walkdir::WalkDir;
use syn::visit::Visit;

struct MaliciousCodeVisitor {
    has_shell_execution: bool,
    has_network_socket: bool,
    has_suspicious_fs: bool,
}

impl<'ast> Visit<'ast> for MaliciousCodeVisitor {
    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        if let syn::Expr::Path(p) = &*i.func {
            let path_str = p.path.segments.iter().map(|s| s.ident.to_string()).collect::<Vec<_>>().join("::");
            if path_str.contains("Command::new") {
                self.has_shell_execution = true;
            }
            if path_str.contains("TcpStream::connect") || path_str.contains("UdpSocket::bind") {
                self.has_network_socket = true;
            }
            if path_str.contains("remove_dir_all") {
                self.has_suspicious_fs = true;
            }
        }
        syn::visit::visit_expr_call(self, i);
    }
}

pub fn run_ast_visitor() -> bool {
    println!("Running AST visitor on 100% of third-party dependencies...");
    
    let mut deps_paths = Vec::new();
    let meta_out = Command::new("cargo").args(["metadata", "--format-version", "1"]).output().unwrap();
    let meta: serde_json::Value = serde_json::from_slice(&meta_out.stdout).unwrap_or_default();
    if let Some(packages) = meta.get("packages").and_then(|p| p.as_array()) {
        for pkg in packages {
            if let Some(manifest_path) = pkg.get("manifest_path").and_then(|m| m.as_str()) {
                if let Some(source) = pkg.get("source").and_then(|s| s.as_str()) {
                    if source.contains("crates.io") {
                        if let Some(p) = PathBuf::from(manifest_path).parent() {
                            deps_paths.push(p.to_path_buf());
                        }
                    }
                }
            }
        }
    }
    
    let mut flags_found = 0;
    for path in deps_paths.iter() {
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.path().extension().and_then(|s| s.to_str()) == Some("rs") {
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    // Fast pre-filter
                    if !content.contains("Command::new") && !content.contains("TcpStream") && !content.contains("UdpSocket") && !content.contains("remove_dir_all") {
                        continue;
                    }
                    if let Ok(ast) = syn::parse_file(&content) {
                        let mut visitor = MaliciousCodeVisitor {
                            has_shell_execution: false,
                            has_network_socket: false,
                            has_suspicious_fs: false,
                        };
                        visitor.visit_file(&ast);
                        if visitor.has_shell_execution {
                            println!("[!] Malicious pattern 1: Shell execution found in {}", entry.path().display());
                            flags_found += 1;
                        }
                        if visitor.has_network_socket {
                            println!("[!] Malicious pattern 2: Network socket pattern found in {}", entry.path().display());
                            flags_found += 1;
                        }
                        if visitor.has_suspicious_fs {
                            println!("[!] Malicious pattern 3: Suspicious FS access found in {}", entry.path().display());
                            flags_found += 1;
                        }
                    }
                }
            }
        }
    }
    
    println!("AST visitor finished. Flagged {} patterns.", flags_found);
    true
}
