import re
content = open('/app/crates/unified_verification/src/main.rs').read()
content = re.sub(
    r'fn get_llvm_cov_output\(\) -> std::process::Output \{.*?\n\}',
    '''fn get_llvm_cov_output() -> std::process::Output {
    let mut cmd = Command::new("cargo");
    cmd.args(["llvm-cov", "--all-features", "--workspace", "--json"]);
    cmd.output().expect("Failed to run cargo llvm-cov")
}''',
    content,
    flags=re.DOTALL
)
open('/app/crates/unified_verification/src/main.rs', 'w').write(content)
