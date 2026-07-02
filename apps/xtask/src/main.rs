use std::env;
use std::fs;
use std::process::{Command, exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: xtask <command> [args...]");
        println!(
            "Commands: setup, test-features, verify-suite, verify-records, compile-papers, traceability, check-file-lengths"
        );
        exit(1);
    }
    match args[1].as_str() {
        "setup" => setup(),
        "test-features" => test_features(&args[2..]),
        "verify-suite" => verify_suite(),
        "verify-records" => verify_records(),
        "compile-papers" => compile_papers(),
        "traceability" => traceability(),
        "check-file-lengths" => check_file_lengths(),
        _ => {
            println!("Unknown command");
            exit(1);
        }
    }
}

fn run_cmd(cmd: &str, args: &[&str]) {
    println!("Running: {} {}", cmd, args.join(" "));
    let status = Command::new(cmd).args(args).status();
    match status {
        Ok(s) if s.success() => (),
        _ => {
            eprintln!("Command failed!");
            exit(1);
        }
    }
}

fn setup() {
    println!("=== Math Explorer Setup Script ===");
    run_cmd("cargo", &["build"]);
    run_cmd("cargo", &["test"]);
    fs::create_dir_all(".jules/personal").unwrap();
    println!("=== Setup Complete ===");
}

fn test_features(args: &[String]) {
    if args.is_empty() {
        let features = [
            "pure_math",
            "applied",
            "ai",
            "biology",
            "climate",
            "epidemiology",
            "physics",
        ];
        println!("=== Running core-only (no features) ===");
        run_cmd(
            "cargo",
            &["test", "-p", "math_explorer", "--no-default-features"],
        );
        for f in features.iter() {
            println!("=== Running tests for feature: {} ===", f);
            run_cmd(
                "cargo",
                &[
                    "test",
                    "-p",
                    "math_explorer",
                    "--no-default-features",
                    "--features",
                    f,
                ],
            );
        }
        println!("=== Running all features ===");
        run_cmd("cargo", &["test", "-p", "math_explorer", "--all-features"]);
        println!("All feature combinations passed successfully!");
    } else {
        let feature = &args[0];
        if feature == "core-only" || feature.is_empty() {
            run_cmd(
                "cargo",
                &["test", "-p", "math_explorer", "--no-default-features"],
            );
        } else {
            run_cmd(
                "cargo",
                &[
                    "test",
                    "-p",
                    "math_explorer",
                    "--no-default-features",
                    "--features",
                    feature,
                ],
            );
        }
    }
}

fn verify_suite() {
    println!("=== High-Integrity Verified Suite ===");
    run_cmd("cargo", &["run", "-p", "unified_verification", "--release", "--", "verify-suite"]);
}

fn verify_records() {
    println!("=== Verify Records ===");
    run_cmd("cargo", &["run", "-p", "unified_verification", "--release", "--", "verify-records"]);
}

fn compile_papers() {
    println!("Compiling papers skipped in native Rust tool...");
}

fn traceability() {
    println!("=== Traceability Report ===");
    run_cmd("cargo", &["run", "-p", "unified_verification", "--release", "--", "traceability"]);
}

fn check_file_lengths() {
    println!("=== Check File Lengths ===");
    run_cmd("cargo", &["run", "-p", "unified_verification", "--release", "--", "check-file-lengths"]);
}
