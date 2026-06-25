use crate::vfs::VirtualFileSystem;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TraceabilityReport {
    pub scanned_files: usize,
    pub invalid_links: Vec<(String, String)>,
    pub orphaned_papers: Vec<String>,
    pub unlinked_code: Vec<String>,
    pub paper_coverage: HashMap<String, Vec<String>>,
}

pub struct TraceabilityEngine<'a> {
    vfs: &'a dyn VirtualFileSystem,
}

impl<'a> TraceabilityEngine<'a> {
    pub fn new(vfs: &'a dyn VirtualFileSystem) -> Self {
        Self { vfs }
    }

    /// Checks if a module name and paper name follow the naming parity rule.
    pub fn check_naming_parity(module_name: &str, paper_name: &str) -> bool {
        let expected = format!("{}.tex", module_name);
        paper_name == expected
    }

    /// Extract citations matching the regex `\[cite:([a-zA-Z0-9_.-]+)\]` manually to avoid heavy regex compilation.
    pub fn extract_citations(content: &str) -> Vec<String> {
        let mut cites = Vec::new();
        let mut search_idx = 0;
        while let Some(start) = content[search_idx..].find("[cite:") {
            let real_start = search_idx + start + 6;
            if let Some(end) = content[real_start..].find("]") {
                let paper_name = content[real_start..real_start + end].to_string();
                // basic regex validation: [a-zA-Z0-9_.-]+
                if paper_name
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.' || c == '-')
                    && !paper_name.is_empty()
                {
                    cites.push(paper_name);
                }
                search_idx = real_start + end + 1;
            } else {
                break;
            }
        }
        cites
    }

    pub fn scan_repository(
        &self,
        code_dirs: &[&str],
        papers_dir: &str,
    ) -> Result<TraceabilityReport, std::io::Error> {
        let mut valid_papers = HashSet::new();
        let mut report = TraceabilityReport::default();

        // 1. Scan papers
        if let Ok(entries) = self.vfs.list_dir(papers_dir) {
            for name in entries {
                if name.ends_with(".tex") {
                    let paper_name = name.trim_end_matches(".tex").to_string();
                    valid_papers.insert(paper_name.clone());
                    report.paper_coverage.insert(paper_name, Vec::new());
                }
            }
        }

        // 2. Scan code files
        let mut code_files = Vec::new();
        for dir in code_dirs {
            self.scan_dir(dir, &mut code_files);
        }

        for file in code_files {
            report.scanned_files += 1;
            let is_module =
                file.ends_with("mod.rs") || file.contains("/tabs/") || file.ends_with("lib.rs");

            if let Ok(content) = self.vfs.read_to_string(&file) {
                let cites = Self::extract_citations(&content);

                if cites.is_empty() && is_module {
                    report.unlinked_code.push(file.clone());
                }

                for paper_name in cites {
                    if valid_papers.contains(&paper_name) {
                        if let Some(linked) = report.paper_coverage.get_mut(&paper_name) {
                            linked.push(file.clone());
                        }
                    } else {
                        report.invalid_links.push((file.clone(), paper_name));
                    }
                }
            }
        }

        // 3. Find orphans
        for (name, linked_code) in &report.paper_coverage {
            if linked_code.is_empty() {
                report.orphaned_papers.push(name.clone());
            }
        }
        report.orphaned_papers.sort();
        report.unlinked_code.sort();
        report.invalid_links.sort();

        Ok(report)
    }

    fn scan_dir(&self, dir: &str, files: &mut Vec<String>) {
        if let Ok(entries) = self.vfs.list_dir(dir) {
            for entry in entries {
                let path = format!("{}/{}", dir, entry);
                if entry.contains('.') {
                    if path.ends_with(".rs") {
                        files.push(path.clone());
                    }
                } else {
                    self.scan_dir(&path, files);
                }
            }
        }
    }
}
