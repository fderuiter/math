use eframe::egui;
use oxidize_core::plugin::{Plugin, DynamicSimulation};

pub struct MathExplorerApp {
    plugins: Vec<&'static dyn Plugin>,
    selected_plugin: usize,
    active_sim: Option<Box<dyn DynamicSimulation>>,
    config_json: String,
    state_json: String,
    error_msg: Option<String>,
}

impl Default for MathExplorerApp {
    fn default() -> Self {
        let mut plugins = vec![];
        for plugin in inventory::iter::<&'static dyn Plugin> {
            plugins.push(*plugin);
        }

        Self {
            plugins,
            selected_plugin: 0,
            active_sim: None,
            config_json: String::new(),
            state_json: String::new(),
            error_msg: None,
        }
    }
}

impl MathExplorerApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MathExplorerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("main_menu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Math Explorer (Dynamic)");
                ui.separator();
                egui::ScrollArea::horizontal().show(ui, |ui| {
                    for (i, plugin) in self.plugins.iter().enumerate() {
                        if ui.selectable_label(self.selected_plugin == i, plugin.name()).clicked() {
                            if self.selected_plugin != i {
                                self.selected_plugin = i;
                                self.active_sim = None;
                                self.config_json = plugin.get_default_config_json();
                                self.state_json = String::new();
                                self.error_msg = None;
                            }
                        }
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.plugins.is_empty() {
                ui.centered_and_justified(|ui| {
                    ui.label("No plugins discovered.");
                });
                return;
            }

            let plugin = self.plugins[self.selected_plugin];
            ui.heading(plugin.name());
            ui.label(plugin.description());
            ui.separator();

            if self.active_sim.is_none() {
                ui.label("Configuration (JSON):");
                if self.config_json.is_empty() {
                    self.config_json = plugin.get_default_config_json();
                }
                ui.text_edit_multiline(&mut self.config_json);
                if ui.button("Initialize Simulation").clicked() {
                    match plugin.initialize_from_json(&self.config_json) {
                        Ok(sim) => {
                            self.active_sim = Some(sim);
                            self.error_msg = None;
                        }
                        Err(e) => {
                            self.error_msg = Some(e);
                        }
                    }
                }
            } else {
                ui.horizontal(|ui| {
                    if ui.button("Step Simulation").clicked() {
                        if let Some(sim) = &mut self.active_sim {
                            if let Err(e) = sim.step() {
                                self.error_msg = Some(e);
                            } else {
                                self.state_json = sim.get_state_json();
                            }
                        }
                    }
                    if ui.button("Stop Simulation").clicked() {
                        self.active_sim = None;
                    }
                });

                ui.separator();
                ui.label("State (JSON):");
                ui.text_edit_multiline(&mut self.state_json);
            }

            if let Some(err) = &self.error_msg {
                ui.colored_label(egui::Color32::RED, err);
            }
        });
    }
}
