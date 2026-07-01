use crate::accessibility::AccessibleHoverText;
use crate::async_sim::{SimCommand, SimulationController, SimulationRunner, StateSnapshot};
use crate::framework::InteractiveTool;
use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use math_commons::theory::TheoryDescribable;
use math_explorer::biology::neuroscience::{
    HodgkinHuxleyModel, HodgkinHuxleyNeuron, HodgkinHuxleyParameters,
};
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
            SimCommand::UpdateTypedParam(math_commons::generated_schemas::TypedModelCommand::HodgkinHuxley(p)) => {
                self.params.g_na = p.g_na;
                self.params.g_k = p.g_k;
                self.params.g_l = p.g_l;
                self.i_ext = p.i_ext;
            }
            SimCommand::Reset => {
                self.neuron = HodgkinHuxleyNeuron::new(-65.0);
                self.time = 0.0;
                self.history.clear();
            }
            _ => {}
        }

        // Reconstruct neuron if parameters changed
        if let SimCommand::UpdateTypedParam(math_commons::generated_schemas::TypedModelCommand::HodgkinHuxley(_)) = cmd {
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

    fn step(&mut self) {
        self.neuron.update(self.dt, self.i_ext);
        self.time += self.dt;
        self.history.push_back([self.time, self.neuron.v()]);
        while self.history.len() > 10_000 {
            self.history.pop_front();
        }
    }

    fn get_snapshot(&self) -> StateSnapshot {
        StateSnapshot {
            width: 0,
            height: 0,
            pixels: Arc::new(Vec::new()),
            custom_data: Vec::new(),
            structured_data: Some(Box::new((self.time, self.neuron.v(), self.history.clone()))),
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
    schema_params: math_commons::generated_schemas::HodgkinHuxleyParams,
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
            schema_params: math_commons::generated_schemas::HodgkinHuxleyParams::default(),
history: VecDeque::new(),
time: 0.0,
voltage: -65.0, // Default injection
        }
    }
}

impl InteractiveTool for HodgkinHuxleyTool {
    fn name(&self) -> &'static str {
        "Hodgkin-Huxley Model"
    }

    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    fn show(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("hh_controls").show(ctx, |ui| {
            let dummy_model = HodgkinHuxleyModel::new(HodgkinHuxleyParameters::default(), 0.0);
            ui.heading("Parameters")
                .accessible_hover_text(dummy_model.theory_description());
            ui.separator();

            if let Some(cmd) = crate::generated_ui::generate_ui_HodgkinHuxley(ui, &mut self.schema_params) {
                self.controller.send_command(SimCommand::UpdateTypedParam(cmd));
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
                        .send_command(SimCommand::UpdateTypedParam(
                            math_commons::generated_schemas::TypedModelCommand::HodgkinHuxley(self.schema_params)
                        ));
                }
            });

            ui.label(format!("Time: {:.2} ms", self.time));
            ui.label(format!("Voltage: {:.2} mV", self.voltage));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // Update from background thread
            if let Some(snapshot) = self.controller.update() {
                if let Some(data) = &snapshot.structured_data {
                    if let Some((t, v, hist)) = data.downcast_ref::<(f64, f64, std::collections::VecDeque<[f64; 2]>)>() {
                        self.time = *t;
                        self.voltage = *v;
                        self.history = hist.clone();
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
                .show(ui, |plot_ui| {
                    plot_ui.line(line);
                });
        });
    }
}

// Removed local reset and update_params logic.

// [cite:advanced_linear_algebra]
