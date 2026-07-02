use crate::accessibility::AccessibleHoverText;
use crate::async_sim::{SimCommand, SimulationController, SimulationRunner, StateSnapshot};
use crate::framework::InteractiveTool;
use eframe::egui;
use egui::{ColorImage, TextureOptions};
use egui_plot::{Plot, PlotImage, PlotPoint};
use math_commons::theory::ParameterConstraint;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub trait UnifiedModel: Send + 'static {
    /// Initialize the model given the starting parameters.
    fn new(params: &HashMap<String, f64>) -> Self where Self: Sized;
    
    /// Step the simulation forward one tick.
    fn step(&mut self, params: &HashMap<String, f64>);
    
    /// Extract a visual snapshot of the state.
    fn get_snapshot(&self) -> StateSnapshot;
    
    /// Process any custom commands (e.g. ApplyBrush). Default is no-op.
    fn process_command(&mut self, _cmd: SimCommand, _params: &HashMap<String, f64>) {}
    
    /// Return parameter definitions for automatic UI generation.
    fn parameters() -> HashMap<String, ParameterConstraint> where Self: Sized;
    
    /// The name of the tool.
    fn name() -> &'static str where Self: Sized;
    
    /// The name of the model in the theory portal.
    fn theory_description() -> Option<String> where Self: Sized { None }
}

pub struct UnifiedSimRunner<M: UnifiedModel> {
    model: M,
    params: Arc<RwLock<HashMap<String, f64>>>,
    steps_per_frame: usize,
}

impl<M: UnifiedModel> UnifiedSimRunner<M> {
    pub fn new(params: Arc<RwLock<HashMap<String, f64>>>) -> Self {
        let initial_params = params.read().unwrap().clone();
        Self {
            model: M::new(&initial_params),
            params,
            steps_per_frame: 5,
        }
    }
}

impl<M: UnifiedModel> SimulationRunner for UnifiedSimRunner<M> {
    fn process_command(&mut self, cmd: SimCommand) {
        let current_params = self.params.read().unwrap().clone();
        match cmd {
            SimCommand::SetSpeed(speed) => self.steps_per_frame = speed,
            SimCommand::Reset => {
                self.model = M::new(&current_params);
            }
            _ => self.model.process_command(cmd, &current_params),
        }
    }

    fn step(&mut self) {
        let current_params = self.params.read().unwrap().clone();
        self.model.step(&current_params);
    }

    fn get_snapshot(&self) -> StateSnapshot {
        self.model.get_snapshot()
    }

    fn get_steps_per_frame(&self) -> usize {
        self.steps_per_frame
    }
}

struct CachedSnapshot {
    width: usize,
    height: usize,
    pixels: Arc<Vec<egui::Color32>>,
}

pub struct UnifiedSimTool<M: UnifiedModel> {
    controller: SimulationController,
    params: Arc<RwLock<HashMap<String, f64>>>,
    param_metadata: Vec<(String, ParameterConstraint)>,
    steps_per_frame: usize,
    last_snapshot: Option<CachedSnapshot>,
    texture: Option<egui::TextureHandle>,
    _marker: std::marker::PhantomData<M>,
}

impl<M: UnifiedModel> UnifiedSimTool<M> {
    pub fn new() -> Self {
        let param_map = M::parameters();
        let mut param_metadata: Vec<_> = param_map.into_iter().collect();
        param_metadata.sort_by(|a, b| a.0.cmp(&b.0)); // Sort for consistent UI order

        let mut initial_params = HashMap::new();
        for (name, constraint) in &param_metadata {
            initial_params.insert(name.clone(), (constraint.min + constraint.max) / 2.0);
        }

        let params = Arc::new(RwLock::new(initial_params));
        let runner = UnifiedSimRunner::<M>::new(Arc::clone(&params));
        let controller = SimulationController::new(runner);

        Self {
            controller,
            params,
            param_metadata,
            steps_per_frame: 5,
            last_snapshot: None,
            texture: None,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<M: UnifiedModel> InteractiveTool for UnifiedSimTool<M> {
    fn name(&self) -> &'static str {
        M::name()
    }

    fn show(&mut self, ctx: &egui::Context) {
        if let Some(snapshot) = self.controller.update() {
            self.last_snapshot = Some(CachedSnapshot {
                width: snapshot.width,
                height: snapshot.height,
                pixels: Arc::clone(&snapshot.pixels),
            });
            ctx.request_repaint();
        } else if self.controller.running {
            ctx.request_repaint();
        }

        let is_running = self.controller.running;

        egui::SidePanel::left(format!("{}_controls", M::name())).show(ctx, |ui| {
            ui.heading(M::name());
            ui.separator();

            if ui
                .button(if is_running { "⏸ Pause" } else { "▶ Run" })
                .accessible_hover_text(if is_running { "Pause simulation" } else { "Start simulation" })
                .clicked()
            {
                if is_running {
                    self.controller.send_command(SimCommand::Pause);
                } else {
                    self.controller.send_command(SimCommand::Start);
                }
            }

            if ui.button("↻ Reset").accessible_hover_text("Reset simulation state").clicked() {
                self.controller.send_command(SimCommand::Reset);
            }

            ui.separator();
            ui.label("Simulation Constants");
            if ui
                .add(egui::Slider::new(&mut self.steps_per_frame, 1..=100).text("Speed (Steps/Frame)"))
                .changed()
            {
                self.controller.send_command(SimCommand::SetSpeed(self.steps_per_frame));
            }

            ui.separator();
            ui.label("Model Parameters");
            
            let mut params_lock = self.params.write().unwrap();
            for (name, constraint) in &self.param_metadata {
                if let Some(val) = params_lock.get_mut(name) {
                    let slider = egui::Slider::new(val, constraint.min..=constraint.max)
                        .step_by(constraint.step)
                        .text(name);
                    let mut resp = ui.add(slider);
                    if let Some(desc) = M::theory_description() {
                        resp = resp.accessible_hover_text(desc);
                    }
                    let _ = resp.changed(); // Just calling it to suppress unused warning if necessary, or not needed.
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(snapshot) = &self.last_snapshot {
                let image = ColorImage::new([snapshot.width, snapshot.height], snapshot.pixels.as_ref().clone());
                let texture = ctx.load_texture(M::name(), image, TextureOptions::NEAREST);
                self.texture = Some(texture);
            }

            if let Some(texture) = &self.texture {
                let width = texture.size()[0] as f32;
                let height = texture.size()[1] as f32;
                
                Plot::new(format!("{}_plot", M::name()))
                    .data_aspect(1.0)
                    .view_aspect(1.0)
                    .show_grid(false)
                    .show_axes([false, false])
                    .allow_drag(false)
                    .allow_zoom(false)
                    .allow_scroll(false)
                    .show(ui, |plot_ui| {
                        plot_ui.image(PlotImage::new(
                            format!("{}_image", M::name()),
                            texture.id(),
                            PlotPoint::new(width as f64 / 2.0, height as f64 / 2.0),
                            [width, height],
                        ));
                    });
            }
        });
    }
}
