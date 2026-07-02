#[cfg(not(target_arch = "wasm32"))]
use oxidize_core::traceability::TraceabilityEngine;
#[cfg(not(target_arch = "wasm32"))]
use oxidize_core::vfs::DefaultVfs;
#[cfg(not(target_arch = "wasm32"))]
use std::process;

#[cfg(not(target_arch = "wasm32"))]
fn check_and_print_errors(report: &oxidize_core::traceability::TraceabilityReport) -> bool {
    let mut failed = false;

    if !report.unlinked_code.is_empty() {
        println!("\n=== Unlinked Modules ===");
        println!(
            "The following modules contain a verification macro but are missing from the registry:"
        );
        for file in &report.unlinked_code {
            println!("   [!] UNLINKED module: {}", file);
        }
        failed = true;
    }

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

    if !report.unverified_modules.is_empty() {
        println!("\n[!] Unverified Modules (Missing theory_verification! or #[verified]):");
        for module in &report.unverified_modules {
            println!("  - {}", module);
        }
        println!("\nThreshold not met: False Green detected!");
        failed = true;
    }

    failed
}

#[cfg(not(target_arch = "wasm32"))]
fn print_dashboard(report: &oxidize_core::traceability::TraceabilityReport) {
    println!("\n=== High-Integrity Dashboard ===");
    let total_density = if report.total_funcs > 0 {
        report.total_asserts as f64 / report.total_funcs as f64
    } else {
        0.0
    };
    let verified_density = if report.verified_funcs > 0 {
        report.verified_asserts as f64 / report.verified_funcs as f64
    } else {
        0.0
    };
    let unverified_funcs = report.total_funcs.saturating_sub(report.verified_funcs);
    let unverified_asserts = report.total_asserts.saturating_sub(report.verified_asserts);
    let unverified_density = if unverified_funcs > 0 {
        unverified_asserts as f64 / unverified_funcs as f64
    } else {
        0.0
    };

    println!("Total Assertion Density: {:.2} asserts/fn", total_density);
    println!(
        "Verified Modules Density: {:.2} asserts/fn",
        verified_density
    );
    println!(
        "Unverified Modules Density: {:.2} asserts/fn",
        unverified_density
    );

    if report.verified_funcs > 0 && verified_density < 2.0 {
        println!(
            "\n[!] Assertion Density Failure: Verified modules have a density of {:.2} asserts/fn, which is below the minimum required 2.0 asserts/fn.",
            verified_density
        );
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let vfs = DefaultVfs;
    let engine = TraceabilityEngine::new(&vfs);

    let code_dirs = vec![
        "math_explorer/src".to_string(),
        "math_explorer_gui/src/tabs".to_string(),
        "crates/domain_ai/src".to_string(),
        "crates/domain_applied/src".to_string(),
        "crates/domain_biology/src".to_string(),
        "crates/domain_climate/src".to_string(),
        "crates/domain_epidemiology/src".to_string(),
        "crates/domain_physics/src".to_string(),
        "crates/pure_math/src".to_string(),
    ];

    let all_dirs: Vec<&str> = code_dirs.iter().map(|s| s.as_str()).collect();

    match engine.scan_repository(&all_dirs, "papers") {
        Ok(report) => {
            println!("=== Traceability Report ===");
            println!("Summary: Scanned {} source files.", report.scanned_files);

            let failed = check_and_print_errors(&report);

            if failed {
                process::exit(1);
            }

            print_dashboard(&report);

            println!("All checks passed!");
        }
        Err(e) => {
            eprintln!("Error scanning repository: {}", e);
            process::exit(1);
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {}
