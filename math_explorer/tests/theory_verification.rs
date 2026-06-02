#![allow(clippy::all)]
use std::fs;
use std::path::Path;

#[test]
fn test_theory_verification_coverage_and_parity() {
    let papers_dir = Path::new("/app/papers");
    let _applied_dir = Path::new("/app/crates/domain_applied/src/applied");

    // 1. Get all papers
    let mut papers = vec![];
    if let Ok(entries) = fs::read_dir(papers_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("tex") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    papers.push(stem.to_string());
                }
            }
        }
    }

    // 2. Get all applied and physics modules
    let mut target_modules = vec![];
    for dir in &[
        Path::new("/app/crates/domain_applied/src/applied"),
        Path::new("/app/crates/domain_physics/src/physics"),
    ] {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        target_modules.push((dir.to_path_buf(), name.to_string()));
                    }
                }
            }
        }
    }

    // 3. Check for missing implementations for existing papers (warnings)
    for paper in &papers {
        let mut found = false;
        for (_, module) in &target_modules {
            // Naming parity check or simple mapping
            if module == paper || paper.starts_with(module) || module.starts_with(paper) {
                found = true;
                break;
            }
        }
        if !found {
            // We only warn for missing implementations
            println!(
                "cargo:warning=Missing implementation for paper: {}.tex",
                paper
            );
        }
    }

    // 4. Check that 100% of modules in the `applied` and `physics` domains have a corresponding "Theory Verification" test.
    let mut missing_tests = vec![];
    for (parent_dir, module) in &target_modules {
        let module_dir = parent_dir.join(module);

        let mut has_test = false;
        // Check mod.rs or any .rs file in the module for `theory_verification!` or `mod theory_verification`
        let mut check_dir = vec![module_dir];
        while let Some(dir) = check_dir.pop() {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        check_dir.push(path);
                    } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                        if let Ok(content) = fs::read_to_string(&path) {
                            if content.contains("theory_verification!")
                                || content.contains("mod theory_verification")
                            {
                                has_test = true;
                                break;
                            }
                        }
                    }
                }
            }
            if has_test {
                break;
            }
        }

        if !has_test {
            missing_tests.push(format!(
                "{}/{}",
                parent_dir.file_name().unwrap().to_string_lossy(),
                module
            ));
        }
    }

    if !missing_tests.is_empty() {
        panic!(
            "The following applied modules are missing Theory Verification tests: {:?}",
            missing_tests
        );
    }
}
