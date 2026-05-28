use crate::accessibility::AccessibleHoverText;
use crate::tabs::clinical_trials::ClinicalTrialsTool;
use eframe::egui;
use math_explorer::applied::clinical_trials::design::{
    AllocationStrategy, BlockRandomizer, Group, SimpleRandomizer,
};
use rand::thread_rng;

#[derive(PartialEq)]
enum RandomizationType {
    Simple,
    Block,
}

pub struct RandomizationTool {
    randomization_type: RandomizationType,
    n_subjects: usize,
    block_size: usize,
    assignments: Vec<Group>,
    error_message: Option<String>,
}

impl Default for RandomizationTool {
    fn default() -> Self {
        Self {
            randomization_type: RandomizationType::Simple,
            n_subjects: 10,
            block_size: 4,
            assignments: Vec::new(),
            error_message: None,
        }
    }
}

impl ClinicalTrialsTool for RandomizationTool {
    fn name(&self) -> &'static str {
        "Randomization"
    }

    fn show(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Interactive Subject Allocation");

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Randomization Type:");
                ui.radio_value(
                    &mut self.randomization_type,
                    RandomizationType::Simple,
                    "Simple",
                );
                ui.radio_value(
                    &mut self.randomization_type,
                    RandomizationType::Block,
                    "Block",
                );
            });

            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label("Number of Subjects:");
                ui.add(egui::DragValue::new(&mut self.n_subjects).range(1..=1000));
            });

            if self.randomization_type == RandomizationType::Block {
                ui.horizontal(|ui| {
                    ui.label("Block Size:");
                    ui.add(egui::DragValue::new(&mut self.block_size).range(2..=100));
                });
            }

            ui.add_space(10.0);

            let enter_pressed = ui.input(|i| i.key_pressed(egui::Key::Enter));
            if ui
                .button("▶ Allocate Subjects")
                .accessible_hover_text("Randomly assign subjects to treatment and control groups")
                .clicked()
                || enter_pressed
            {
                self.error_message = None;
                self.assignments.clear();

                let mut rng = thread_rng();

                match self.randomization_type {
                    RandomizationType::Simple => {
                        let randomizer = SimpleRandomizer;
                        match randomizer.assign(&mut rng, self.n_subjects) {
                            Ok(assignments) => self.assignments = assignments,
                            Err(e) => self.error_message = Some(e.to_string()),
                        }
                    }
                    RandomizationType::Block => match BlockRandomizer::new(self.block_size) {
                        Ok(randomizer) => match randomizer.assign(&mut rng, self.n_subjects) {
                            Ok(assignments) => self.assignments = assignments,
                            Err(e) => self.error_message = Some(e.to_string()),
                        },
                        Err(e) => self.error_message = Some(e.to_string()),
                    },
                }
            }

            ui.add_space(10.0);

            if let Some(ref err) = self.error_message {
                ui.colored_label(egui::Color32::RED, format!("Error: {}", err));
            } else if self.assignments.is_empty() {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    ui.label(
                        egui::RichText::new("No subjects allocated yet.")
                            .strong()
                            .color(egui::Color32::DARK_GRAY),
                    );
                    ui.label(
                        egui::RichText::new(
                            "Configure parameters and click 'Allocate Subjects' to begin.",
                        )
                        .color(egui::Color32::GRAY),
                    );
                });
            } else {
                ui.heading("Assignments:");

                let mut treatment_count = 0;
                let mut control_count = 0;
                for group in &self.assignments {
                    match group {
                        Group::Treatment => treatment_count += 1,
                        Group::Control => control_count += 1,
                    }
                }

                ui.label(format!("Total Treatment: {}", treatment_count));
                ui.label(format!("Total Control: {}", control_count));

                egui::ScrollArea::vertical()
                    .max_height(300.0)
                    .show(ui, |ui| {
                        for (i, group) in self.assignments.iter().enumerate() {
                            let group_str = match group {
                                Group::Treatment => "Treatment",
                                Group::Control => "Control",
                            };
                            ui.label(format!("Subject {}: {}", i + 1, group_str));
                        }
                    });
            }
        });
    }
}

// [cite:clinical_trials_statistics]
