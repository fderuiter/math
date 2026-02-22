#![allow(deprecated)]
use crate::tabs::clinical_trials::ClinicalTrialTool;
use eframe::egui;
use math_explorer::applied::clinical_trials::design::{Group, simple_randomization, block_randomization};

#[derive(PartialEq)]
enum RandomizationStrategyType {
    Simple,
    Block,
}

pub struct RandomizationTool {
    n_subjects: usize,
    block_size: usize,
    strategy: RandomizationStrategyType,
    assignments: Vec<Group>,
    error_msg: Option<String>,
}

impl Default for RandomizationTool {
    fn default() -> Self {
        Self {
            n_subjects: 20,
            block_size: 4,
            strategy: RandomizationStrategyType::Simple,
            assignments: Vec::new(),
            error_msg: None,
        }
    }
}

impl ClinicalTrialTool for RandomizationTool {
    fn name(&self) -> &'static str {
        "Randomization"
    }

    fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("Allocation Strategy");

        ui.radio_value(&mut self.strategy, RandomizationStrategyType::Simple, "Simple Randomization");
        ui.radio_value(&mut self.strategy, RandomizationStrategyType::Block, "Block Randomization");

        ui.add(egui::Slider::new(&mut self.n_subjects, 1..=1000).text("Number of Subjects"));

        if self.strategy == RandomizationStrategyType::Block {
            ui.add(egui::Slider::new(&mut self.block_size, 2..=20).step_by(2.0).text("Block Size (Must be even)"));
        }

        if ui.button("Randomize").clicked() {
            self.randomize();
        }

        if let Some(err) = &self.error_msg {
            ui.colored_label(egui::Color32::RED, err);
        }

        ui.separator();

        if !self.assignments.is_empty() {
            ui.heading("Assignments");

            let treatment_count = self.assignments.iter().filter(|&&g| g == Group::Treatment).count();
            let control_count = self.assignments.iter().filter(|&&g| g == Group::Control).count();

            ui.label(format!("Treatment: {} | Control: {}", treatment_count, control_count));

            egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                for (i, group) in self.assignments.iter().enumerate() {
                    let text = format!("Subject {}: {:?}", i + 1, group);
                    let color = if *group == Group::Treatment {
                        egui::Color32::from_rgb(100, 200, 255)
                    } else {
                        egui::Color32::from_rgb(255, 100, 100)
                    };
                    ui.label(egui::RichText::new(text).color(color));
                }
            });
        }
    }
}

impl RandomizationTool {
    fn randomize(&mut self) {
        self.error_msg = None;
        self.assignments.clear();

        #[allow(deprecated)]
        let result = match self.strategy {
            RandomizationStrategyType::Simple => {
                // simple_randomization returns Vec<Group> directly
                Ok(simple_randomization(self.n_subjects))
            }
            RandomizationStrategyType::Block => {
                block_randomization(self.n_subjects, self.block_size)
            }
        };

        match result {
            Ok(assignments) => {
                self.assignments = assignments;
            }
            Err(e) => {
                self.error_msg = Some(e);
            }
        }
    }
}
