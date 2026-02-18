use crate::tabs::{fluid_dynamics::FluidDynamicsTab, mri::MriTab, quantum::QuantumTab, ExplorerTab};
use eframe::egui;

pub struct MathExplorerApp {
    tabs: Vec<Box<dyn ExplorerTab>>,
    selected_tab: usize,
}

impl Default for MathExplorerApp {
    fn default() -> Self {
        Self {
            tabs: vec![
                Box::new(MriTab::default()),
                Box::new(QuantumTab::default()),
                Box::new(FluidDynamicsTab::default()),
            ],
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
                for (i, tab) in self.tabs.iter().enumerate() {
                    let name = tab.name();
                    if ui.selectable_label(self.selected_tab == i, name).clicked() {
                        self.selected_tab = i;
                    }
                }
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
