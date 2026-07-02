use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn check_file_lengths(members: &[String]) -> Vec<String> {
    let mut exceeding = Vec::new();

    for dir in members {
        if Path::new(dir).exists() {
            for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
                let path_str = entry.path().to_string_lossy();
                if path_str.contains("/target/") || path_str.contains("\\target\\") {
                    continue;
                }
                if entry.file_type().is_file()
                    && entry.path().extension().and_then(|s| s.to_str()) == Some("rs")
                {
                    if let Ok(content) = fs::read_to_string(entry.path()) {
                        let lines = content.lines().count();
                        if lines > 500 {
                            exceeding.push(format!(
                                "File length violation: {} ({} lines)",
                                entry.path().display(),
                                lines
                            ));
                        }
                    }
                }
            }
        }
    }
    exceeding
}
