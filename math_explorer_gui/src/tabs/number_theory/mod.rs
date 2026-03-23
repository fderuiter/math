use crate::tabs::ExplorerTab;
use eframe::egui;

pub mod factorization;
pub mod prime_spiral;
pub mod ualbf_widget;

/// A trait for sub-tools within the Number Theory tab.
pub trait NumberTheoryTool {
    /// Returns the name of the tool.
    fn name(&self) -> &'static str;

    /// Renders the tool's UI.
    fn show(&mut self, ctx: &egui::Context);
}

pub struct NumberTheoryTab {
    tools: Vec<Box<dyn NumberTheoryTool>>,
    selected_tool_index: usize,
}

impl Default for NumberTheoryTab {
    fn default() -> Self {
        Self {
            tools: vec![
                Box::new(prime_spiral::PrimeSpiralWidget::default()),
                Box::new(ualbf_widget::UalbfWidget::default()),
                Box::new(factorization::FactorizationTool::default()),
            ],
            selected_tool_index: 0,
        }
    }
}

impl ExplorerTab for NumberTheoryTab {
    fn name(&self) -> &'static str {
        "Number Theory"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Render Top Menu for Tool Selection
        egui::TopBottomPanel::top("number_theory_tool_selector").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Tool:");
                for (i, tool) in self.tools.iter().enumerate() {
                    if ui
                        .selectable_label(self.selected_tool_index == i, tool.name())
                        .clicked()
                    {
                        self.selected_tool_index = i;
                    }
                }
            });
        });

        // Delegate to active tool
        if let Some(tool) = self.tools.get_mut(self.selected_tool_index) {
            tool.show(ctx);
        } else {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.label("No tool selected");
                });
            });
        }
    }
}
