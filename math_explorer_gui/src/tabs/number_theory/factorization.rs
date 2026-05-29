use super::NumberTheoryTool;
use crate::accessibility::AccessibleHoverText;
use eframe::egui;
use pure_math::number_theory::ambs::prime_factors;
use pure_math::number_theory::primes::is_prime;

type FactorizationResult = Result<(u64, bool, Vec<(u64, u32)>), String>;

#[derive(Default)]
pub struct FactorizationTool {
    input_text: String,
    results: Option<FactorizationResult>,
}

impl NumberTheoryTool for FactorizationTool {
    fn name(&self) -> &'static str {
        "Factorization Tool"
    }

    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    fn show(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Large Number Factorization and Primality Testing");
            ui.label("Enter an integer to test for primality and find its prime factors.");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Number:");
                let response =
                    ui.add(egui::TextEdit::singleline(&mut self.input_text).hint_text("e.g. 42"));
                let enter_pressed =
                    response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));

                if ui
                    .button("▶ Analyze")
                    .accessible_hover_text(
                        "Analyze the number for primality and calculate its prime factors",
                    )
                    .clicked()
                    || enter_pressed
                {
                    self.results = Some(match self.input_text.trim().parse::<u64>() {
                        Ok(num) => {
                            if num < 2 {
                                Err("Number must be at least 2.".to_string())
                            } else {
                                let prime_check = is_prime(num);
                                let factors = prime_factors(num);
                                Ok((num, prime_check, factors))
                            }
                        }
                        Err(_) => {
                            Err("Please enter a valid positive integer (max 64-bit).".to_string())
                        }
                    });
                }
            });

            ui.add_space(15.0);

            if let Some(ref res) = self.results {
                match res {
                    Ok((num, prime_check, factors)) => {
                        ui.group(|ui| {
                            ui.label(egui::RichText::new(format!("Analysis for {}", num)).strong());
                            ui.add_space(5.0);

                            ui.horizontal(|ui| {
                                ui.label("Is Prime:");
                                if *prime_check {
                                    ui.label(
                                        egui::RichText::new("Yes").color(egui::Color32::GREEN),
                                    );
                                } else {
                                    ui.label(egui::RichText::new("No").color(egui::Color32::RED));
                                }
                            });

                            ui.add_space(5.0);
                            ui.label("Prime Factors:");
                            if factors.is_empty() {
                                ui.label("None (less than 2)");
                            } else {
                                let mut factor_strings = Vec::new();
                                for (p, e) in factors {
                                    if *e == 1 {
                                        factor_strings.push(format!("{}", p));
                                    } else {
                                        factor_strings.push(format!("{}^{}", p, e));
                                    }
                                }
                                ui.label(factor_strings.join(" × "));
                            }
                        });
                    }
                    Err(err_msg) => {
                        ui.label(egui::RichText::new(err_msg).color(egui::Color32::RED));
                    }
                }
            }
        });
    }
}

// [cite:algorithmic_information_rust]
