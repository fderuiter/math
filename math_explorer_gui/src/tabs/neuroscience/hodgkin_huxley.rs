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
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

pub struct HodgkinHuxleyParams {
    pub g_na: AtomicU64,
    pub g_k: AtomicU64,
    pub g_l: AtomicU64,
    pub i_ext: AtomicU64,
}

impl HodgkinHuxleyParams {
    pub fn new(g_na: f64, g_k: f64, g_l: f64, i_ext: f64) -> Self {
        Self {
            g_na: AtomicU64::new(g_na.to_bits()),
            g_k: AtomicU64::new(g_k.to_bits()),
            g_l: AtomicU64::new(g_l.to_bits()),
            i_ext: AtomicU64::new(i_ext.to_bits()),
        }
    }
}

struct HodgkinHuxleyRunner {
    neuron: HodgkinHuxleyNeuron,
    params: HodgkinHuxleyParameters,
    shared_params: Arc<HodgkinHuxleyParams>,
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
            SimCommand::Reset => {
                self.neuron = HodgkinHuxleyNeuron::new(-65.0);
                self.time = 0.0;
                self.history.clear();
            }
            _ => {}
        }
    }

    fn step(&mut self) {
        let new_g_na = f64::from_bits(self.shared_params.g_na.load(Ordering::Relaxed));
        let new_g_k = f64::from_bits(self.shared_params.g_k.load(Ordering::Relaxed));
        let new_g_l = f64::from_bits(self.shared_params.g_l.load(Ordering::Relaxed));
        let new_i_ext = f64::from_bits(self.shared_params.i_ext.load(Ordering::Relaxed));
        
        let params_changed = self.params.g_na != new_g_na || 
                             self.params.g_k != new_g_k || 
                             self.params.g_l != new_g_l;

        self.params.g_na = new_g_na;
        self.params.g_k = new_g_k;
        self.params.g_l = new_g_l;
        self.i_ext = new_i_ext;

        if params_changed {
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
            pixels: std::sync::Arc::new(std::sync::RwLock::new(Vec::new())),
            custom_data,
            structured_data: None,
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
    shared_params: Arc<HodgkinHuxleyParams>,
}

impl Default for HodgkinHuxleyTool {
    fn default() -> Self {
        let params = HodgkinHuxleyParameters::default();
        let neuron = HodgkinHuxleyNeuron::new(-65.0);
        let i_ext = 10.0;
        let shared_params = Arc::new(HodgkinHuxleyParams::new(params.g_na, params.g_k, params.g_l, i_ext));

        let runner = HodgkinHuxleyRunner {
            neuron,
            params: params.clone(),
            shared_params: Arc::clone(&shared_params),
            history: VecDeque::new(),
            time: 0.0,
            i_ext,
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
            i_ext, // Default injection
            shared_params,
        }
    }
}

impl InteractiveTool for HodgkinHuxleyTool {
    fn name(&self) -> &'static str {
        "Hodgkin-Huxley Model"
    }

    fn theory(&self) -> Option<&dyn TheoryDescribable> {
        Some(self)
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
                self.shared_params.g_na.store(self.g_na.to_bits(), Ordering::Relaxed);
            }
            if ui
                .add(egui::Slider::new(&mut self.g_k, 0.0..=100.0).text("K+ (Potassium)"))
                .changed()
            {
                self.shared_params.g_k.store(self.g_k.to_bits(), Ordering::Relaxed);
            }
            if ui
                .add(egui::Slider::new(&mut self.g_l, 0.0..=5.0).text("Leak"))
                .changed()
            {
                self.shared_params.g_l.store(self.g_l.to_bits(), Ordering::Relaxed);
            }

            ui.separator();
            ui.label("Input");
            if ui
                .add(egui::Slider::new(&mut self.i_ext, 0.0..=50.0).text("I_ext (Current)"))
                .changed()
            {
                self.shared_params.i_ext.store(self.i_ext.to_bits(), Ordering::Relaxed);
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
                    self.shared_params.g_na.store(self.g_na.to_bits(), Ordering::Relaxed);
                    self.shared_params.g_k.store(self.g_k.to_bits(), Ordering::Relaxed);
                    self.shared_params.g_l.store(self.g_l.to_bits(), Ordering::Relaxed);
                    self.shared_params.i_ext.store(self.i_ext.to_bits(), Ordering::Relaxed);
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
                .show(ui, |plot_ui| {
                    plot_ui.line(line);
                });
        });
    }
}

impl TheoryDescribable for HodgkinHuxleyTool {
    fn theory_description(&self) -> String {
        HodgkinHuxleyModel::new(HodgkinHuxleyParameters::default(), 0.0).theory_description()
    }

    fn phonetic_description(&self) -> String {
        self.theory_description()
    }
    fn theory_citation(&self) -> String {
        HodgkinHuxleyModel::new(HodgkinHuxleyParameters::default(), 0.0).theory_citation()
    }
    fn available_descriptions(&self) -> std::collections::HashMap<String, String> {
        HodgkinHuxleyModel::new(HodgkinHuxleyParameters::default(), 0.0).available_descriptions()
    }
}

// Removed local reset and update_params logic.

// [cite:advanced_linear_algebra]


inventory::submit! {
    crate::framework::ToolMetadata {
        name: "HodgkinHuxleyTool",
        domain: "neuroscience",
        tags: &[],
        build: || Box::new(HodgkinHuxleyTool::default()),
    }
}
