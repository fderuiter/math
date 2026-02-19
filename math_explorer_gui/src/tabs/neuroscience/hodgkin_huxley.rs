use super::NeuroscienceTool;
use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use math_explorer::biology::neuroscience::{HodgkinHuxleyNeuron, HodgkinHuxleyParameters};

pub struct HodgkinHuxleyTool {
    neuron: HodgkinHuxleyNeuron,
    params: HodgkinHuxleyParameters,
    history: Vec<[f64; 2]>, // [time, voltage]
    time: f64,
    is_running: bool,

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

        Self {
            neuron,
            g_na: params.g_na,
            g_k: params.g_k,
            g_l: params.g_l,
            params, // Keep a copy
            history: Vec::new(),
            time: 0.0,
            is_running: false,
            i_ext: 10.0, // Default injection
        }
    }
}

impl NeuroscienceTool for HodgkinHuxleyTool {
    fn name(&self) -> &'static str {
        "Hodgkin-Huxley Model"
    }

    fn show(&mut self, ctx: &egui::Context) {
        let mut params_changed = false;

        egui::SidePanel::left("hh_controls").show(ctx, |ui| {
            ui.heading("Parameters");
            ui.separator();

            ui.label("Conductances (mS/cm²)");
            if ui.add(egui::Slider::new(&mut self.g_na, 0.0..=200.0).text("Na+ (Sodium)")).changed() {
                params_changed = true;
            }
            if ui.add(egui::Slider::new(&mut self.g_k, 0.0..=100.0).text("K+ (Potassium)")).changed() {
                params_changed = true;
            }
            if ui.add(egui::Slider::new(&mut self.g_l, 0.0..=5.0).text("Leak")).changed() {
                params_changed = true;
            }

            ui.separator();
            ui.label("Input");
            if ui.add(egui::Slider::new(&mut self.i_ext, 0.0..=50.0).text("I_ext (Current)")).changed() {
                // I_ext is passed to update(), so we don't need to rebuild neuron
            }

            ui.separator();
            ui.heading("Simulation");
            ui.horizontal(|ui| {
                if ui.button(if self.is_running { "Pause" } else { "Start" }).clicked() {
                    self.is_running = !self.is_running;
                }
                if ui.button("Reset").clicked() {
                    self.reset();
                }
            });

            ui.label(format!("Time: {:.2} ms", self.time));
            ui.label(format!("Voltage: {:.2} mV", self.neuron.v()));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // Simulation Step
            if self.is_running {
                // If params changed via slider, update them
                if params_changed {
                    self.update_params();
                }

                // Run multiple steps per frame for smooth animation
                let dt = 0.01;
                let steps_per_frame = 10;

                for _ in 0..steps_per_frame {
                    self.neuron.update(dt, self.i_ext);
                    self.time += dt;

                    // Record history (every step or subsample?)
                    // Subsample to avoid memory explosion, or use a ring buffer.
                    // For now, just append. Maybe limit size.
                    self.history.push([self.time, self.neuron.v()]);
                }

                // Limit history size to keep UI responsive
                if self.history.len() > 10_000 {
                    self.history.drain(0..100);
                }

                // Request repaint to animate
                ctx.request_repaint();
            } else if params_changed {
                 self.update_params();
            }

            // Plotting
            let line = Line::new("Membrane Potential (V)", PlotPoints::new(self.history.clone()))
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

impl HodgkinHuxleyTool {
    fn reset(&mut self) {
        self.neuron = HodgkinHuxleyNeuron::new(-65.0);
        self.time = 0.0;
        self.history.clear();
        self.is_running = false;
        // Keep current slider values and apply them to the new neuron
        self.update_params();
    }

    fn update_params(&mut self) {
        // Update local params struct
        self.params.g_na = self.g_na;
        self.params.g_k = self.g_k;
        self.params.g_l = self.g_l;

        // Reconstruct neuron with new params but OLD state
        // We need to use the builder to inject current state + new params
        let v = self.neuron.v();
        let n = self.neuron.n();
        let m = self.neuron.m();
        let h = self.neuron.h();

        // Note: We use try_new_with_params or builder.
        // But builder allows setting initial state.

        let builder = HodgkinHuxleyNeuron::builder()
            .with_initial_v(v)
            .with_n(n)
            .with_m(m)
            .with_h(h)
            .with_params(self.params.clone());

        if let Ok(new_neuron) = builder.build() {
            self.neuron = new_neuron;
        } else {
            // If parameters are invalid (e.g. negative), we might fail.
            // But sliders are clamped to positive ranges.
            // Log error or ignore?
            eprintln!("Failed to rebuild neuron with new parameters");
        }
    }
}
