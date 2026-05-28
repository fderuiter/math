use crate::tabs::ExplorerTab;
use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use math_explorer_core::discovery::{GenericSimulation, ParameterValue};
use math_explorer_core::state::StateData;
use std::collections::VecDeque;

pub struct BiologyTab {
    simulations: Vec<Box<dyn GenericSimulation>>,
    selected_sim_index: usize,
    history: VecDeque<[f64; 2]>,
    time: f64,
    is_running: bool,
}

impl Default for BiologyTab {
    fn default() -> Self {
        #[allow(unused_mut)]
        let mut simulations: Vec<Box<dyn GenericSimulation>> = vec![];
        
        #[cfg(feature = "biology")]
        {
            simulations = math_explorer::biology::get_simulations();
        }

        Self {
            simulations,
            selected_sim_index: 0,
            history: VecDeque::new(),
            time: 0.0,
            is_running: false,
        }
    }
}

impl ExplorerTab for BiologyTab {
    fn name(&self) -> &'static str {
        "Biology (Generic)"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.simulations.is_empty() {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("No biology simulations available. Enable the 'biology' feature.");
            });
            return;
        }

        egui::TopBottomPanel::top("biology_tool_selector").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Simulation:");
                let mut switch_to = None;
                egui::ScrollArea::horizontal().show(ui, |ui| {
                    for i in 0..self.simulations.len() {
                        let name = self.simulations[i].name().to_string();
                        if ui
                            .selectable_label(self.selected_sim_index == i, name)
                            .clicked()
                        {
                            if self.selected_sim_index != i {
                                switch_to = Some(i);
                            }
                        }
                    }
                });
                if let Some(idx) = switch_to {
                    self.selected_sim_index = idx;
                    self.time = 0.0;
                    self.history.clear();
                    self.is_running = false;
                    self.simulations[idx].reset();
                }
            });
        });

        let sim = &mut self.simulations[self.selected_sim_index];

        egui::SidePanel::left("biology_controls").show(ctx, |ui| {
            ui.heading("Parameters");
            ui.separator();
            
            let mut changes = vec![];
            for param in sim.get_parameters() {
                ui.label(&param.description);
                if let ParameterValue::Float(mut v) = param.value {
                    let mut min_val = 0.0;
                    let mut max_val = 100.0;
                    if let Some(ParameterValue::Float(min)) = param.min { min_val = min; }
                    if let Some(ParameterValue::Float(max)) = param.max { max_val = max; }
                    
                    if ui.add(egui::Slider::new(&mut v, min_val..=max_val).text(&param.name)).changed() {
                        changes.push((param.name.clone(), ParameterValue::Float(v)));
                    }
                }
            }
            
            for (name, val) in changes {
                sim.set_parameter(&name, val);
            }

            ui.separator();
            ui.heading("Simulation");
            ui.horizontal(|ui| {
                if ui.button(if self.is_running { "⏸ Pause" } else { "▶ Start" }).clicked() {
                    self.is_running = !self.is_running;
                }
                if ui.button("↻ Reset").clicked() {
                    sim.reset();
                    self.time = 0.0;
                    self.history.clear();
                    self.is_running = false;
                }
            });
            ui.label(format!("Time: {:.2}", self.time));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.is_running {
                let dt = 0.01;
                let steps = 10;
                for _ in 0..steps {
                    sim.step(dt, Some(10.0));
                    self.time += dt;
                    
                    if let StateData::TimeSeries { values, .. } = sim.get_state() {
                        if let Some(v) = values.first() {
                            self.history.push_back([self.time, *v]);
                        }
                    }
                }
                while self.history.len() > 10_000 {
                    self.history.pop_front();
                }
                ctx.request_repaint();
            }

            let state = sim.get_state();
            match state {
                StateData::TimeSeries { .. } => {
                    let line = Line::new("Membrane Potential", PlotPoints::from_iter(self.history.iter().copied()))
                        .color(egui::Color32::from_rgb(100, 200, 255));
                    Plot::new("biology_plot")
                        .view_aspect(2.0)
                        .show(ui, |plot_ui| plot_ui.line(line));
                }
                StateData::Discrete(grid) => {
                    // Quick and dirty viz for discrete grid
                    let w = 50; // hardcoded for demo
                    ui.label("Discrete Grid View (Live update)");
                    egui::ScrollArea::both().show(ui, |ui| {
                        let mut repr = String::new();
                        for (i, cell) in grid.iter().enumerate() {
                            if i > 0 && i % w == 0 {
                                repr.push('\n');
                            }
                            repr.push(if *cell == 1 { '█' } else { ' ' });
                        }
                        ui.monospace(repr);
                    });
                }
            }
        });
    }
}
