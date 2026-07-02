import re

with open('math_explorer_gui/src/tabs/neuroscience/hodgkin_huxley.rs', 'r') as f:
    content = f.read()

conflict = """<<<<<<< HEAD
    fn name() -> &'static str {
        "Hodgkin-Huxley Model"
    }

    fn create_theory() -> Option<Box<dyn TheoryDescribable>> {
        Some(Box::new(HodgkinHuxleyModel::new(HodgkinHuxleyParameters::default(), 0.0)))
    }
}
=======
pub struct HodgkinHuxleyTool {
    controller: SimulationController,
    history: VecDeque<[f64; 2]>, // Local cache for plotting
    params: Arc<RwLock<HodgkinHuxleyParameters>>,
    i_inj: f64, // Not currently passed in runner? We'll leave it as a parameter later.
}

impl Default for HodgkinHuxleyTool {
    fn default() -> Self {
        Self::new()
    }
}

impl HodgkinHuxleyTool {
    pub fn new() -> Self {
        let params = Arc::new(RwLock::new(HodgkinHuxleyParameters::default()));
        let runner = HodgkinHuxleyRunner::new(Arc::clone(&params));
        
        Self {
            controller: SimulationController::new(runner),
            history: VecDeque::with_capacity(1000),
            params,
            i_inj: 0.0,
        }
    }
}

impl crate::framework::InteractiveTool for HodgkinHuxleyTool {
    fn name(&self) -> &'static str {
        "Hodgkin-Huxley Model"
    }

    fn theory(&self) -> &dyn TheoryDescribable {
        self
    }

    fn show(&mut self, ctx: &egui::Context) {
        let is_running = self.controller.running;
        let mut sim_command = None;

        egui::SidePanel::left("hh_controls").show(ctx, |ui| {
            ui.heading("Hodgkin-Huxley Controls");
            
            ui.horizontal(|ui| {
                if ui.button(if is_running { "⏸ Pause" } else { "▶ Run" }).clicked() {
                    sim_command = Some(if is_running { SimCommand::Pause } else { SimCommand::Start });
                }
                if ui.button("↻ Reset").clicked() {
                    sim_command = Some(SimCommand::Reset);
                    self.history.clear();
                }
            });

            ui.separator();
            ui.heading("Parameters");
            
            let descs = self.available_descriptions();
            
            let mut p = self.params.write().unwrap();
            
            let mut add_slider = |val: &mut f64, range: std::ops::RangeInclusive<f64>, text: &str| {
                let mut resp = ui.add(egui::Slider::new(val, range).text(text));
                if let Some(desc) = descs.get(text) {
                    resp = resp.accessible_hover_text(desc);
                }
            };
            
            add_slider(&mut p.g_na, 0.0..=200.0, "g_Na");
            add_slider(&mut p.g_k, 0.0..=100.0, "g_K");
            add_slider(&mut p.g_l, 0.0..=1.0, "g_L");
            add_slider(&mut p.e_na, -50.0..=100.0, "e_Na");
            add_slider(&mut p.e_k, -100.0..=0.0, "e_K");
            add_slider(&mut p.e_l, -100.0..=0.0, "e_L");
            add_slider(&mut p.c_m, 0.1..=2.0, "c_M");
        });

        if let Some(cmd) = sim_command {
            self.controller.send_command(cmd);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(snapshot) = self.controller.update() {
                for chunk in snapshot.custom_data.chunks_exact(2) {
                    self.history.push_back([chunk[0], chunk[1]]);
                    if self.history.len() > 1000 {
                        self.history.pop_front();
                    }
                }
                ctx.request_repaint();
            } else if self.controller.running {
                ctx.request_repaint();
            }

            use eframe::egui_plot::{Line, Plot, PlotPoints};
            
            Plot::new("hh_plot")
                .view_aspect(2.0)
                .show(ui, |plot_ui| {
                    let points: PlotPoints = self.history.iter().copied().collect();
                    plot_ui.line(Line::new(points).name("Membrane Potential (mV)"));
                });
        });
    }
}

impl TheoryDescribable for HodgkinHuxleyTool {
    fn theory_description(&self) -> String {
        HodgkinHuxleyModel::new(HodgkinHuxleyParameters::default(), 0.0).theory_description()
    }
    
    fn phonetic_description(&self) -> String {
        HodgkinHuxleyModel::new(HodgkinHuxleyParameters::default(), 0.0).phonetic_description()
    }
    
    fn theory_citation(&self) -> String {
        HodgkinHuxleyModel::new(HodgkinHuxleyParameters::default(), 0.0).theory_citation()
    }
    
    fn available_descriptions(&self) -> std::collections::HashMap<String, String> {
        HodgkinHuxleyModel::new(HodgkinHuxleyParameters::default(), 0.0).available_descriptions()
    }
}
>>>>>>> origin/main"""

resolved = """    fn name() -> &'static str {
        "Hodgkin-Huxley Model"
    }
}

impl TheoryDescribable for HodgkinHuxleyUnified {
    fn theory_description(&self) -> String {
        HodgkinHuxleyModel::new(HodgkinHuxleyParameters::default(), 0.0).theory_description()
    }
    
    fn phonetic_description(&self) -> String {
        HodgkinHuxleyModel::new(HodgkinHuxleyParameters::default(), 0.0).phonetic_description()
    }
    
    fn theory_citation(&self) -> String {
        HodgkinHuxleyModel::new(HodgkinHuxleyParameters::default(), 0.0).theory_citation()
    }
    
    fn available_descriptions(&self) -> std::collections::HashMap<String, String> {
        HodgkinHuxleyModel::new(HodgkinHuxleyParameters::default(), 0.0).available_descriptions()
    }
}
"""

content = content.replace(conflict, resolved)
with open('math_explorer_gui/src/tabs/neuroscience/hodgkin_huxley.rs', 'w') as f:
    f.write(content)
