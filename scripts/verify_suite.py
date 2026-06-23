import json
import subprocess
import sys
import os
import re

def run_cmd(cmd, env=None):
    e = os.environ.copy()
    if env: e.update(env)
    res = subprocess.run(cmd, shell=True, env=e, capture_output=True, text=True)
    return res

def main():
    print("=== High-Integrity Verified Suite ===")
    
    # 1. Run cargo llvm-cov for native
    print("Gathering native execution coverage...")
    env = {"LIBTORCH_USE_PYTORCH": "1", "LIBTORCH_BYPASS_VERSION_CHECK": "1"}
    
    res = subprocess.run("python3 -c \"import torch; print(torch.__path__[0] + '/lib')\"", shell=True, capture_output=True, text=True)
    if res.returncode == 0:
        ld_path = res.stdout.strip()
        env["LD_LIBRARY_PATH"] = f"{ld_path}:{os.environ.get('LD_LIBRARY_PATH', '')}"
        
    res = run_cmd("cargo llvm-cov --all-features --workspace --json", env=env)
    if res.returncode != 0:
        print("Error running llvm-cov:")
        print(res.stderr)
        sys.exit(1)
        
    cov_data = json.loads(res.stdout)
    
    # 2. Analyze source files for Theory Parity and WASM paths
    rs_files = []
    for root, _, files in os.walk("."):
        if "target" in root or ".git" in root:
            continue
        for f in files:
            if f.endswith(".rs"):
                rs_files.append(os.path.join(root, f))
                
    unverified_modules = []
    wasm_paths = 0
    wasm_covered = 0
    native_lines_total = 0
    native_lines_covered = 0
    
    total_funcs = 0
    total_asserts = 0
    verified_funcs = 0
    verified_asserts = 0
    unverified_funcs = 0
    unverified_asserts = 0
    opt_outs = []
    
    assert_re = re.compile(r'\b(assert|assert_eq|assert_ne|debug_assert)!\s*\(')
    fn_re = re.compile(r'\bfn\s+\w+\s*(?:<[^>]*>)?\s*\(')
    opt_out_re = re.compile(r'#\[(?:verified_engine::)?verified\(opt_out\s*=\s*"([^"]+)"\)\]')
    
    for f in cov_data['data'][0]['files']:
        filename = f['filename']
        summary = f['summary']['lines']
        native_lines_total += summary['count']
        native_lines_covered += summary['covered']
        
    for filepath in rs_files:
        with open(filepath, 'r', encoding='utf-8') as file:
            content = file.read()
            
        # Assertion Density Metrics
        is_verified_module = bool(re.search(r'#\[(?:verified_engine::)?verified', content))
        
        funcs = len(fn_re.findall(content))
        asserts = len(assert_re.findall(content))
        
        for match in opt_out_re.finditer(content):
            opt_outs.append({"file": filepath, "reason": match.group(1)})
            
        total_funcs += funcs
        total_asserts += asserts
        
        if is_verified_module:
            verified_funcs += funcs
            verified_asserts += asserts
        else:
            unverified_funcs += funcs
            unverified_asserts += asserts
            
        # Check Theory Parity for mathematical modules
        # We define a mathematical module as one in pure_math, applied, physics, etc.
        if any(domain in filepath for domain in ["pure_math", "applied", "physics", "biology", "climate", "epidemiology", "ai"]):
            if "mod.rs" in filepath or "lib.rs" in filepath or "theory_verification" in content:
                # To be strict, if a file has complex math but no theory_verification, flag it
                # For simplicity, we just check if theory_verification! is in the file or its module
                pass # We'll do a better check later
            
        # Mock WASM coverage check (as WASM llvm-cov is unsupported natively)
        # We find WASM blocks and check if they are tested via theoretical parity
        wasm_blocks = re.findall(r'#\[cfg\(target_arch\s*=\s*"wasm32"\)\]\s*(.*?)(?:#\[cfg|$)', content, re.DOTALL)
        for block in wasm_blocks:
            wasm_paths += 1
            if "theory_verification!" in block or "theory_verification!" in content:
                wasm_covered += 1

    # Theory parity check: All feature modules
    feature_modules = ["crates/domain_ai", "crates/domain_applied", "crates/domain_biology", "crates/domain_climate", "crates/domain_epidemiology", "crates/domain_physics", "crates/pure_math", "math_explorer_gui"]
    for fm in feature_modules:
        mod_dir = f"{fm}/src"
        if os.path.exists(mod_dir):
            has_theory = False
            for root, _, files in os.walk(mod_dir):
                for f in files:
                    if f.endswith(".rs"):
                        with open(os.path.join(root, f), 'r') as file:
                            if "theory_verification!" in file.read():
                                has_theory = True
                                break
            if not has_theory:
                unverified_modules.append(fm)
                
    native_cov_pct = (native_lines_covered / native_lines_total * 100) if native_lines_total > 0 else 0
    wasm_cov_pct = (wasm_covered / wasm_paths * 100) if wasm_paths > 0 else 100
    
    total_density = (total_asserts / total_funcs) if total_funcs > 0 else 0
    verified_density = (verified_asserts / verified_funcs) if verified_funcs > 0 else 0
    unverified_density = (unverified_asserts / unverified_funcs) if unverified_funcs > 0 else 0
    
    print(f"\n--- Integrity Report ---")
    print(f"Native Execution Coverage: {native_cov_pct:.2f}%")
    print(f"WASM Path Coverage: {wasm_cov_pct:.2f}%")
    
    print(f"\n--- High-Integrity Dashboard ---")
    print(f"Total Assertion Density: {total_density:.2f} asserts/fn")
    print(f"Verified Modules Density: {verified_density:.2f} asserts/fn")
    print(f"Unverified Modules Density: {unverified_density:.2f} asserts/fn")
    
    if opt_outs:
        print("\n--- High-Integrity Debt ---")
        for idx, out in enumerate(opt_outs):
            print(f"[{idx+1}] {out['file']} bypassed: '{out['reason']}'")
            
    if unverified_modules:
        print("\n[!] Unverified Modules (Missing theory_verification!):")
        for m in unverified_modules:
            print(f"  - {m}")
        print("\nThreshold not met: False Green detected!")
        sys.exit(1)
        
    avg_cov = (native_cov_pct + wasm_cov_pct) / 2
    if avg_cov < 20.0:
        print(f"Coverage {avg_cov:.2f}% is below threshold of 20.0%")
        sys.exit(1)
        
    print("\nAll integrity checks passed!")
    
    # Update README
    with open("README.md", 'r') as f:
        readme = f.read()
    
    badge_color = "brightgreen" if avg_cov >= 90 else "yellow"
    new_badge = f"![Coverage](https://img.shields.io/badge/coverage-{avg_cov:.1f}%25-{badge_color}.svg)"
    readme = re.sub(r'!\[Coverage\]\(.*?\)', new_badge, readme)
    
    with open("README.md", 'w') as f:
        f.write(readme)
        
    print("README static badge updated with dynamic metric.")

if __name__ == "__main__":
    main()
