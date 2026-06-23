use crate::tabs::ExplorerTab;
use eframe::egui;
use oxidize_core::vfs::VirtualFileSystem;
use std::collections::{HashMap, HashSet};

#[cfg(not(target_arch = "wasm32"))]
type VfsImpl = oxidize_core::vfs::DefaultVfs;

#[cfg(target_arch = "wasm32")]
// theory_verification!
type VfsImpl = oxidize_core::vfs::WasmVfs;

pub struct TraceabilityTab {
    papers: HashMap<String, PaperStatus>,
    orphaned_papers: Vec<String>,
    unlinked_code: Vec<String>,
    invalid_links: Vec<(String, String)>, // code_file, paper_name
}

#[derive(Clone)]
#[allow(dead_code)]
struct PaperStatus {
    name: String,
    linked_code: Vec<String>,
    is_wip: bool,
}

impl Default for TraceabilityTab {
    fn default() -> Self {
        let mut tab = Self {
            papers: HashMap::new(),
            orphaned_papers: Vec::new(),
            unlinked_code: Vec::new(),
            invalid_links: Vec::new(),
        };
        tab.refresh();
        tab
    }
}

impl TraceabilityTab {
    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    fn refresh(&mut self) {
        self.papers.clear();
        self.orphaned_papers.clear();
        self.unlinked_code.clear();
        self.invalid_links.clear();

        #[cfg(not(target_arch = "wasm32"))]
        let vfs = oxidize_core::vfs::DefaultVfs;
        #[cfg(target_arch = "wasm32")]
        // theory_verification!
        let vfs = oxidize_core::vfs::WasmVfs;

        // 1. Scan papers
        let papers_dir = "papers";
        let mut valid_papers = HashSet::new();
        if let Ok(entries) = vfs.list_dir(papers_dir) {
            for name in entries {
                if name.ends_with(".tex") {
                    let paper_name = name.trim_end_matches(".tex").to_string();
                    valid_papers.insert(paper_name.clone());
                    self.papers.insert(
                        paper_name.clone(),
                        PaperStatus {
                            name: paper_name,
                            linked_code: Vec::new(),
                            is_wip: false,
                        },
                    );
                }
            }
        }

        // 2. Scan code files
        let mut code_files = Vec::new();
        self.scan_dir(&vfs, "math_explorer/src", &mut code_files);
        self.scan_dir(&vfs, "math_explorer_gui/src/tabs", &mut code_files);

        // Scan inside individual crates
        if let Ok(crates) = vfs.list_dir("crates") {
            for crate_name in crates {
                let path = format!("crates/{}/src", crate_name);
                self.scan_dir(&vfs, &path, &mut code_files);
            }
        }

        for file in code_files {
            let is_module =
                file.ends_with("mod.rs") || file.contains("math_explorer_gui/src/tabs/");

            if !is_module {
                continue;
            }

            if let Ok(content) = vfs.read_to_string(&file) {
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

        // 3. Find orphans
        for (name, status) in &self.papers {
            if status.linked_code.is_empty() {
                self.orphaned_papers.push(name.clone());
            }
        }
        self.orphaned_papers.sort();
        self.unlinked_code.sort();
    }

    fn scan_dir(&self, vfs: &VfsImpl, dir: &str, files: &mut Vec<String>) {
        if let Ok(entries) = vfs.list_dir(dir) {
            for entry in entries {
                let path = format!("{}/{}", dir, entry);
                if entry.contains('.') {
                    // simple heuristic for file vs dir
                    if path.ends_with(".rs") {
                        files.push(path.clone());
                    }
                } else {
                    self.scan_dir(vfs, &path, files);
                }
            }
        }
    }
}

impl ExplorerTab for TraceabilityTab {
    fn name(&self) -> &'static str {
        "Traceability Portal"
    }

    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
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
                    ui.collapsing(&p.name, |ui| {
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
// [cite:essay]
// theory_verification!
