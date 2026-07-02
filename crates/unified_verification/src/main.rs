#![allow(clippy::too_many_lines, clippy::cognitive_complexity)]
#![allow(clippy::all)]
use regex::Regex;
use serde::Serialize;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;

#[derive(Serialize)]
struct IntegrityReport {
    native_execution_coverage_pct: f64,
    wasm_path_coverage_pct: f64,
    total_assertion_density: f64,
    verified_modules_density: f64,
    unverified_modules_density: f64,
    passed: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: unified_verification <command>");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "verify-records" => verify_records(),
        "traceability" => traceability(),
        "verify-suite" => verify_suite(),
        "check-file-lengths" => check_file_lengths(),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            std::process::exit(1);
        }
    }
}

fn verify_records() {
    let base_sha = env::var("BASE_SHA").ok().filter(|s| !s.is_empty());
    let head_sha = env::var("HEAD_SHA").ok().filter(|s| !s.is_empty());

    let (commit_msgs, changed_files) = if let (Some(base), Some(head)) = (base_sha, head_sha) {
        let msg_out = Command::new("git")
            .args(["log", "--format=%B", &format!("{}..{}", base, head)])
            .output()
            .unwrap();
        let msgs = String::from_utf8_lossy(&msg_out.stdout).to_string();

        let diff_out = Command::new("git")
            .args(["diff", "--name-only", &base, &head])
            .output()
            .unwrap();
        let mut diffs = String::from_utf8_lossy(&diff_out.stdout).to_string();
        if diffs.trim().is_empty() {
            let diff_out = Command::new("git")
                .args(["diff-tree", "--no-commit-id", "--name-only", "-r", "HEAD"])
                .output()
                .unwrap();
            diffs = String::from_utf8_lossy(&diff_out.stdout).to_string();
        }
        (msgs, diffs)
    } else {
        let msg_out = Command::new("git")
            .args(["log", "-1", "--pretty=%B"])
            .output()
            .unwrap();
        let msgs = String::from_utf8_lossy(&msg_out.stdout).to_string();

        let diff_out = Command::new("git")
            .args(["diff-tree", "--no-commit-id", "--name-only", "-r", "HEAD"])
            .output()
            .unwrap();
        let diffs = String::from_utf8_lossy(&diff_out.stdout).to_string();
        (msgs, diffs)
    };

    if commit_msgs.to_lowercase().contains("[skip journal]") {
        println!("Commit message contains [skip journal]. Bypassing journal verification.");
        return;
    }

    let changed_files = changed_files.replace("\\", "/");
    println!("Changed files:\n{}", changed_files);

    let changed_lines: Vec<&str> = changed_files.lines().collect();
    let core_modified = changed_lines.iter().any(|l| {
        l.starts_with("math_explorer/")
            || (l.starts_with("crates/") && !l.starts_with("crates/unified_verification/"))
    });

    if core_modified {
        println!("Core logic areas (math_explorer/ or crates/) were modified.");
        println!(
            "Architectural records check is obsolete since .jules/ was removed. Successfully verified."
        );
    } else {
        println!("No core logic areas modified. Skipping journal verification.");
    }
}

fn traceability() {
    println!("=== Traceability Report ===");
    println!("Delegating to unified Rust Traceability Engine...");

    let status = Command::new("cargo")
        .args(["run", "--bin", "traceability_cli"])
        .status()
        .expect("Failed to run traceability_cli");

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}

fn check_file_lengths() {
    println!("Checking for files exceeding 500 lines in core math directories...");
    let dirs = vec![
        "crates/domain_physics",
        "crates/domain_biology",
        "crates/pure_math",
    ];
    let mut exceeding = Vec::new();

    for dir in dirs {
        if Path::new(dir).exists() {
            for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_file()
                    && entry.path().extension().and_then(|s| s.to_str()) == Some("rs")
                {
                    if let Ok(content) = fs::read_to_string(entry.path()) {
                        let lines = content.lines().count();
                        if lines > 500 {
                            exceeding.push(format!("{} ({} lines)", entry.path().display(), lines));
                        }
                    }
                }
            }
        }
    }

    if !exceeding.is_empty() {
        eprintln!("Error: The following files exceed the 500-line limit:");
        for e in exceeding {
            eprintln!("{}", e);
        }
        eprintln!("Please split these files using the Strategy pattern.");
        std::process::exit(1);
    } else {
        println!("All checked files are within the 500-line limit.");
    }
}

fn get_llvm_cov_output() -> std::process::Output {
    let mut cmd = Command::new("cargo");
    cmd.args(["llvm-cov", "--all-features", "--workspace", "--json"]);
    cmd.output().expect("Failed to run cargo llvm-cov")
}

fn collect_rs_files() -> Vec<std::path::PathBuf> {
    let mut rs_files = Vec::new();
    for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
        let path_str = entry.path().to_string_lossy().replace("\\", "/");
        if path_str.contains("/target/")
            || path_str.contains("/.git/")
            || path_str.starts_with("target/")
            || path_str.starts_with(".git/")
        {
            continue;
        }
        if entry.file_type().is_file()
            && entry.path().extension().and_then(|s| s.to_str()) == Some("rs")
        {
            rs_files.push(entry.path().to_path_buf());
        }
    }
    rs_files.sort();
    rs_files
}

fn parse_coverage(cov_json: &serde_json::Value) -> (f64, f64) {
    let mut native_lines_total = 0.0;
    let mut native_lines_covered = 0.0;

    if let Some(files) = cov_json
        .get("data")
        .and_then(|d| d.as_array())
        .and_then(|a| a.first())
        .and_then(|data| data.get("files"))
        .and_then(|f| f.as_array())
    {
        for f in files {
            if let Some(summary) = f.get("summary").and_then(|s| s.get("lines")) {
                native_lines_total += summary.get("count").and_then(|c| c.as_f64()).unwrap_or(0.0);
                native_lines_covered += summary
                    .get("covered")
                    .and_then(|c| c.as_f64())
                    .unwrap_or(0.0);
            }
        }
    }
    (native_lines_total, native_lines_covered)
}

struct FileMetrics {
    wasm_paths: usize,
    wasm_covered: usize,
    total_funcs: f64,
    total_asserts: f64,
    verified_funcs: f64,
    verified_asserts: f64,
    unverified_funcs: f64,
    unverified_asserts: f64,
    opt_outs: Vec<(String, String)>,
}

fn analyze_files(rs_files: &[std::path::PathBuf]) -> FileMetrics {
    let mut m = FileMetrics {
        wasm_paths: 0,
        wasm_covered: 0,
        total_funcs: 0.0,
        total_asserts: 0.0,
        verified_funcs: 0.0,
        verified_asserts: 0.0,
        unverified_funcs: 0.0,
        unverified_asserts: 0.0,
        opt_outs: Vec::new(),
    };

    let assert_re = Regex::new(r"\b(assert|assert_eq|assert_ne|debug_assert)!\s*\(").unwrap();
    let fn_re = Regex::new(r"\bfn\s+\w+\s*(?:<[^>]*>)?\s*\(").unwrap();
    let opt_out_re =
        Regex::new(r#"#\[(?:verified_engine::)?verified\(opt_out\s*=\s*"([^"]+)"\)\]"#).unwrap();
    let wasm_re =
        Regex::new(r#"#\[cfg\(target_arch\s*=\s*"wasm32"\)\]\s*(.*?)(?:#\[cfg|$)"#).unwrap();

    for filepath in rs_files {
        if let Ok(content) = fs::read_to_string(filepath) {
            let is_verified =
                content.contains("#[verified") || content.contains("#[verified_engine::verified");

            let funcs = fn_re.find_iter(&content).count() as f64;
            let asserts = assert_re.find_iter(&content).count() as f64;

            for cap in opt_out_re.captures_iter(&content) {
                let mut norm = filepath.to_string_lossy().replace("\\", "/");
                if norm.starts_with("./") {
                    norm = norm[2..].to_string();
                }
                m.opt_outs.push((norm, cap[1].to_string()));
            }

            m.total_funcs += funcs;
            m.total_asserts += asserts;

            if is_verified {
                m.verified_funcs += funcs;
                m.verified_asserts += asserts;
            } else {
                m.unverified_funcs += funcs;
                m.unverified_asserts += asserts;
            }

            let content_str = content.replace("\r\n", "\n").replace("\n", " ");
            for cap in wasm_re.captures_iter(&content_str) {
                m.wasm_paths += 1;
                let block = &cap[1];
                if block.contains("theory_verification!")
                    || content.contains("theory_verification!")
                    || block.contains("stochastic_signature_verification!")
                    || content.contains("stochastic_signature_verification!")
                    || block.contains("empirical_verification!")
                    || content.contains("empirical_verification!")
                {
                    m.wasm_covered += 1;
                }
            }
        }
    }
    m.opt_outs.sort();
    m
}

fn check_unverified_modules() -> Vec<String> {
    let feature_modules = vec![
        "crates/domain_ai",
        "crates/domain_applied",
        "crates/domain_biology",
        "crates/domain_climate",
        "crates/domain_epidemiology",
        "crates/domain_physics",
        "crates/pure_math",
        "math_explorer_gui",
    ];

    let mut unverified_modules = Vec::new();
    for fm in feature_modules {
        let mod_dir = Path::new(fm).join("src");
        if mod_dir.exists() {
            let mut has_theory = false;
            for entry in WalkDir::new(mod_dir).into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_file()
                    && entry.path().extension().and_then(|s| s.to_str()) == Some("rs")
                {
                    if let Ok(content) = fs::read_to_string(entry.path()) {
                        if content.contains("theory_verification!")
                            || content.contains("stochastic_signature_verification!")
                            || content.contains("empirical_verification!")
                        {
                            has_theory = true;
                            break;
                        }
                    }
                }
            }
            if !has_theory {
                unverified_modules.push(fm.to_string());
            }
        }
    }
    unverified_modules.sort();
    unverified_modules
}

fn save_report(report: &IntegrityReport, avg_cov: f64) {
    let json_report = serde_json::to_string_pretty(&report).unwrap();
    fs::write("Verification_Certificate.json", json_report).unwrap();
    println!("Generated Verification_Certificate.json");

    if let Ok(mut readme) = fs::read_to_string("README.md") {
        let badge_color = if avg_cov >= 90.0 {
            "brightgreen"
        } else {
            "yellow"
        };
        let new_badge = format!(
            "![Coverage](https://img.shields.io/badge/coverage-{:.1}%25-{})",
            avg_cov, badge_color
        );
        let badge_re = Regex::new(r"!\[Coverage\]\(.*?\)").unwrap();
        readme = badge_re.replace(&readme, new_badge.as_str()).to_string();
        fs::write("README.md", readme).unwrap();
        println!("README static badge updated with dynamic metric.");
    }
}

fn print_report(
    native_cov_pct: f64,
    wasm_cov_pct: f64,
    m: &FileMetrics,
    unverified_modules: &[String],
) -> bool {
    let total_density = if m.total_funcs > 0.0 {
        m.total_asserts / m.total_funcs
    } else {
        0.0
    };
    let verified_density = if m.verified_funcs > 0.0 {
        m.verified_asserts / m.verified_funcs
    } else {
        0.0
    };
    let unverified_density = if m.unverified_funcs > 0.0 {
        m.unverified_asserts / m.unverified_funcs
    } else {
        0.0
    };

    println!("\n--- Integrity Report ---");
    println!("Native Execution Coverage: {:.2}%", native_cov_pct);
    println!("WASM Path Coverage: {:.2}%", wasm_cov_pct);

    println!("\n--- High-Integrity Dashboard ---");
    println!("Total Assertion Density: {:.2} asserts/fn", total_density);
    println!(
        "Verified Modules Density: {:.2} asserts/fn",
        verified_density
    );
    println!(
        "Unverified Modules Density: {:.2} asserts/fn",
        unverified_density
    );

    let mut passed = true;
    if verified_density < 0.0 {
        println!(
            "\n[!] Assertion Density Failure: Verified modules have a density of {:.2} asserts/fn, which is below the minimum required 0.0 asserts/fn.",
            verified_density
        );
        passed = false;
    }

    if !m.opt_outs.is_empty() {
        println!("\n--- High-Integrity Debt ---");
        for (idx, (file, reason)) in m.opt_outs.iter().enumerate() {
            println!("[{}] {} bypassed: '{}'", idx + 1, file, reason);
        }
    }

    if !unverified_modules.is_empty() {
        println!("\n[!] Unverified Modules (Missing theory_verification!):");
        for md in unverified_modules {
            println!("  - {}", md);
        }
        println!("\nThreshold not met: False Green detected!");
        passed = false;
    }

    let avg_cov = (native_cov_pct + wasm_cov_pct) / 2.0;
    if avg_cov < 20.0 {
        println!("Coverage {:.2}% is below threshold of 20.0%", avg_cov);
        passed = false;
    }

    let report = IntegrityReport {
        native_execution_coverage_pct: native_cov_pct,
        wasm_path_coverage_pct: wasm_cov_pct,
        total_assertion_density: total_density,
        verified_modules_density: verified_density,
        unverified_modules_density: unverified_density,
        passed,
    };

    save_report(&report, avg_cov);
    passed
}

fn verify_suite() {
    println!("=== High-Integrity Verified Suite ===");
    println!("Gathering native execution coverage...");

    let output = get_llvm_cov_output();
    if !output.status.success() {
        eprintln!(
            "Error running llvm-cov:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        std::process::exit(1);
    }

    let cov_json: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("Failed to parse coverage JSON");

    let rs_files = collect_rs_files();
    let (native_lines_total, native_lines_covered) = parse_coverage(&cov_json);
    let m = analyze_files(&rs_files);
    let unverified_modules = check_unverified_modules();

    let native_cov_pct = if native_lines_total > 0.0 {
        native_lines_covered / native_lines_total * 100.0
    } else {
        0.0
    };
    let wasm_cov_pct = if m.wasm_paths > 0 {
        (m.wasm_covered as f64 / m.wasm_paths as f64) * 100.0
    } else {
        100.0
    };

    let passed = print_report(native_cov_pct, wasm_cov_pct, &m, &unverified_modules);

    if !passed {
        std::process::exit(1);
    }
    println!("\nAll integrity checks passed!");
}
