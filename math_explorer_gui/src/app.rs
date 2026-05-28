#[cfg(feature = "domain_logic")]
use crate::tabs::{
    biology::BiologyTab,
    ai::AiTab, analysis::AnalysisTab, battery_degradation::BatteryDegradationTab, chaos::ChaosTab,
    climate::ClimateTab, clinical_trials::ClinicalTrialsTab, epidemiology::EpidemiologyTab,
    favoritism::FavoritismTab, financial_math::FinancialMathTab, fluid_dynamics::FluidDynamicsTab,
    game_theory::GameTheoryTab, graph_theory::GraphTheoryTab, medical::MedicalTab,
    mri::MriTab,     number_theory::NumberTheoryTab, quantum::QuantumTab, solid_state::SolidStateTab,
    GeometryTopologyTab,
};
use crate::tabs::ExplorerTab;
use eframe::egui;

pub struct MathExplorerApp {
    tabs: Vec<Box<dyn ExplorerTab>>,
    selected_tab: usize,
}

impl Default for MathExplorerApp {
    fn default() -> Self {
        #[allow(unused_mut)]
        let mut tabs: Vec<Box<dyn ExplorerTab>> = vec![];
        
        #[cfg(feature = "domain_logic")]
        {
            tabs.push(Box::new(MriTab::default()));
            tabs.push(Box::new(QuantumTab::default()));
            tabs.push(Box::new(FluidDynamicsTab::default()));
            tabs.push(Box::new(ChaosTab::default()));
            tabs.push(Box::new(SolidStateTab::default()));
            tabs.push(Box::new(MedicalTab::default()));

        #[cfg(feature = "biology")]
        tabs.push(Box::new(BiologyTab::default()));

                        tabs.push(Box::new(NumberTheoryTab::default()));
            tabs.push(Box::new(GraphTheoryTab::default()));
            tabs.push(Box::new(GeometryTopologyTab::default()));
            tabs.push(Box::new(AnalysisTab::default()));
            tabs.push(Box::new(ClimateTab::default()));
            tabs.push(Box::new(EpidemiologyTab::default()));
            tabs.push(Box::new(GameTheoryTab::default()));
                        tabs.push(Box::new(ClinicalTrialsTab::default()));
            tabs.push(Box::new(BatteryDegradationTab::default()));
            tabs.push(Box::new(AiTab::default()));
            tabs.push(Box::new(FavoritismTab::default()));
            tabs.push(Box::new(FinancialMathTab::default()));
        }

        Self {
            tabs,
            selected_tab: 0,
        }
    }
}

impl MathExplorerApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MathExplorerApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
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
