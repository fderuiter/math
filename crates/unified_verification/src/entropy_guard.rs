use regex::Regex;
use std::fs;
use walkdir::WalkDir;

pub fn check_entropy(members: &[String]) -> Vec<String> {
    println!("Running Entropy Guard...");
    let forbidden_patterns = [
        (
            Regex::new(r"\bthread_rng\s*\(").unwrap(),
            "thread_rng()",
            "Use oxidize_core::rng::OxidizeRng or inject a seeded RNG.",
        ),
        (
            Regex::new(r"\brandom\s*\(").unwrap(),
            "random()",
            "Use oxidize_core::rng::OxidizeRng or inject a seeded RNG.",
        ),
        (
            Regex::new(r"\bSystemTime::now\s*\(").unwrap(),
            "SystemTime::now()",
            "Use oxidize_core::rng::OxidizeRng or inject a seeded RNG.",
        ),
    ];

    let ignore_re = Regex::new(r"allow\(entropy_guard\)").unwrap();

    let mut violations = Vec::new();

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
                    let lines: Vec<&str> = content.lines().collect();
                    for (i, line) in lines.iter().enumerate() {
                        if ignore_re.is_match(line) || (i > 0 && ignore_re.is_match(lines[i - 1])) {
                            continue; // Skip if ignored
                        }
                        for (re, name, _suggestion) in forbidden_patterns.iter() {
                            if re.is_match(line) {
                                violations.push(format!(
                                    "Entropy Guard Violation: Prohibited pattern '{}' found in {} at line {}",
                                    name,
                                    entry.path().display(),
                                    i + 1
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    violations
}
