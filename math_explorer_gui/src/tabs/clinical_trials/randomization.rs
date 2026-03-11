use super::ClinicalTrialsTool;
use eframe::egui;
use math_explorer::applied::clinical_trials::design::{AllocationStrategy, BlockRandomizer, Group, SimpleRandomizer};

#[derive(PartialEq)]
enum RandomizationMethod {
    Simple,
    Block,
}

pub struct RandomizationTool {
    method: RandomizationMethod,
    n_subjects: usize,
    block_size: usize,
    assignments: Vec<Group>,
    error_message: Option<String>,
}

impl Default for RandomizationTool {
    fn default() -> Self {
        Self {
            method: RandomizationMethod::Simple,
            n_subjects: 20,
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
            ui.vertical(|ui| {
                ui.heading("Subject Randomization");
                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Randomization Method:");
                    ui.radio_value(&mut self.method, RandomizationMethod::Simple, "Simple");
                    ui.radio_value(&mut self.method, RandomizationMethod::Block, "Block");
                });

                ui.add(egui::Slider::new(&mut self.n_subjects, 1..=500).text("Total Subjects"));

                if self.method == RandomizationMethod::Block {
                    ui.add(egui::Slider::new(&mut self.block_size, 2..=20).step_by(2.0).text("Block Size"));
                }

                if ui.button("Generate Assignments").clicked() {
                    self.error_message = None;
                    self.assignments.clear();

                    let mut rng = rand::thread_rng();

                    match self.method {
                        RandomizationMethod::Simple => {
                            let randomizer = SimpleRandomizer;
                            match randomizer.assign(&mut rng, self.n_subjects) {
                                Ok(assignments) => self.assignments = assignments,
                                Err(e) => self.error_message = Some(e.to_string()),
                            }
                        }
                        RandomizationMethod::Block => {
                            match BlockRandomizer::new(self.block_size) {
                                Ok(randomizer) => {
                                    match randomizer.assign(&mut rng, self.n_subjects) {
                                        Ok(assignments) => self.assignments = assignments,
                                        Err(e) => self.error_message = Some(e.to_string()),
                                    }
                                }
                                Err(e) => self.error_message = Some(e.to_string()),
                            }
                        }
                    }
                }

                if let Some(err) = &self.error_message {
                    ui.colored_label(egui::Color32::RED, format!("Error: {}", err));
                }

                ui.separator();

                if !self.assignments.is_empty() {
                    ui.heading("Assignments");

                    // Count treatments vs controls
                    let treatment_count = self.assignments.iter().filter(|&&g| g == Group::Treatment).count();
                    let control_count = self.assignments.iter().filter(|&&g| g == Group::Control).count();

                    ui.label(format!("Treatment Group: {} subjects", treatment_count));
                    ui.label(format!("Control Group: {} subjects", control_count));

                    ui.separator();

                    egui::ScrollArea::vertical().show(ui, |ui| {
                        egui::Grid::new("assignments_grid")
                            .striped(true)
                            .show(ui, |ui| {
                                ui.label(egui::RichText::new("Subject ID").strong());
                                ui.label(egui::RichText::new("Assigned Group").strong());
                                ui.end_row();

                                for (i, group) in self.assignments.iter().enumerate() {
                                    ui.label(format!("{:03}", i + 1));

                                    let group_text = match group {
                                        Group::Treatment => egui::RichText::new("Treatment").color(egui::Color32::LIGHT_BLUE),
                                        Group::Control => egui::RichText::new("Control").color(egui::Color32::LIGHT_GREEN),
                                    };
                                    ui.label(group_text);
                                    ui.end_row();
                                }
                            });
                    });
                }
            });
        });
    }
}
