use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use serde_yaml::Value;

pub fn lint_workflows() -> bool {
    let workflows_dir = Path::new(".github/workflows");
    if !workflows_dir.exists() {
        println!("No .github/workflows directory found. Skipping workflow linting.");
        return true;
    }

    let mut all_passed = true;

    for entry in WalkDir::new(workflows_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_file() {
            let path_str = entry.path().to_string_lossy();
            if path_str.ends_with(".yml") || path_str.ends_with(".yaml") {
                let content = match fs::read_to_string(entry.path()) {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("Error reading {}: {}", path_str, e);
                        all_passed = false;
                        continue;
                    }
                };

                let yaml: Value = match serde_yaml::from_str(&content) {
                    Ok(y) => y,
                    Err(e) => {
                        eprintln!("Invalid YAML syntax in {}: {}", path_str, e);
                        all_passed = false;
                        continue;
                    }
                };

                // Check for global permissions block
                let has_permissions = yaml.get("permissions").is_some();
                if !has_permissions {
                    eprintln!(
                        "Error in {}: Workflow is missing a global 'permissions' block.",
                        path_str
                    );
                    all_passed = false;
                    continue;
                }

                let permissions = yaml.get("permissions").unwrap();
                let mut is_read_only = false;

                if let Value::String(s) = permissions {
                    if s == "read-all" || s == "none" {
                        is_read_only = true;
                    }
                } else if let Value::Mapping(map) = permissions {
                    let mut only_read = true;
                    for (_k, v) in map {
                        if let Value::String(v_str) = v {
                            if v_str != "read" && v_str != "none" {
                                only_read = false;
                                break;
                            }
                        } else {
                            only_read = false;
                            break;
                        }
                    }
                    if only_read {
                        is_read_only = true;
                    }
                }

                if !is_read_only {
                    eprintln!(
                        "Error in {}: Workflow 'permissions' block must explicitly be read-only globally. (e.g., 'permissions: read-all' or specific 'read' permissions)",
                        path_str
                    );
                    all_passed = false;
                }
            }
        }
    }

    if all_passed {
        println!("All workflows passed linting (read-only permissions enforced).");
    }

    all_passed
}
