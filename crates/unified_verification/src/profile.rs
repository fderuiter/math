use std::fs;
use std::path::Path;
use toml::Value;

pub fn check_profiles(workspace_members: &[&str]) -> bool {
    let mut passed = true;
    for member in workspace_members {
        let path = Path::new(member).join("Cargo.toml");
        if path.exists() {
            let content = fs::read_to_string(&path).unwrap();
            let parsed: Value = match toml::from_str(&content) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Failed to parse TOML for {}: {}", path.display(), e);
                    continue;
                }
            };

            let has_overflow_checks = parsed
                .get("profile")
                .and_then(|p| p.get("release"))
                .and_then(|r| r.get("overflow-checks"))
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            if !has_overflow_checks {
                eprintln!(
                    "[!] Profile mismatch in {}: missing or incorrect [profile.release] overflow-checks = true",
                    path.display()
                );
                passed = false;
            }
        }
    }
    passed
}
