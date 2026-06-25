use oxidize_core::traceability::TraceabilityEngine;
use oxidize_core::vfs::{DefaultVfs, VirtualFileSystem};
use std::process;

fn main() {
    let vfs = DefaultVfs;
    let engine = TraceabilityEngine::new(&vfs);

    let mut code_dirs = vec!["math_explorer/src".to_string(), "math_explorer_gui/src/tabs".to_string()];
    
    // Add crate dirs
    if let Ok(crates) = vfs.list_dir("crates") {
        for crate_name in crates {
            code_dirs.push(format!("crates/{}/src", crate_name));
        }
    }

    let all_dirs: Vec<&str> = code_dirs.iter().map(|s| s.as_str()).collect();

    match engine.scan_repository(&all_dirs, "papers") {
        Ok(report) => {
            println!("=== Traceability Report ===");
            println!("Summary: Scanned {} source files.", report.scanned_files);

            let mut failed = false;
            if !report.invalid_links.is_empty() {
                println!("\n=== Validation Failures ===");
                println!("Invalid citations were found (see details above).");
                for (file, cite) in &report.invalid_links {
                    println!("   [!] INVALID citation: {} in {}", cite, file);
                }
                failed = true;
            }

            if !report.orphaned_papers.is_empty() {
                if !failed {
                    println!("\n=== Validation Failures ===");
                }
                println!("Orphaned papers found: {:?}", report.orphaned_papers);
                failed = true;
            }

            if failed {
                process::exit(1);
            }
            println!("All checks passed!");
        }
        Err(e) => {
            eprintln!("Error scanning repository: {}", e);
            process::exit(1);
        }
    }
}
