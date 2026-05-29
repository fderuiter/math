#[cfg(feature = "pure_math")]
#[cfg(feature = "ai")]
use crate::tabs::ai::AiTab;
#[cfg(feature = "pure_math")]
use crate::tabs::analysis::AnalysisTab;
#[cfg(feature = "applied")]
use crate::tabs::battery_degradation::BatteryDegradationTab;
#[cfg(feature = "physics")]
use crate::tabs::chaos::ChaosTab;
#[cfg(feature = "climate")]
use crate::tabs::climate::ClimateTab;
#[cfg(feature = "applied")]
use crate::tabs::clinical_trials::ClinicalTrialsTab;
#[cfg(feature = "epidemiology")]
use crate::tabs::epidemiology::EpidemiologyTab;
#[cfg(feature = "strict-opt-in-experimental")]
use crate::tabs::experimental_tab::ExperimentalTab;
#[cfg(feature = "applied")]
use crate::tabs::favoritism::FavoritismTab;
#[cfg(feature = "pure_math")]
use crate::tabs::financial_math::FinancialMathTab;
#[cfg(feature = "physics")]
use crate::tabs::fluid_dynamics::FluidDynamicsTab;
#[cfg(feature = "applied")]
use crate::tabs::game_theory::GameTheoryTab;
#[cfg(feature = "pure_math")]
use crate::tabs::graph_theory::GraphTheoryTab;
#[cfg(feature = "physics")]
use crate::tabs::medical::MedicalTab;
#[cfg(feature = "biology")]
use crate::tabs::morphogenesis::MorphogenesisTab;
#[cfg(feature = "physics")]
use crate::tabs::mri::MriTab;
#[cfg(feature = "biology")]
use crate::tabs::neuroscience::NeuroscienceTab;
#[cfg(feature = "pure_math")]
use crate::tabs::number_theory::NumberTheoryTab;
#[cfg(feature = "physics")]
use crate::tabs::quantum::QuantumTab;
#[cfg(feature = "physics")]
use crate::tabs::solid_state::SolidStateTab;
use crate::tabs::ExplorerTab;
#[cfg(feature = "pure_math")]
#[cfg(feature = "pure_math")]
use crate::tabs::GeometryTopologyTab;
use eframe::egui;
use math_explorer::diagnostics::{global_bus, DiagnosticEvent, Severity};

pub struct MathExplorerApp {
    tabs: Vec<Box<dyn ExplorerTab>>,
    selected_tab: usize,
    diagnostic_events: Vec<DiagnosticEvent>,
    show_info: bool,
    show_warnings: bool,
    show_errors: bool,
}

impl Default for MathExplorerApp {
    #[allow(clippy::vec_init_then_push)]
    fn default() -> Self {
        #[allow(unused_mut)]
        let mut tabs: Vec<Box<dyn ExplorerTab>> = vec![];

        #[cfg(feature = "physics")]
        tabs.push(Box::new(MriTab::default()));
        #[cfg(feature = "physics")]
        tabs.push(Box::new(QuantumTab::default()));
        #[cfg(feature = "physics")]
        tabs.push(Box::new(FluidDynamicsTab::default()));
        #[cfg(feature = "physics")]
        tabs.push(Box::new(ChaosTab::default()));
        #[cfg(feature = "physics")]
        tabs.push(Box::new(SolidStateTab::default()));
        #[cfg(feature = "physics")]
        tabs.push(Box::new(MedicalTab::default()));
        #[cfg(feature = "biology")]
        tabs.push(Box::new(NeuroscienceTab::default()));
        #[cfg(feature = "pure_math")]
        tabs.push(Box::new(NumberTheoryTab::default()));
        #[cfg(feature = "pure_math")]
        tabs.push(Box::new(GraphTheoryTab::default()));
        #[cfg(feature = "pure_math")]
        tabs.push(Box::new(GeometryTopologyTab::default()));
        #[cfg(feature = "pure_math")]
        tabs.push(Box::new(AnalysisTab::default()));
        #[cfg(feature = "climate")]
        tabs.push(Box::new(ClimateTab::default()));
        #[cfg(feature = "epidemiology")]
        tabs.push(Box::new(EpidemiologyTab::default()));
        #[cfg(feature = "applied")]
        tabs.push(Box::new(GameTheoryTab::default()));
        #[cfg(feature = "biology")]
        tabs.push(Box::new(MorphogenesisTab::default()));
        #[cfg(feature = "applied")]
        tabs.push(Box::new(ClinicalTrialsTab::default()));
        #[cfg(feature = "applied")]
        tabs.push(Box::new(BatteryDegradationTab::default()));
        #[cfg(feature = "ai")]
        tabs.push(Box::new(AiTab::default()));
        #[cfg(feature = "applied")]
        tabs.push(Box::new(FavoritismTab::default()));
        #[cfg(feature = "pure_math")]
        tabs.push(Box::new(FinancialMathTab::default()));

        tabs.push(Box::new(crate::tabs::TraceabilityTab::default()));

        #[cfg(feature = "strict-opt-in-experimental")]
        tabs.push(Box::new(ExperimentalTab::default()));

        Self {
            tabs,
            selected_tab: 0,
            diagnostic_events: Vec::new(),
            show_info: true,
            show_warnings: true,
            show_errors: true,
        }
    }
}

impl MathExplorerApp {
    #[allow(clippy::vec_init_then_push)]
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MathExplorerApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Fetch new events
        self.diagnostic_events.extend(global_bus().try_recv_all());

        // Issues & Diagnostics Panel
        egui::TopBottomPanel::bottom("issues_panel")
            .resizable(true)
            .min_height(100.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("Issues & Diagnostics");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Clear").clicked() {
                            self.diagnostic_events.clear();
                        }
                        ui.checkbox(&mut self.show_errors, "Errors/Fatal");
                        ui.checkbox(&mut self.show_warnings, "Warnings");
                        ui.checkbox(&mut self.show_info, "Info");
                    });
                });
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for event in &self.diagnostic_events {
                        let show = match event.severity {
                            Severity::Info => self.show_info,
                            Severity::Warning => self.show_warnings,
                            Severity::Error | Severity::Fatal => self.show_errors,
                        };
                        if !show {
                            continue;
                        }

                        let color = match event.severity {
                            Severity::Info => egui::Color32::LIGHT_BLUE,
                            Severity::Warning => egui::Color32::YELLOW,
                            Severity::Error => egui::Color32::RED,
                            Severity::Fatal => egui::Color32::DARK_RED,
                        };

                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(format!("[{}]", event.severity)).color(color).strong());
                                if let Some(thread) = &event.thread_name {
                                    ui.label(egui::RichText::new(format!("(Thread: {})", thread)).italics());
                                }
                                ui.label(&event.message);
                            });
                            if !event.metadata.is_empty() {
                                ui.horizontal_wrapped(|ui| {
                                    for (k, v) in &event.metadata {
                                        ui.label(egui::RichText::new(format!("{}: {}", k, v)).monospace().size(10.0));
                                    }
                                });
                            }
                        });
                    }
                });
            });

        // Render Tab Bar
        egui::TopBottomPanel::top("main_menu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Math Explorer");
                ui.separator();
                egui::ScrollArea::horizontal().show(ui, |ui| {
                    for (i, tab) in self.tabs.iter().enumerate() {
                        let name = tab.name();
                        if ui.selectable_label(self.selected_tab == i, name).clicked() {
                            self.selected_tab = i;
                        }
                    }
                });
            });
        });

        // Render Active Tab
        if let Some(tab) = self.tabs.get_mut(self.selected_tab) {
            tab.show(ctx, frame);
        } else {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.label("No module selected.");
                });
            });
        }
    }
}
