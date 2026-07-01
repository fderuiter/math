import re
content = open('/app/apps/xtask/src/main.rs').read()

# Add check-file-lengths to the help output
content = content.replace(
    '"Commands: setup, test-features, verify-suite, verify-records, compile-papers, traceability"',
    '"Commands: setup, test-features, verify-suite, verify-records, compile-papers, traceability, check-file-lengths"'
)

# Add it to the match block
content = content.replace(
    '"traceability" => traceability(),',
    '"traceability" => traceability(),\n        "check-file-lengths" => check_file_lengths(),'
)

# Replace the functions
content = re.sub(r'fn verify_suite\(\) \{.*?\n\}', '''fn verify_suite() {
    println!("=== High-Integrity Verified Suite ===");
    run_cmd("cargo", &["run", "-p", "unified_verification", "--release", "--", "verify-suite"]);
}''', content, flags=re.DOTALL)

content = re.sub(r'fn verify_records\(\) \{.*?\n\}', '''fn verify_records() {
    println!("=== Verify Records ===");
    run_cmd("cargo", &["run", "-p", "unified_verification", "--release", "--", "verify-records"]);
}''', content, flags=re.DOTALL)

content = re.sub(r'fn traceability\(\) \{.*?\n\}', '''fn traceability() {
    println!("=== Traceability Report ===");
    run_cmd("cargo", &["run", "-p", "unified_verification", "--release", "--", "traceability"]);
}

fn check_file_lengths() {
    println!("=== Check File Lengths ===");
    run_cmd("cargo", &["run", "-p", "unified_verification", "--release", "--", "check-file-lengths"]);
}''', content, flags=re.DOTALL)

open('/app/apps/xtask/src/main.rs', 'w').write(content)
