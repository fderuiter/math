//! Legacy crate.
use std::env;
use std::fs;
use std::process::{Command, exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: xtask <command> [args...]");
        println!(
            "Commands: setup, test-features, verify-suite, verify-records, compile-papers, traceability, check-file-lengths, regenerate-baseline"
        );
        exit(1);
    }
    match args[1].as_str() {
        "setup" => setup(),
        "test-features" => test_features(&args[2..]),
        "verify-suite" => verify_suite(&args[2..]),
        "verify-records" => verify_records(),
        "compile-papers" => compile_papers(),
        "traceability" => traceability(),
        "check-file-lengths" => check_file_lengths(),
        "regenerate-baseline" => regenerate_baseline(),
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

    let hook_path = ".git/hooks/pre-commit";
    let hook_content = r#"#!/bin/sh
# auto-generated pre-commit hook

for profile in "$HOME/.bash_profile" "$HOME/.zprofile" "$HOME/.bashrc" "$HOME/.zshrc"; do
    if [ -f "$profile" ]; then
        . "$profile" >/dev/null 2>&1 || true
    fi
done

if ! command -v cargo >/dev/null 2>&1; then
    if [ -d "$HOME/.cargo/bin" ]; then
        export PATH="$PATH:$HOME/.cargo/bin"
    fi
fi

echo "Running centralized verification suite..."
OUTPUT=$(cargo run -p xtask -- check-file-lengths 2>&1)
EXIT_CODE=$?

echo "$OUTPUT"

if [ $EXIT_CODE -ne 0 ]; then
    echo "Verification failed! Commit blocked due to file-length constraints."
    exit 1
fi

case "$OUTPUT" in
    *"File length violation"*)
        echo "Verification failed! Commit blocked due to file-length constraints."
        exit 1
        ;;
esac

exit 0
"#;

    if std::path::Path::new(hook_path).exists() {
        let existing = fs::read_to_string(hook_path).unwrap_or_default();
        if !existing.contains("auto-generated pre-commit hook") {
            println!(
                "Warning: A custom pre-commit hook exists at {}. Please merge the verification check manually or remove it to allow auto-installation.",
                hook_path
            );
        } else {
            fs::write(hook_path, hook_content).unwrap();
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(hook_path).unwrap().permissions();
                perms.set_mode(0o755);
                fs::set_permissions(hook_path, perms).unwrap();
            }
            println!("Pre-commit hook updated.");
        }
    } else {
        if let Some(parent) = std::path::Path::new(hook_path).parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(hook_path, hook_content).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(hook_path).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(hook_path, perms).unwrap();
        }
        println!("Pre-commit hook installed.");
    }

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

fn verify_suite(args: &[String]) {
    println!("=== High-Integrity Verified Suite ===");
    let mut cmd_args = vec![
        "run",
        "-p",
        "unified_verification",
        "--release",
        "--",
        "verify-suite",
    ];
    let args_str: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    cmd_args.extend(args_str);
    run_cmd("cargo", &cmd_args);
}

fn verify_records() {
    println!("=== Verify Records ===");
    run_cmd(
        "cargo",
        &[
            "run",
            "-p",
            "unified_verification",
            "--release",
            "--",
            "verify-records",
        ],
    );
}

fn compile_papers() {
    println!("=== Compiling Papers with Tectonic ===");
    
    let check = Command::new("tectonic").arg("--version").output();
    match check {
        Ok(output) if output.status.success() => {}
        _ => {
            eprintln!("Error: Tectonic is not installed or not in PATH.");
            eprintln!("Please install Tectonic 0.15.0 to compile academic papers:");
            eprintln!("curl -sL \"https://github.com/tectonic-typesetting/tectonic/releases/download/tectonic%400.15.0/tectonic-0.15.0-x86_64-unknown-linux-gnu.tar.gz\" | tar xz");
            eprintln!("sudo mv tectonic /usr/local/bin/");
            exit(1);
        }
    }

    let papers_dir = std::path::Path::new("papers");
    let output_dir = std::path::Path::new("papers/output");
    
    if !output_dir.exists() {
        fs::create_dir_all(output_dir).unwrap();
    }

    for entry in fs::read_dir(papers_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() && path.extension().is_some_and(|ext| ext == "tex") {
            println!("Compiling {:?}", path);
            let status = Command::new("tectonic")
                .arg("-X")
                .arg("compile")
                .arg("--outdir")
                .arg(output_dir)
                .arg(&path)
                .status();
                
            match status {
                Ok(s) if s.success() => println!("Successfully compiled {:?}", path),
                _ => {
                    eprintln!("Failed to compile {:?}", path);
                    exit(1);
                }
            }
        }
    }
    println!("=== All papers compiled successfully ===");
}

fn traceability() {
    println!("=== Traceability Report ===");
    run_cmd(
        "cargo",
        &[
            "run",
            "-p",
            "unified_verification",
            "--release",
            "--",
            "traceability",
        ],
    );
}

fn check_file_lengths() {
    println!("=== Check File Lengths ===");
    run_cmd(
        "cargo",
        &[
            "run",
            "-p",
            "unified_verification",
            "--release",
            "--",
            "check-file-lengths",
        ],
    );
}
fn regenerate_baseline() {
    println!("=== Regenerate Public API Baseline ===");
    run_cmd(
        "cargo",
        &[
            "run",
            "-p",
            "unified_verification",
            "--release",
            "--",
            "regenerate-baseline",
        ],
    );
}
