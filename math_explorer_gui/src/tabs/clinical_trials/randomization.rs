use super::ClinicalTrialsTool;
use eframe::egui;
use math_explorer::applied::clinical_trials::design::{
    AllocationStrategy, BlockRandomizer, Group, SimpleRandomizer,
};

#[derive(Debug, PartialEq, Clone, Copy)]
enum RandomizationMode {
    Simple,
    Block,
}

pub struct RandomizationTool {
    mode: RandomizationMode,
    n_subjects: usize,
    block_size: usize,
    assignments: Option<Result<Vec<Group>, String>>,
}

impl Default for RandomizationTool {
    fn default() -> Self {
        Self {
            mode: RandomizationMode::Simple,
            n_subjects: 20,
            block_size: 4,
            assignments: None,
        }
    }
}

impl RandomizationTool {
    fn randomize(&mut self) {
        let mut rng = rand::thread_rng();

        self.assignments = match self.mode {
            RandomizationMode::Simple => {
                let strategy = SimpleRandomizer;
                Some(
                    strategy
                        .assign(&mut rng, self.n_subjects)
                        .map_err(|e| e.to_string()),
                )
            }
            RandomizationMode::Block => match BlockRandomizer::new(self.block_size) {
                Ok(strategy) => Some(
                    strategy
                        .assign(&mut rng, self.n_subjects)
                        .map_err(|e| e.to_string()),
                ),
                Err(e) => Some(Err(e.to_string())),
            },
        };
    }
}

impl ClinicalTrialsTool for RandomizationTool {
    fn name(&self) -> &'static str {
        "Randomization"
    }

    fn show(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Subject Randomization");
            ui.label("Allocate subjects to Treatment or Control groups.");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Strategy:");
                if ui
                    .radio_value(&mut self.mode, RandomizationMode::Simple, "Simple")
                    .changed()
                {
                    self.assignments = None;
                }
                if ui
                    .radio_value(&mut self.mode, RandomizationMode::Block, "Block")
                    .changed()
                {
                    self.assignments = None;
                }
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Number of Subjects:");
                if ui
                    .add(egui::Slider::new(&mut self.n_subjects, 2..=1000).text("Subjects"))
                    .changed()
                {
                    self.assignments = None;
                }
            });

            if self.mode == RandomizationMode::Block {
                ui.horizontal(|ui| {
                    ui.label("Block Size:");
                    if ui
                        .add(
                            egui::Slider::new(&mut self.block_size, 2..=20)
                                .step_by(2.0)
                                .text("Size"),
                        )
                        .changed()
                    {
                        self.assignments = None;
                    }
                    ui.small("(Must be even)");
                });
            }

            ui.add_space(10.0);

            if ui.button("Randomize!").clicked() {
                self.randomize();
            }

            ui.separator();
            ui.add_space(10.0);

            // Display Results
            if let Some(result) = &self.assignments {
                match result {
                    Ok(assignments) => {
                        let mut treatment_count = 0;
                        let mut control_count = 0;

                        for &g in assignments {
                            if g == Group::Treatment {
                                treatment_count += 1;
                            } else {
                                control_count += 1;
                            }
                        }

                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Summary:").strong());
                            ui.label(format!(
                                "{} Treatment, {} Control",
                                treatment_count, control_count
                            ));
                        });

                        ui.add_space(10.0);

                        egui::ScrollArea::vertical().show(ui, |ui| {
                            egui::Grid::new("assignments_grid")
                                .num_columns(2)
                                .spacing([40.0, 4.0])
                                .striped(true)
                                .show(ui, |ui| {
                                    ui.label(egui::RichText::new("Subject ID").strong());
                                    ui.label(egui::RichText::new("Assigned Group").strong());
                                    ui.end_row();

                                    for (i, &g) in assignments.iter().enumerate() {
                                        ui.label(format!("Subject {:03}", i + 1));

                                        let (text, color) = if g == Group::Treatment {
                                            ("Treatment", egui::Color32::from_rgb(100, 150, 250))
                                        } else {
                                            ("Control", egui::Color32::from_rgb(200, 100, 100))
                                        };

                                        ui.colored_label(color, text);
                                        ui.end_row();
                                    }
                                });
                        });
                    }
                    Err(e) => {
                        ui.colored_label(egui::Color32::RED, format!("Error: {}", e));
                    }
                }
            } else {
                ui.label("Click 'Randomize!' to generate assignments.");
            }
        });
    }
}
