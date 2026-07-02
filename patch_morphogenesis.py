import re

with open('math_explorer_gui/src/tabs/morphogenesis.rs', 'r') as f:
    content = f.read()

# Fix conflict 1
conflict1 = """<<<<<<< HEAD
    fn process_command(&mut self, cmd: SimCommand, _params: &HashMap<String, f64>) {
        if let SimCommand::Reset = cmd {
            initialize_system(&mut self.system, self.width, self.height);
        }
    }

    fn parameters() -> HashMap<String, ParameterConstraint>
    where
        Self: Sized,
    {
        let mut map = HashMap::new();
        map.insert(
            "a".to_string(),
            ParameterConstraint {
                min: 0.0,
                max: 1.0,
                step: 0.01,
            },
        );
        map.insert(
            "b".to_string(),
            ParameterConstraint {
                min: 0.0,
                max: 2.0,
                step: 0.01,
            },
        );
        map.insert(
            "d_u".to_string(),
            ParameterConstraint {
                min: 0.1,
                max: 5.0,
                step: 0.1,
            },
        );
        map.insert(
            "d_v".to_string(),
            ParameterConstraint {
                min: 10.0,
                max: 200.0,
                step: 1.0,
            },
        );
        map
    }

    fn name() -> &'static str
    where
        Self: Sized,
    {
        "Morphogenesis (Turing Patterns)"
    }

    fn create_theory() -> Option<Box<dyn math_commons::theory::TheoryDescribable>>
    where
        Self: Sized,
    {
        Some(Box::new(ReactionDiffusionModel::<
            SchnakenbergKinetics,
            FiniteDifference2D,
        > {
            reaction: SchnakenbergKinetics { a: 1.0, b: 1.0 },
            diffusion: FiniteDifference2D::new(
                math_explorer::math_kernel::types::Dimension(1),
                math_explorer::math_kernel::types::Dimension(1),
                math_explorer::math_kernel::types::StepSize(1.0),
                math_explorer::math_kernel::types::StepSize(1.0),
            ),
            diffusion_coeffs: vec![],
        }))
    }
}
=======
    fn get_steps_per_frame(&self) -> usize {
        self.steps_per_frame
    }
}

#[allow(dead_code)]
pub struct MorphogenesisTool {
    controller: SimulationController,
    texture: Option<egui::TextureHandle>,
    // Simulation parameters
    params: Arc<RwLock<MorphogenesisConfig>>,
    dt: f64,
    width: usize,
    height: usize,
    simulation_speed: usize,
    selected_preset: Option<PatternPreset>,
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq)]
enum PatternPreset {
    Spots,
    Stripes,
    Labyrinths,
    Chaos,
}

impl PatternPreset {
    fn label(&self) -> &'static str {
        match self {
            Self::Spots => "Spots",
            Self::Stripes => "Stripes",
            Self::Labyrinths => "Labyrinths",
            Self::Chaos => "Chaos",
        }
    }
    
    fn config(&self) -> MorphogenesisConfig {
        match self {
            Self::Spots => MorphogenesisConfig {
                a: 0.1,
                b: 0.9,
                d_u: 1.0,
                d_v: 100.0,
            },
            Self::Stripes => MorphogenesisConfig {
                a: 0.14,
                b: 0.86,
                d_u: 1.0,
                d_v: 60.0,
            },
            Self::Labyrinths => MorphogenesisConfig {
                a: 0.1,
                b: 0.8,
                d_u: 1.0,
                d_v: 80.0,
            },
            Self::Chaos => MorphogenesisConfig {
                a: 0.05,
                b: 0.95,
                d_u: 1.0,
                d_v: 150.0,
            },
        }
    }
}

impl Default for MorphogenesisTool {
    fn default() -> Self {
        Self::new()
    }
}

impl MorphogenesisTool {
    pub fn new() -> Self {
        let width = 100;
        let height = 100;
        let initial_config = MorphogenesisConfig {
            a: 0.1,
            b: 0.9,
            d_u: 1.0,
            d_v: 100.0,
        };

        let runner = MorphogenesisRunner::new(width, height, initial_config.clone());
        let controller = SimulationController::new(runner);

        Self {
            controller,
            texture: None,
            params: Arc::new(RwLock::new(initial_config)),
            dt: 0.05,
            width,
            height,
            simulation_speed: 5,
            selected_preset: Some(PatternPreset::Spots),
        }
    }

    fn parameters() -> HashMap<String, ParameterConstraint> {
        let mut map = HashMap::new();
        map.insert("a".to_string(), ParameterConstraint { min: 0.0, max: 1.0, step: 0.01 });
        map.insert("b".to_string(), ParameterConstraint { min: 0.0, max: 2.0, step: 0.01 });
        map.insert("d_u".to_string(), ParameterConstraint { min: 0.1, max: 5.0, step: 0.1 });
        map.insert("d_v".to_string(), ParameterConstraint { min: 10.0, max: 200.0, step: 1.0 });
        map
    }

    fn name() -> &'static str {
        "Morphogenesis (Turing Patterns)"
    }

    fn create_theory() -> Option<Box<dyn TheoryDescribable>> {
        Some(Box::new(ReactionDiffusionModel::<SchnakenbergKinetics, FiniteDifference2D> {
            reaction: SchnakenbergKinetics { a: 1.0, b: 1.0 },
            diffusion: FiniteDifference2D::new(
                math_explorer::math_kernel::types::Dimension(1),
                math_explorer::math_kernel::types::Dimension(1),
                math_explorer::math_kernel::types::StepSize(1.0),
                math_explorer::math_kernel::types::StepSize(1.0),
            ),
            diffusion_coeffs: vec![],
        }))
    }
}
"""

resolved1 = """    fn process_command(&mut self, cmd: SimCommand, _params: &HashMap<String, f64>) {
        if let SimCommand::Reset = cmd {
            initialize_system(&mut self.system, self.width, self.height);
        }
    }

    fn parameters() -> HashMap<String, ParameterConstraint>
    where
        Self: Sized,
    {
        let mut map = HashMap::new();
        map.insert(
            "a".to_string(),
            ParameterConstraint {
                min: 0.0,
                max: 1.0,
                step: 0.01,
            },
        );
        map.insert(
            "b".to_string(),
            ParameterConstraint {
                min: 0.0,
                max: 2.0,
                step: 0.01,
            },
        );
        map.insert(
            "d_u".to_string(),
            ParameterConstraint {
                min: 0.1,
                max: 5.0,
                step: 0.1,
            },
        );
        map.insert(
            "d_v".to_string(),
            ParameterConstraint {
                min: 10.0,
                max: 200.0,
                step: 1.0,
            },
        );
        map
    }

    fn name() -> &'static str
    where
        Self: Sized,
    {
        "Morphogenesis (Turing Patterns)"
    }
}

impl TheoryDescribable for MorphogenesisUnified {
    fn theory_description(&self) -> String {
        ReactionDiffusionModel::<SchnakenbergKinetics, FiniteDifference2D> {
            reaction: SchnakenbergKinetics { a: 1.0, b: 1.0 },
            diffusion: FiniteDifference2D::new(
                math_explorer::math_kernel::types::Dimension(1),
                math_explorer::math_kernel::types::Dimension(1),
                math_explorer::math_kernel::types::StepSize(1.0),
                math_explorer::math_kernel::types::StepSize(1.0),
            ),
            diffusion_coeffs: vec![],
        }.theory_description()
    }
    
    fn phonetic_description(&self) -> String {
        ReactionDiffusionModel::<SchnakenbergKinetics, FiniteDifference2D> {
            reaction: SchnakenbergKinetics { a: 1.0, b: 1.0 },
            diffusion: FiniteDifference2D::new(
                math_explorer::math_kernel::types::Dimension(1),
                math_explorer::math_kernel::types::Dimension(1),
                math_explorer::math_kernel::types::StepSize(1.0),
                math_explorer::math_kernel::types::StepSize(1.0),
            ),
            diffusion_coeffs: vec![],
        }.phonetic_description()
    }
    
    fn theory_citation(&self) -> String {
        ReactionDiffusionModel::<SchnakenbergKinetics, FiniteDifference2D> {
            reaction: SchnakenbergKinetics { a: 1.0, b: 1.0 },
            diffusion: FiniteDifference2D::new(
                math_explorer::math_kernel::types::Dimension(1),
                math_explorer::math_kernel::types::Dimension(1),
                math_explorer::math_kernel::types::StepSize(1.0),
                math_explorer::math_kernel::types::StepSize(1.0),
            ),
            diffusion_coeffs: vec![],
        }.theory_citation()
    }
    
    fn available_descriptions(&self) -> std::collections::HashMap<String, String> {
        ReactionDiffusionModel::<SchnakenbergKinetics, FiniteDifference2D> {
            reaction: SchnakenbergKinetics { a: 1.0, b: 1.0 },
            diffusion: FiniteDifference2D::new(
                math_explorer::math_kernel::types::Dimension(1),
                math_explorer::math_kernel::types::Dimension(1),
                math_explorer::math_kernel::types::StepSize(1.0),
                math_explorer::math_kernel::types::StepSize(1.0),
            ),
            diffusion_coeffs: vec![],
        }.available_descriptions()
    }
}
"""
content = content.replace(conflict1, resolved1)

conflict2 = """<<<<<<< HEAD
=======
pub struct MorphogenesisTab {
    framework: crate::framework::SimulationFramework,
}

impl Default for MorphogenesisTab {
    fn default() -> Self {
        Self {
            framework: crate::framework::SimulationFramework::new("morphogenesis"),
        }
    }
}

impl ExplorerTab for MorphogenesisTab {
    fn name(&self) -> &'static str {
        "Morphogenesis"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.framework.show(ctx, "morphogenesis");
    }
}

impl crate::framework::InteractiveTool for MorphogenesisTool {
    fn name(&self) -> &'static str {
        "Morphogenesis (Turing Patterns)"
    }

    fn theory(&self) -> &dyn TheoryDescribable {
        self
    }

    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    #[allow(clippy::field_reassign_with_default)]
    fn show(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("morphogenesis_controls").show(ctx, |ui| {
            ui.heading("Turing Patterns");
            ui.label("Schnakenberg Kinetics");
            ui.separator();

            ui.heading("Pattern Gallery");
            ui.horizontal_wrapped(|ui| {
                for preset in [
                    PatternPreset::Spots,
                    PatternPreset::Stripes,
                    PatternPreset::Labyrinths,
                    PatternPreset::Chaos,
                ] {
                    let selected = self.selected_preset == Some(preset);
                    if ui.selectable_label(selected, preset.label()).clicked() {
                        self.selected_preset = Some(preset);
                        let config = preset.config();
                        *self.params.write().unwrap() = config;
                        self.controller.send_command(SimCommand::Reset);
                    }
                }
            });

            ui.separator();
            ui.heading("Parameters");

            let is_running = self.controller.running;
            
            if ui.button(if is_running { "⏸ Pause" } else { "▶ Run" }).clicked() {
                if is_running {
                    self.controller.send_command(SimCommand::Pause);
                } else {
                    self.controller.send_command(SimCommand::Start);
                }
            }
            
            if ui.button("↻ Reset").clicked() {
                self.controller.send_command(SimCommand::Reset);
            }
            
            ui.separator();
            if ui.add(egui::Slider::new(&mut self.simulation_speed, 1..=50).text("Speed")).changed() {
                self.controller.send_command(SimCommand::SetSpeed(self.simulation_speed));
            }
            ui.separator();

            let mut config = self.params.write().unwrap();
            let mut changed = false;

            let descs = self.available_descriptions();
            
            let mut add_slider = |val: &mut f64, range: std::ops::RangeInclusive<f64>, text: &str| {
                let mut resp = ui.add(egui::Slider::new(val, range).text(text));
                if let Some(desc) = descs.get(text) {
                    resp = resp.accessible_hover_text(desc);
                }
                changed |= resp.changed();
            };

            add_slider(&mut config.a, 0.0..=1.0, "a");
            add_slider(&mut config.b, 0.0..=2.0, "b");
            add_slider(&mut config.d_u, 0.1..=5.0, "d_u");
            add_slider(&mut config.d_v, 10.0..=200.0, "d_v");

            if changed {
                self.selected_preset = None;
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let mut has_image = false;
            
            if let Some(snapshot) = self.controller.update() {
                if let Ok(guard) = snapshot.pixels.try_read() {
                    if !guard.is_empty() {
                        let image = ColorImage::new([snapshot.width, snapshot.height], guard.clone());
                        let texture = ctx.load_texture("morphogenesis", image, egui::TextureOptions::NEAREST);
                        self.texture = Some(texture);
                        has_image = true;
                    }
                }
                ctx.request_repaint();
            } else if self.controller.running {
                ctx.request_repaint();
            }

            if has_image || self.texture.is_some() {
                if let Some(texture) = &self.texture {
                    let width = texture.size()[0] as f32;
                    let height = texture.size()[1] as f32;
                    
                    eframe::egui_plot::Plot::new("morphogenesis_plot")
                        .data_aspect(1.0)
                        .view_aspect(1.0)
                        .show_grid(false)
                        .show_axes([false, false])
                        .show(ui, |plot_ui| {
                            plot_ui.image(eframe::egui_plot::PlotImage::new(
                                texture.id(),
                                eframe::egui_plot::PlotPoint::new(width as f64 / 2.0, height as f64 / 2.0),
                                [width, height]
                            ));
                        });
                }
            }
        });
    }
}

impl TheoryDescribable for MorphogenesisTool {
    fn theory_description(&self) -> String {
        ReactionDiffusionModel::<SchnakenbergKinetics, FiniteDifference2D> {
            reaction: SchnakenbergKinetics { a: 1.0, b: 1.0 },
            diffusion: FiniteDifference2D::new(
                math_explorer::math_kernel::types::Dimension(1),
                math_explorer::math_kernel::types::Dimension(1),
                math_explorer::math_kernel::types::StepSize(1.0),
                math_explorer::math_kernel::types::StepSize(1.0),
            ),
            diffusion_coeffs: vec![],
        }.theory_description()
    }
    
    fn phonetic_description(&self) -> String {
        ReactionDiffusionModel::<SchnakenbergKinetics, FiniteDifference2D> {
            reaction: SchnakenbergKinetics { a: 1.0, b: 1.0 },
            diffusion: FiniteDifference2D::new(
                math_explorer::math_kernel::types::Dimension(1),
                math_explorer::math_kernel::types::Dimension(1),
                math_explorer::math_kernel::types::StepSize(1.0),
                math_explorer::math_kernel::types::StepSize(1.0),
            ),
            diffusion_coeffs: vec![],
        }.phonetic_description()
    }
    
    fn theory_citation(&self) -> String {
        ReactionDiffusionModel::<SchnakenbergKinetics, FiniteDifference2D> {
            reaction: SchnakenbergKinetics { a: 1.0, b: 1.0 },
            diffusion: FiniteDifference2D::new(
                math_explorer::math_kernel::types::Dimension(1),
                math_explorer::math_kernel::types::Dimension(1),
                math_explorer::math_kernel::types::StepSize(1.0),
                math_explorer::math_kernel::types::StepSize(1.0),
            ),
            diffusion_coeffs: vec![],
        }.theory_citation()
    }
    fn available_descriptions(&self) -> std::collections::HashMap<String, String> {
        ReactionDiffusionModel::<SchnakenbergKinetics, FiniteDifference2D> {
            reaction: SchnakenbergKinetics { a: 1.0, b: 1.0 },
            diffusion: FiniteDifference2D::new(
                math_explorer::math_kernel::types::Dimension(1),
                math_explorer::math_kernel::types::Dimension(1),
                math_explorer::math_kernel::types::StepSize(1.0),
                math_explorer::math_kernel::types::StepSize(1.0),
            ),
            diffusion_coeffs: vec![],
        }.available_descriptions()
    }
}

>>>>>>> origin/main"""

resolved2 = ""
content = content.replace(conflict2, resolved2)

with open('math_explorer_gui/src/tabs/morphogenesis.rs', 'w') as f:
    f.write(content)
