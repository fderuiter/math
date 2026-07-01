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

    pub fn verify_module_registered(module_name: &str) -> bool {
        let registry_content = include_str!("../../../traceability.toml");
        if let Ok(value) = registry_content.parse::<toml::Table>()
            && let Some(links) = value.get("links").and_then(|v| v.as_table())
            && let Some(paper) = links.get(module_name)
        {
            return paper.as_str().is_some();
        }
        false
    }

    fn verify_and_link_registry(
        &self,
        valid_papers: &HashSet<String>,
        report: &mut TraceabilityReport,
    ) {
        if let Ok(registry_content) = self.vfs.read_to_string("traceability.toml")
            && let Ok(value) = registry_content.parse::<toml::Table>()
            && let Some(links) = value.get("links").and_then(|v| v.as_table())
        {
            for (module_name, paper_val) in links {
                if let Some(paper_name) = paper_val.as_str() {
                    let paper_name_string = paper_name.to_string();
                    if valid_papers.contains(&paper_name_string) {
                        if let Some(linked) = report.paper_coverage.get_mut(&paper_name_string) {
                            linked.push(module_name.clone());
                        }
                    } else {
                        report
                            .invalid_links
                            .push((module_name.clone(), paper_name_string));
                    }
                } else {
                    report
                        .invalid_links
                        .push((module_name.clone(), "INVALID_FORMAT".to_string()));
                }
            }
        }
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
                    let paper_name = name.clone();
                    valid_papers.insert(paper_name.clone());
                    report.paper_coverage.insert(paper_name, Vec::new());
                }
            }
        }

        // 2. Read registry
        self.verify_and_link_registry(&valid_papers, &mut report);

        // 3. Scan code files for unlinked code
        let mut code_files = Vec::new();
        for dir in code_dirs {
            self.scan_dir(dir, &mut code_files);
        }

        let mut registered_modules = HashSet::new();
        if let Ok(registry_content) = self.vfs.read_to_string("traceability.toml")
            && let Ok(value) = registry_content.parse::<toml::Table>()
            && let Some(links) = value.get("links").and_then(|v| v.as_table())
        {
            for module_name in links.keys() {
                registered_modules.insert(module_name.clone());
            }
        }

        for file in code_files {
            report.scanned_files += 1;
            let is_module =
                file.ends_with("mod.rs") || file.contains("/tabs/") || file.ends_with("lib.rs");

            if is_module
                && let Ok(content) = self.vfs.read_to_string(&file)
                && (content.contains("theory_verification!") || content.contains("stochastic_signature_verification!"))
            {
                // Very basic check to see if the module name is in the file
                // The actual macro test will panic if it's missing from registry
                let mut found = false;
                for reg_mod in &registered_modules {
                    if content.contains(&format!("module = \"{}\"", reg_mod)) {
                        found = true;
                        break;
                    }
                }
                if !found {
                    report.unlinked_code.push(file.clone());
                }
            }
        }

        // 4. Find orphans
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
