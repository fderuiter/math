use crate::tabs::ExplorerTab;
use eframe::egui;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;

pub struct TraceabilityTab {
    papers: HashMap<String, PaperStatus>,
    orphaned_papers: Vec<String>,
    unlinked_code: Vec<String>,
    invalid_links: Vec<(String, String)>, // code_file, paper_name
    executed_runs: HashMap<String, usize>, // paper_name -> count
}

#[derive(Clone)]
struct PaperStatus {
    name: String,
    linked_code: Vec<String>,
    is_wip: bool,
    executions: usize,
}

impl Default for TraceabilityTab {
    fn default() -> Self {
        let mut tab = Self {
            papers: HashMap::new(),
            orphaned_papers: Vec::new(),
            unlinked_code: Vec::new(),
            invalid_links: Vec::new(),
            executed_runs: HashMap::new(),
        };
        tab.refresh();
        tab
    }
}

impl TraceabilityTab {
    fn refresh(&mut self) {
        self.papers.clear();
        self.orphaned_papers.clear();
        self.unlinked_code.clear();
        self.invalid_links.clear();
        self.executed_runs.clear();

        // 1. Scan papers
        let papers_dir = PathBuf::from("papers");
        let mut valid_papers = HashSet::new();
        if let Ok(entries) = fs::read_dir(&papers_dir) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".tex") {
                        let paper_name = name.trim_end_matches(".tex").to_string();
                        valid_papers.insert(paper_name.clone());
                        self.papers.insert(
                            paper_name.clone(),
                            PaperStatus {
                                name: paper_name,
                                linked_code: Vec::new(),
                                is_wip: false,
                                executions: 0,
                            },
                        );
                    }
                }
            }
        }

        // 2. Scan code files
        let mut code_files = Vec::new();
        self.scan_dir(PathBuf::from("math_explorer/src"), &mut code_files);
        self.scan_dir(PathBuf::from("math_explorer_gui/src/tabs"), &mut code_files);

        for file in code_files {
            let is_module =
                file.ends_with("mod.rs") || file.contains("math_explorer_gui/src/tabs/");

            if !is_module {
                continue;
            }

            if let Ok(content) = fs::read_to_string(&file) {
                let mut found_cite = false;

                let mut search_idx = 0;
                while let Some(start) = content[search_idx..].find("[cite:") {
                    found_cite = true;
                    let real_start = search_idx + start + 6;
                    if let Some(end) = content[real_start..].find("]") {
                        let paper_name = content[real_start..real_start + end].to_string();
                        if valid_papers.contains(&paper_name) {
                            if let Some(status) = self.papers.get_mut(&paper_name) {
                                status.linked_code.push(file.clone());
                            }
                        } else {
                            self.invalid_links.push((file.clone(), paper_name));
                        }
                        search_idx = real_start + end + 1;
                    } else {
                        break;
                    }
                }

                if !found_cite {
                    self.unlinked_code.push(file.clone());
                }
            }
        }

        // 3. Scan Audit Logs
        let audit_dir = PathBuf::from("audit_logs");
        if let Ok(entries) = fs::read_dir(&audit_dir) {
            for entry in entries.flatten() {
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    for line in content.lines() {
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
                            if let Some(link) = json.get("theory_link").and_then(|v| v.as_str()) {
                                *self.executed_runs.entry(link.to_string()).or_insert(0) += 1;
                            }
                        }
                    }
                }
            }
        }
        
        for (link, count) in &self.executed_runs {
            if let Some(status) = self.papers.get_mut(link) {
                status.executions += count;
            }
        }

        // 4. Find orphans
        for (name, status) in &self.papers {
            if status.linked_code.is_empty() {
                self.orphaned_papers.push(name.clone());
            }
        }
        self.orphaned_papers.sort();
        self.unlinked_code.sort();
    }

    fn scan_dir(&self, dir: PathBuf, files: &mut Vec<String>) {
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    self.scan_dir(path, files);
                } else if path.extension().map_or(false, |e| e == "rs") {
                    files.push(path.to_string_lossy().into_owned());
                }
            }
        }
    }
}

impl ExplorerTab for TraceabilityTab {
    fn name(&self) -> &'static str {
        "Traceability Portal"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Theory-to-Code Traceability Portal");
                if ui.button("Refresh").clicked() {
                    self.refresh();
                }
            });
            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.collapsing("System Health & Alignment", |ui| {
                    if self.orphaned_papers.is_empty()
                        && self.unlinked_code.is_empty()
                        && self.invalid_links.is_empty()
                    {
                        ui.label(
                            egui::RichText::new("100% Parity Achieved").color(egui::Color32::GREEN),
                        );
                    } else {
                        ui.label(
                            egui::RichText::new("Partial Alignment").color(egui::Color32::YELLOW),
                        );
                    }
                    ui.label(format!("Total Papers: {}", self.papers.len()));
                });

                if !self.orphaned_papers.is_empty() {
                    ui.add_space(10.0);
                    ui.heading(
                        egui::RichText::new("Orphaned Papers (No Code)").color(egui::Color32::RED),
                    );
                    for p in &self.orphaned_papers {
                        ui.label(p);
                    }
                }

                if !self.unlinked_code.is_empty() {
                    ui.add_space(10.0);
                    ui.heading(
                        egui::RichText::new("Unlinked Code Modules (No Paper)")
                            .color(egui::Color32::RED),
                    );
                    for c in &self.unlinked_code {
                        ui.label(c);
                    }
                }

                if !self.invalid_links.is_empty() {
                    ui.add_space(10.0);
                    ui.heading(
                        egui::RichText::new("Naming Mismatches / Invalid Cites")
                            .color(egui::Color32::RED),
                    );
                    for (c, p) in &self.invalid_links {
                        ui.label(format!("{} cites invalid paper: {}", c, p));
                    }
                }

                ui.add_space(10.0);
                ui.heading("Traceability Matrix");

                let mut papers: Vec<_> = self.papers.values().collect();
                papers.sort_by(|a, b| a.name.cmp(&b.name));

                for p in papers {
                    ui.collapsing(format!("{} (Executions: {})", p.name, p.executions), |ui| {
                        if p.linked_code.is_empty() {
                            ui.label(
                                egui::RichText::new("No implementation found")
                                    .color(egui::Color32::RED),
                            );
                        } else {
                            for code in &p.linked_code {
                                ui.label(code);
                            }
                        }
                    });
                }
            });
        });
    }
}
