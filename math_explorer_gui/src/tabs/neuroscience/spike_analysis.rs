use crate::accessibility::AccessibleHoverText;
use crate::framework::InteractiveTool;
use eframe::egui;
use egui_plot::{Bar, BarChart, Line, Plot, PlotPoints};
use math_explorer::biology::neuroscience::HodgkinHuxleyNeuron;
use std::collections::VecDeque;

pub struct SpikeAnalysisTool {
    // Simulation State
    neuron: HodgkinHuxleyNeuron,
    time: f64,
    is_running: bool,
    history: VecDeque<[f64; 2]>, // [time, voltage]

    // Analysis State
    spike_times: Vec<f64>,
    isis: VecDeque<f64>, // Inter-Spike Intervals (ms)
    last_voltage: f64,

    // UI Controls
    input_current: f64,
    spike_threshold: f64,
    simulation_speed: usize,
}

impl Default for SpikeAnalysisTool {
    fn default() -> Self {
        let neuron = HodgkinHuxleyNeuron::new(-65.0);

        Self {
            neuron,
            time: 0.0,
            is_running: false,
            history: VecDeque::new(),
            spike_times: Vec::new(),
            isis: VecDeque::new(),
            last_voltage: -65.0,
            input_current: 10.0,
            spike_threshold: 0.0,
            simulation_speed: 10,
        }
    }
}

impl InteractiveTool for SpikeAnalysisTool {
    fn name(&self) -> &'static str {
        "Spike Train Analysis"
    }

    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    fn show(&mut self, ctx: &egui::Context) {
        // --- Controls Panel ---
        egui::SidePanel::left("spike_analysis_controls").show(ctx, |ui| {
            ui.heading("Configuration");
            ui.separator();

            ui.add(egui::Slider::new(&mut self.input_current, 0.0..=50.0).text("Input Current (µA/cm²) - I_ext"));

            ui.add(egui::Slider::new(&mut self.spike_threshold, -50.0..=20.0).text("Spike Threshold (mV) - Threshold"));

            ui.separator();
            ui.add(egui::Slider::new(&mut self.simulation_speed, 1..=100).text("Simulation Speed (steps/frame)"));

            ui.separator();
            ui.horizontal(|ui| {
                if ui
                    .button(if self.is_running {
                        "⏸ Pause"
                    } else {
                        "▶ Run"
                    })
                    .accessible_hover_text(if self.is_running {
                        "Pause the spike analysis simulation"
                    } else {
                        "Start the spike analysis simulation"
                    })
                    .clicked()
                {
                    self.is_running = !self.is_running;
                }
                if ui
                    .button("↻ Reset")
                    .accessible_hover_text("Reset the simulation to its initial state")
                    .clicked()
                {
                    self.reset();
                }
            });

            ui.separator();
            ui.label(format!("Time: {:.2} ms", self.time));
            ui.label(format!("Spikes Detected: {}", self.spike_times.len()));
            if let Some(last_isi) = self.isis.back() {
                ui.label(format!("Last ISI: {:.2} ms", last_isi));
            }
            if !self.isis.is_empty() {
                let mean_isi: f64 = self.isis.iter().sum::<f64>() / self.isis.len() as f64;
                ui.label(format!("Mean ISI: {:.2} ms", mean_isi));
                ui.label(format!("Firing Rate: {:.2} Hz", 1000.0 / mean_isi));
            }
        });

        // --- Visualization Panel ---
        egui::CentralPanel::default().show(ctx, |ui| {
            // Update Simulation
            if self.is_running {
                self.step_simulation();
                ctx.request_repaint();
            }

            // Top Plot: Voltage Trace & Raster
            let height = ui.available_height() / 2.0;
            ui.push_id("voltage_trace", |ui| {
                Plot::new("voltage_plot")
                    .height(height)
                    .x_axis_label("Time (ms)")
                    .y_axis_label("Membrane Potential (mV)")
                    .show(ui, |plot_ui| {
                        // Voltage Line
                        plot_ui.line(Line::new(
                            "Voltage",
                            PlotPoints::new(self.history.iter().copied().collect::<Vec<_>>()),
                        ));

                        // Threshold Line
                        plot_ui.hline(
                            egui_plot::HLine::new("Threshold", self.spike_threshold)
                                .color(egui::Color32::RED),
                        );

                        // Spike Markers (Raster)
                        // Using VLine might be too much if there are many spikes, maybe Points?
                        // Let's use Points at (time, threshold)
                        let spike_points: Vec<[f64; 2]> = self
                            .spike_times
                            .iter()
                            .map(|&t| [t, self.spike_threshold])
                            .collect();
                        plot_ui.points(
                            egui_plot::Points::new("Spikes", PlotPoints::new(spike_points))
                                .radius(3.0_f32)
                                .color(egui::Color32::GREEN),
                        );
                    });
            });

            ui.separator();

            // Bottom Plot: ISI Histogram
            ui.push_id("isi_histogram", |ui| {
                Plot::new("isi_plot")
                    .x_axis_label("Inter-Spike Interval (ms)")
                    .y_axis_label("Count")
                    .show(ui, |plot_ui| {
                        if !self.isis.is_empty() {
                            let bars = self.compute_histogram(20); // 20 bins
                            plot_ui.bar_chart(
                                BarChart::new("ISI Distribution", bars).color(egui::Color32::GOLD),
                            );
                        }
                    });
            });
        });
    }
}

impl SpikeAnalysisTool {
    fn step_simulation(&mut self) {
        let dt = 0.01; // Time step (ms)

        for _ in 0..self.simulation_speed {
            // Update Neuron
            self.neuron.update(dt, self.input_current);
            let v = self.neuron.v();
            self.time += dt;

            // Spike Detection (Rising Edge)
            if v >= self.spike_threshold && self.last_voltage < self.spike_threshold {
                self.record_spike(self.time);
            }

            // Record History (subsample to avoid memory explosion)
            // Recording every 10th step = 0.1ms resolution
            // Or just check time % 0.1 < dt
            if (self.time * 10.0).round() % 1.0 == 0.0 {
                self.history.push_back([self.time, v]);
            }

            // Manage History Size (keep last 5000 points = 500ms window approx if subsampled)
            // But for analysis we might want longer history. Let's keep last 2000ms.
            // 2000ms / 0.1ms = 20,000 points.
            if self.history.len() > 20_000 {
                self.history.pop_front();
            }

            self.last_voltage = v;
        }
    }

    fn record_spike(&mut self, time: f64) {
        if let Some(&last_spike_time) = self.spike_times.last() {
            let isi = time - last_spike_time;
            self.isis.push_back(isi);
        }
        self.spike_times.push(time);

        // Keep ISI history manageable? Maybe last 1000 intervals.
        if self.isis.len() > 1000 {
            self.isis.pop_front();
        }
        // Keep spike times aligned with view?
        // If we clear history, we might lose context.
        // For now, simple vector is fine.
    }

    fn reset(&mut self) {
        self.neuron = HodgkinHuxleyNeuron::new(-65.0);
        self.time = 0.0;
        self.history.clear();
        self.spike_times.clear();
        self.isis.clear();
        self.last_voltage = -65.0;
        self.is_running = false;
    }

    fn compute_histogram(&self, num_bins: usize) -> Vec<Bar> {
        if self.isis.is_empty() {
            return Vec::new();
        }

        let (min_isi, max_isi) = self
            .isis
            .iter()
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &val| {
                (min.min(val), max.max(val))
            });

        if (max_isi - min_isi).abs() < 1e-6 {
            // All ISIs are the same (perfect periodicity)
            return vec![Bar::new(min_isi, self.isis.len() as f64).width(0.5)];
        }

        let bin_width = (max_isi - min_isi) / num_bins as f64;
        let mut counts = vec![0; num_bins];

        for &isi in &self.isis {
            let mut bin_idx = ((isi - min_isi) / bin_width).floor() as usize;
            if bin_idx >= num_bins {
                bin_idx = num_bins - 1;
            }
            counts[bin_idx] += 1;
        }

        counts
            .into_iter()
            .enumerate()
            .map(|(i, count)| {
                let center = min_isi + (i as f64 + 0.5) * bin_width;
                Bar::new(center, count as f64).width(bin_width * 0.9)
            })
            .collect()
    }
}

// [cite:dwarf_galaxy_empirical_dependencies]
