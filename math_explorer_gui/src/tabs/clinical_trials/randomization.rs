use super::ClinicalTrialsTool;
use eframe::egui;
use math_explorer::applied::clinical_trials::design::{
    AllocationStrategy, BlockRandomizer, Group, SimpleRandomizer,
};

#[derive(Debug, PartialEq, Clone, Copy)]
enum RandomizationStrategy {
    Simple,
    Block,
}

pub struct RandomizationTool {
    n_subjects: usize,
    strategy: RandomizationStrategy,
    block_size: usize,
    assignments: Option<Result<Vec<Group>, String>>,
}

impl Default for RandomizationTool {
    fn default() -> Self {
        Self {
            n_subjects: 20,
            strategy: RandomizationStrategy::Block,
            block_size: 4,
            assignments: None,
        }
    }
}

impl RandomizationTool {
    fn trigger_randomization(&mut self) {
        let mut rng = rand::thread_rng();

        let result = match self.strategy {
            RandomizationStrategy::Simple => {
                let randomizer = SimpleRandomizer;
                randomizer
                    .assign(&mut rng, self.n_subjects)
                    .map_err(|e| e.to_string())
            }
            RandomizationStrategy::Block => match BlockRandomizer::new(self.block_size) {
                Ok(randomizer) => randomizer
                    .assign(&mut rng, self.n_subjects)
                    .map_err(|e| e.to_string()),
                Err(e) => Err(e.to_string()),
            },
        };

        self.assignments = Some(result);
    }
}

impl ClinicalTrialsTool for RandomizationTool {
    fn name(&self) -> &'static str {
        "Randomization"
    }

    fn show(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Randomization Tool");
            ui.label("Interactive subject allocation for clinical trials.");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Strategy:");
                ui.radio_value(&mut self.strategy, RandomizationStrategy::Simple, "Simple");
                ui.radio_value(&mut self.strategy, RandomizationStrategy::Block, "Block");
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Number of Subjects:");
                ui.add(egui::Slider::new(&mut self.n_subjects, 1..=1000).text("Subjects"));
            });

            if self.strategy == RandomizationStrategy::Block {
                ui.horizontal(|ui| {
                    ui.label("Block Size:");
                    ui.add(
                        egui::Slider::new(&mut self.block_size, 2..=20)
                            .step_by(2.0)
                            .text("Size"),
                    );
                });
            }

            ui.add_space(10.0);

            if ui.button("Randomize").clicked() {
                self.trigger_randomization();
            }

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(10.0);

            if let Some(ref result) = self.assignments {
                match result {
                    Ok(assignments) => {
                        let treatment_count = assignments
                            .iter()
                            .filter(|&&g| g == Group::Treatment)
                            .count();
                        let control_count = assignments.len() - treatment_count;

                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(format!("Treatment: {}", treatment_count))
                                    .color(egui::Color32::GREEN),
                            );
                            ui.label(
                                egui::RichText::new(format!("Control: {}", control_count))
                                    .color(egui::Color32::BLUE),
                            );
                        });

                        ui.add_space(10.0);

                        egui::ScrollArea::vertical().show(ui, |ui| {
                            egui::Grid::new("assignments_grid")
                                .striped(true)
                                .show(ui, |ui| {
                                    ui.label(egui::RichText::new("Subject ID").strong());
                                    ui.label(egui::RichText::new("Group").strong());
                                    ui.end_row();

                                    for (i, group) in assignments.iter().enumerate() {
                                        ui.label(format!("Subject {}", i + 1));
                                        match group {
                                            Group::Treatment => {
                                                ui.colored_label(egui::Color32::GREEN, "Treatment")
                                            }
                                            Group::Control => {
                                                ui.colored_label(egui::Color32::BLUE, "Control")
                                            }
                                        };
                                        ui.end_row();
                                    }
                                });
                        });
                    }
                    Err(e) => {
                        ui.colored_label(egui::Color32::RED, format!("Error: {}", e));
                    }
                }
            }
        });
    }
}
