use crate::accessibility::PlotAccessibilityExt;
use super::NeuroscienceTool;
use crate::accessibility::AccessibleHoverText;
use crate::async_sim::{SimCommand, SimulationController, SimulationRunner, StateSnapshot};
use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use math_explorer::biology::neuroscience::{HodgkinHuxleyNeuron, HodgkinHuxleyParameters};
use std::collections::VecDeque;
use std::sync::Arc;

struct HodgkinHuxleyRunner {
    neuron: HodgkinHuxleyNeuron,
    params: HodgkinHuxleyParameters,
    history: VecDeque<[f64; 2]>,
    time: f64,
    i_ext: f64,
    dt: f64,
    steps_per_frame: usize,
}

impl SimulationRunner for HodgkinHuxleyRunner {
    fn process_command(&mut self, cmd: SimCommand) {
        match cmd {
            SimCommand::SetSpeed(speed) => self.steps_per_frame = speed,
            SimCommand::UpdateParam(ref name, val) => match name.as_str() {
                "g_na" => self.params.g_na = val,
                "g_k" => self.params.g_k = val,
                "g_l" => self.params.g_l = val,
                "i_ext" => self.i_ext = val,
                _ => {}
            },
            SimCommand::Reset => {
                self.neuron = HodgkinHuxleyNeuron::new(-65.0);
                self.time = 0.0;
                self.history.clear();
            }
            _ => {}
        }

        // Reconstruct neuron if parameters changed
        if let SimCommand::UpdateParam(name, _) = cmd {
            if name != "i_ext" {
                let builder = HodgkinHuxleyNeuron::builder()
                    .with_initial_v(self.neuron.v())
                    .with_n(self.neuron.n())
                    .with_m(self.neuron.m())
                    .with_h(self.neuron.h())
                    .with_params(self.params.clone());

                if let Ok(new_neuron) = builder.build() {
                    self.neuron = new_neuron;
                }
            }
        }
    }

    fn step(&mut self) {
        self.neuron.update(self.dt, self.i_ext);
        self.time += self.dt;
        self.history.push_back([self.time, self.neuron.v()]);
        while self.history.len() > 10_000 {
            self.history.pop_front();
        }
    }

    fn get_snapshot(&self) -> StateSnapshot {
        // Flatten history and push current time/voltage as the last two elements
        let mut custom_data = Vec::with_capacity(self.history.len() * 2 + 2);
        for &[t, v] in &self.history {
            custom_data.push(t);
            custom_data.push(v);
        }
        custom_data.push(self.time);
        custom_data.push(self.neuron.v());

        StateSnapshot {
            width: 0,
            height: 0,
            pixels: Arc::new(Vec::new()),
            custom_data, structured_data: None,
        }
    }

    fn get_steps_per_frame(&self) -> usize {
        self.steps_per_frame
    }
}

pub struct HodgkinHuxleyTool {
    controller: SimulationController,
    history: VecDeque<[f64; 2]>, // Local cache for plotting
    time: f64,
    voltage: f64,

    // UI State for sliders (to avoid modifying params directly every frame)
    g_na: f64,
    g_k: f64,
    g_l: f64,
    i_ext: f64,
}

impl Default for HodgkinHuxleyTool {
    fn default() -> Self {
        let params = HodgkinHuxleyParameters::default();
        let neuron = HodgkinHuxleyNeuron::new(-65.0);

        let runner = HodgkinHuxleyRunner {
            neuron,
            params: params.clone(),
            history: VecDeque::new(),
            time: 0.0,
            i_ext: 10.0,
            dt: 0.01,
            steps_per_frame: 10,
        };

        Self {
            controller: SimulationController::new(runner),
            g_na: params.g_na,
            g_k: params.g_k,
            g_l: params.g_l,
            history: VecDeque::new(),
            time: 0.0,
            voltage: -65.0,
            i_ext: 10.0, // Default injection
        }
    }
}

impl NeuroscienceTool for HodgkinHuxleyTool {
    fn name(&self) -> &'static str {
        "Hodgkin-Huxley Model"
    }

    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    fn show(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("hh_controls").show(ctx, |ui| {
            ui.heading("Parameters");
            ui.separator();

            ui.label("Conductances (mS/cm²)");
            if ui
                .add(egui::Slider::new(&mut self.g_na, 0.0..=200.0).text("Na+ (Sodium)"))
                .changed()
            {
                self.controller
                    .send_command(SimCommand::UpdateParam("g_na".to_string(), self.g_na));
            }
            if ui
                .add(egui::Slider::new(&mut self.g_k, 0.0..=100.0).text("K+ (Potassium)"))
                .changed()
            {
                self.controller
                    .send_command(SimCommand::UpdateParam("g_k".to_string(), self.g_k));
            }
            if ui
                .add(egui::Slider::new(&mut self.g_l, 0.0..=5.0).text("Leak"))
                .changed()
            {
                self.controller
                    .send_command(SimCommand::UpdateParam("g_l".to_string(), self.g_l));
            }

            ui.separator();
            ui.label("Input");
            if ui
                .add(egui::Slider::new(&mut self.i_ext, 0.0..=50.0).text("I_ext (Current)"))
                .changed()
            {
                self.controller
                    .send_command(SimCommand::UpdateParam("i_ext".to_string(), self.i_ext));
            }

            ui.separator();
            ui.heading("Simulation");
            ui.horizontal(|ui| {
                let is_running = self.controller.running;
                if ui
                    .button(if is_running { "⏸ Pause" } else { "▶ Start" })
                    .accessible_hover_text(if is_running {
                        "Pause the Hodgkin-Huxley simulation"
                    } else {
                        "Start the Hodgkin-Huxley simulation"
                    })
                    .clicked()
                {
                    if is_running {
                        self.controller.send_command(SimCommand::Pause);
                    } else {
                        self.controller.send_command(SimCommand::Start);
                    }
                }
                if ui
                    .button("↻ Reset")
                    .accessible_hover_text("Reset the simulation to its initial state")
                    .clicked()
                {
                    self.controller.send_command(SimCommand::Reset);
                    self.time = 0.0;
                    self.voltage = -65.0;
                    self.history.clear();

                    // Re-apply current slider values
                    self.controller
                        .send_command(SimCommand::UpdateParam("g_na".to_string(), self.g_na));
                    self.controller
                        .send_command(SimCommand::UpdateParam("g_k".to_string(), self.g_k));
                    self.controller
                        .send_command(SimCommand::UpdateParam("g_l".to_string(), self.g_l));
                    self.controller
                        .send_command(SimCommand::UpdateParam("i_ext".to_string(), self.i_ext));
                }
            });

            ui.label(format!("Time: {:.2} ms", self.time));
            ui.label(format!("Voltage: {:.2} mV", self.voltage));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // Update from background thread
            if let Some(snapshot) = self.controller.update() {
                if snapshot.custom_data.len() >= 2 {
                    self.voltage = *snapshot.custom_data.last().unwrap();
                    self.time = snapshot.custom_data[snapshot.custom_data.len() - 2];

                    // Reconstruct history
                    self.history.clear();
                    for i in (0..snapshot.custom_data.len() - 2).step_by(2) {
                        self.history
                            .push_back([snapshot.custom_data[i], snapshot.custom_data[i + 1]]);
                    }
                }
            }

            if self.controller.running {
                // Request repaint to animate
                ctx.request_repaint();
            }

            // Plotting
            let line = Line::new(
                "Membrane Potential (V)",
                PlotPoints::from_iter(self.history.iter().copied()),
            )
            .color(egui::Color32::from_rgb(100, 200, 255));

            Plot::new("hh_voltage_plot")
                .x_axis_label("Time (ms)")
                .y_axis_label("Voltage (mV)")
                .view_aspect(2.0)
                .show_accessible(ui, "Dynamic state of hh_voltage_plot updated.", |plot_ui| {
                    plot_ui.line(line);
                });
        });
    }
}

// Removed local reset and update_params logic.

// [cite:advanced_linear_algebra]
