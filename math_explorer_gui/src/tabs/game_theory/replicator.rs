use super::GameTheoryTool;
use eframe::egui;
use egui_plot::{Legend, Line, Plot, PlotPoints};
use math_explorer::applied::game_theory::evolutionary::ReplicatorDynamics;
use nalgebra::{DMatrix, DVector};

pub struct ReplicatorDynamicsTool {
    payoff_matrix: DMatrix<f64>,
    initial_population: DVector<f64>,
    trajectory: Vec<(f64, DVector<f64>)>,
    time_horizon: f64,
    dt: f64,
    strategy_names: Vec<String>,
}

impl Default for ReplicatorDynamicsTool {
    fn default() -> Self {
        // Default to Hawk-Dove: V=2, C=4.
        // H vs H: (V-C)/2 = -1
        // H vs D: V = 2
        // D vs H: 0
        // D vs D: V/2 = 1
        let payoff = DMatrix::from_row_slice(
            2,
            2,
            &[
                -1.0, 2.0, // Hawk row
                0.0, 1.0, // Dove row
            ],
        );
        let init = DVector::from_vec(vec![0.1, 0.9]); // Mostly Doves initially
        let mut tool = Self {
            payoff_matrix: payoff,
            initial_population: init,
            trajectory: Vec::new(),
            time_horizon: 20.0,
            dt: 0.05,
            strategy_names: vec!["Hawk".to_string(), "Dove".to_string()],
        };
        tool.run_simulation();
        tool
    }
}

impl ReplicatorDynamicsTool {
    fn run_simulation(&mut self) {
        // Normalize initial population
        let sum = self.initial_population.sum();
        if sum > 0.0 {
            self.initial_population /= sum;
        } else {
            // Fallback to uniform
            let n = self.initial_population.len();
            self.initial_population = DVector::from_element(n, 1.0 / n as f64);
        }

        if let Ok(system) = ReplicatorDynamics::new(self.payoff_matrix.clone()) {
            self.trajectory =
                system.simulate(self.initial_population.clone(), self.time_horizon, self.dt);
        }
    }

    fn load_preset_hawk_dove(&mut self) {
        self.payoff_matrix = DMatrix::from_row_slice(2, 2, &[-1.0, 2.0, 0.0, 1.0]);
        self.initial_population = DVector::from_vec(vec![0.1, 0.9]);
        self.strategy_names = vec!["Hawk".to_string(), "Dove".to_string()];
        self.time_horizon = 20.0;
        self.run_simulation();
    }

    fn load_preset_rps(&mut self) {
        self.payoff_matrix =
            DMatrix::from_row_slice(3, 3, &[0.0, -1.0, 1.0, 1.0, 0.0, -1.0, -1.0, 1.0, 0.0]);
        self.initial_population = DVector::from_vec(vec![0.33, 0.33, 0.34]);
        self.strategy_names = vec![
            "Rock".to_string(),
            "Paper".to_string(),
            "Scissors".to_string(),
        ];
        self.time_horizon = 50.0;
        self.run_simulation();
    }
}

impl GameTheoryTool for ReplicatorDynamicsTool {
    fn name(&self) -> &'static str {
        "Replicator Dynamics"
    }

    fn show(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("replicator_control_panel").show(ctx, |ui| {
            ui.heading("Settings");
            ui.separator();

            ui.label("Presets:");
            ui.horizontal(|ui| {
                if ui.button("Hawk-Dove").clicked() {
                    self.load_preset_hawk_dove();
                }
                if ui.button("RPS").clicked() {
                    self.load_preset_rps();
                }
            });

            ui.separator();
            ui.heading("Payoff Matrix");
            let rows = self.payoff_matrix.nrows();
            let cols = self.payoff_matrix.ncols();

            egui::Grid::new("payoff_grid").striped(true).show(ui, |ui| {
                // Header
                ui.label("");
                for j in 0..cols {
                    ui.label(
                        self.strategy_names
                            .get(j)
                            .map(|s| s.as_str())
                            .unwrap_or("?"),
                    );
                }
                ui.end_row();

                for i in 0..rows {
                    ui.label(
                        self.strategy_names
                            .get(i)
                            .map(|s| s.as_str())
                            .unwrap_or("?"),
                    );
                    for j in 0..cols {
                        ui.add(egui::DragValue::new(&mut self.payoff_matrix[(i, j)]).speed(0.1));
                    }
                    ui.end_row();
                }
            });

            ui.separator();
            ui.heading("Initial Population");
            let mut changed = false;
            for i in 0..self.initial_population.len() {
                ui.horizontal(|ui| {
                    ui.label(
                        self.strategy_names
                            .get(i)
                            .map(|s| s.as_str())
                            .unwrap_or("?"),
                    );
                    if ui
                        .add(egui::Slider::new(
                            &mut self.initial_population[i],
                            0.0..=1.0,
                        ))
                        .changed()
                    {
                        changed = true;
                    }
                });
            }

            if changed {
                // Optional: Auto-normalize or wait for Run?
                // Waiting for Run is safer for UX to avoid sliders jumping around.
            }

            ui.separator();
            ui.heading("Simulation");
            ui.add(egui::Slider::new(&mut self.time_horizon, 10.0..=200.0).text("Duration"));
            ui.add(egui::Slider::new(&mut self.dt, 0.01..=0.5).text("Time Step"));

            if ui.button("Run Simulation").clicked() {
                self.run_simulation();
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            Plot::new("replicator_plot")
                .legend(Legend::default())
                .allow_drag(true)
                .allow_zoom(true)
                .show(ui, |plot_ui| {
                    if self.trajectory.is_empty() {
                        return;
                    }

                    let n_strategies = self.trajectory[0].1.len();

                    for i in 0..n_strategies {
                        let points: PlotPoints = self
                            .trajectory
                            .iter()
                            .map(|(t, state)| [*t, state[i]])
                            .collect();

                        let name = self
                            .strategy_names
                            .get(i)
                            .cloned()
                            .unwrap_or(format!("Strategy {}", i));
                        plot_ui.line(Line::new(name, points));
                    }
                });
        });
    }
}
