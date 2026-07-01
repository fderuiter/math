use crate::tabs::ExplorerTab;
use eframe::egui;
use oxidize_core::vfs::VirtualFileSystem;
use std::collections::HashMap;

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
        
        let vfs = oxidize_core::vfs::WasmVfs;

        let engine = oxidize_core::traceability::TraceabilityEngine::new(&vfs);

        // Scan the standard directories
        let code_dirs = vec!["math_explorer/src", "math_explorer_gui/src/tabs"];

        // We simulate reading the crates directories
        let mut crate_dirs = Vec::new();
        if let Ok(crates) = vfs.list_dir("crates") {
            for crate_name in crates {
                crate_dirs.push(format!("crates/{}/src", crate_name));
            }
        }

        let mut all_dirs: Vec<&str> = code_dirs.into_iter().collect();
        for dir in &crate_dirs {
            all_dirs.push(dir.as_str());
        }

        if let Ok(report) = engine.scan_repository(&all_dirs, "papers") {
            self.orphaned_papers = report.orphaned_papers;
            self.unlinked_code = report.unlinked_code;
            self.invalid_links = report.invalid_links;

            for (paper_name, linked_code) in report.paper_coverage {
                self.papers.insert(
                    paper_name.clone(),
                    PaperStatus {
                        name: paper_name,
                        linked_code,
                        is_wip: false,
                    },
                );
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

