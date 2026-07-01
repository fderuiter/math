// @explorer_feature = "biology"
use crate::accessibility::AccessibleHoverText;
use crate::async_sim::{SimCommand, SimulationController, SimulationRunner, StateSnapshot};
use crate::tabs::ExplorerTab;
use eframe::egui;
use egui::ColorImage;
use math_commons::theory::TheoryDescribable;
use math_explorer::biology::diffusion::FiniteDifference2D;
use math_explorer::biology::morphogenesis::{SchnakenbergKinetics, TuringSystem};
use math_explorer::biology::reaction_diffusion::ReactionDiffusionModel;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

pub struct MorphogenesisParams {
    pub a: AtomicU64,
    pub b: AtomicU64,
    pub d_u: AtomicU64,
    pub d_v: AtomicU64,
}

impl MorphogenesisParams {
    pub fn new(a: f64, b: f64, d_u: f64, d_v: f64) -> Self {
        Self {
            a: AtomicU64::new(a.to_bits()),
            b: AtomicU64::new(b.to_bits()),
            d_u: AtomicU64::new(d_u.to_bits()),
            d_v: AtomicU64::new(d_v.to_bits()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PatternPreset {
    Spots,
    Stripes,
    Labyrinths,
    Chaos,
}

impl PatternPreset {
    fn label(&self) -> &'static str {
        match self {
            Self::Spots => "Spots (Leopard)",
            Self::Stripes => "Stripes (Zebra)",
            Self::Labyrinths => "Labyrinths",
            Self::Chaos => "Unstable / Chaos",
        }
    }

    fn params(&self) -> (f64, f64, f64, f64) {
        // Returns (a, b, d_u, d_v)
        // Values tuned for typical Schnakenberg patterns
        match self {
            Self::Spots => (0.12, 0.88, 1.0, 100.0),
            Self::Stripes => (0.1, 0.9, 1.0, 100.0),
            Self::Labyrinths => (0.14, 0.86, 1.0, 100.0),
            Self::Chaos => (0.02, 0.98, 1.0, 100.0),
        }
    }
}

struct MorphogenesisRunner {
    system: TuringSystem<2, SchnakenbergKinetics, FiniteDifference2D>,
    dt: f64,
    width: usize,
    height: usize,
    steps_per_frame: usize,
    params: Arc<MorphogenesisParams>,
}

impl SimulationRunner for MorphogenesisRunner {
    fn process_command(&mut self, cmd: SimCommand) {
        match cmd {
            SimCommand::SetSpeed(speed) => self.steps_per_frame = speed,
            SimCommand::Reset => initialize_system(&mut self.system, self.width, self.height),
            _ => {}
        }
    }

    fn step(&mut self) {
        self.system.kinetics.a = f64::from_bits(self.params.a.load(Ordering::Relaxed));
        self.system.kinetics.b = f64::from_bits(self.params.b.load(Ordering::Relaxed));
        self.system.diffusion_coeffs[0] = f64::from_bits(self.params.d_u.load(Ordering::Relaxed));
        self.system.diffusion_coeffs[1] = f64::from_bits(self.params.d_v.load(Ordering::Relaxed));
        
        self.system.step(self.dt);
    }

    fn get_snapshot(&self) -> StateSnapshot {
        let image = plot_concentration(self.system.u(), self.width, self.height);
        let pixels = Arc::new(
            image
                .pixels
                .iter()
                .map(|c| eframe::egui::Color32::from_rgba_unmultiplied(c[0], c[1], c[2], c[3]))
                .collect(),
        );
        StateSnapshot {
            width: self.width,
            height: self.height,
            pixels,
            custom_data: Vec::new(),
            structured_data: None,
        }
    }

    fn get_steps_per_frame(&self) -> usize {
        self.steps_per_frame
    }
}

#[allow(dead_code)]
pub struct MorphogenesisTool {
    controller: SimulationController,
    texture: Option<egui::TextureHandle>,
    // Simulation parameters
    a: f64,
    b: f64,
    d_u: f64,
    d_v: f64,
    params: Arc<MorphogenesisParams>,
    dt: f64,
    width: usize,
    height: usize,
    simulation_speed: usize,
    selected_preset: Option<PatternPreset>,
}

impl Default for MorphogenesisTool {
    fn default() -> Self {
        let width = 100;
        let height = 100;
        let a = 0.1;
        let b = 0.9;
        let d_u = 1.0;
        let d_v = 100.0; // Needs significant difference for Turing patterns
        let kinetics = SchnakenbergKinetics { a, b }; // Classic spot/stripe params
        let diffusion = FiniteDifference2D::new(
            math_explorer::math_kernel::types::Dimension(width),
            math_explorer::math_kernel::types::Dimension(height),
            math_explorer::math_kernel::types::StepSize(1.0),
            math_explorer::math_kernel::types::StepSize(1.0),
        );

        let mut system = TuringSystem::new_with_kinetics(
            math_explorer::math_kernel::types::Dimension(width * height),
            math_explorer::biology::morphogenesis::DiffusionCoeff(d_u),
            math_explorer::biology::morphogenesis::DiffusionCoeff(d_v),
            kinetics,
            diffusion,
        );

        // Initialize with noise
        initialize_system(&mut system, width, height);

        let params = Arc::new(MorphogenesisParams::new(a, b, d_u, d_v));

        let runner = MorphogenesisRunner {
            system,
            dt: 0.05,
            width,
            height,
            steps_per_frame: 10,
            params: Arc::clone(&params),
        };
        let controller = SimulationController::new(runner);

        Self {
            controller,
            texture: None,
            a,
            b,
            d_u,
            d_v,
            params,
            dt: 0.05,
            width,
            height,
            simulation_speed: 10,
            selected_preset: None,
        }
    }
}

fn initialize_system(
    system: &mut TuringSystem<2, SchnakenbergKinetics, FiniteDifference2D>,
    width: usize,
    height: usize,
) {
    let n = width * height;
    let mut rng = SimpleRng::new(12345);

    // Schnakenberg equilibrium: u = a + b, v = b / (a+b)^2
    // We add noise around this equilibrium
    let a = system.kinetics.a;
    let b = system.kinetics.b;
    let u_eq = a + b;
    let v_eq = b / (u_eq * u_eq);

    for i in 0..n {
        system.u_mut()[i] = u_eq + rng.range(-0.1, 0.1);
        system.v_mut()[i] = v_eq + rng.range(-0.1, 0.1);
    }
}

pub struct MorphogenesisTab {
    framework: crate::framework::SimulationFramework,
}

impl Default for MorphogenesisTab {
    fn default() -> Self {
        Self {
            framework: crate::framework::SimulationFramework::new(vec![Box::new(MorphogenesisTool::default())]),
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

    fn theory(&self) -> Option<&dyn TheoryDescribable> {
        Some(self)
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
                        let (a, b, d_u, d_v) = preset.params();
                        self.a = a;
                        self.b = b;
                        self.d_u = d_u;
                        self.d_v = d_v;

                        self.params.a.store(a.to_bits(), Ordering::Relaxed);
                        self.params.b.store(b.to_bits(), Ordering::Relaxed);
                        self.params.d_u.store(d_u.to_bits(), Ordering::Relaxed);
                        self.params.d_v.store(d_v.to_bits(), Ordering::Relaxed);

                        self.controller.send_command(SimCommand::Reset);
                        self.selected_preset = Some(preset);
                    }
                }
            });
            ui.separator();

            ui.collapsing("Parameters", |ui| {
                let mut changed = false;
                let dummy_kinetics = SchnakenbergKinetics { a: 1.0, b: 1.0 };

                let a_res = crate::reflective_ui::render_theory_parameter(
                    ui,
                    &dummy_kinetics,
                    "a",
                    "a (Feed)",
                    &mut self.a,
                );
                changed |= a_res.changed();

                let b_res = crate::reflective_ui::render_theory_parameter(
                    ui,
                    &dummy_kinetics,
                    "b",
                    "b (Kill)",
                    &mut self.b,
                );
                changed |= b_res.changed();

                ui.separator();
                changed |= ui.add(egui::Slider::new(&mut self.d_u, 0.1..=5.0).text("D_u (Activator)")).changed();
                changed |= ui.add(egui::Slider::new(&mut self.d_v, 10.0..=200.0).text("D_v (Inhibitor)")).changed();

                if changed {
                    self.params.a.store(self.a.to_bits(), Ordering::Relaxed);
                    self.params.b.store(self.b.to_bits(), Ordering::Relaxed);
                    self.params.d_u.store(self.d_u.to_bits(), Ordering::Relaxed);
                    self.params.d_v.store(self.d_v.to_bits(), Ordering::Relaxed);
                    self.selected_preset = None;
                }
            });

            ui.separator();
            if ui.add(egui::Slider::new(&mut self.simulation_speed, 1..=50).text("Speed (steps/frame)")).changed() {
                self.controller.send_command(SimCommand::SetSpeed(self.simulation_speed));
            }

            let pause_btn = ui.button(if !self.controller.running { "▶ Resume" } else { "⏸ Pause" });
            if pause_btn.accessible_hover_text(if !self.controller.running { "Resume the Turing pattern simulation" } else { "Pause the Turing pattern simulation" }).clicked() {
                if self.controller.running {
                    self.controller.send_command(SimCommand::Pause);
                } else {
                    self.controller.send_command(SimCommand::Start);
                }
            }

            if ui.button("↻ Reset / Randomize").accessible_hover_text("Re-initialize the simulation grid with random noise").clicked() {
                 self.controller.send_command(SimCommand::Reset);
            }

            ui.separator();
            ui.label("Description:");
            ui.label("Turing patterns arise from the interaction of two diffusing substances: an activator and an inhibitor. The inhibitor must diffuse significantly faster than the activator.");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.controller.running {
                // Request repaint to keep animation going
                ui.ctx().request_repaint();
            }

            // Update texture
            if let Some(snapshot) = self.controller.update() {
                let mut image = ColorImage::default();
                image.size = [snapshot.width, snapshot.height];
                image.pixels = snapshot.pixels.as_ref().clone();
                let texture = self.texture.get_or_insert_with(|| {
                    ui.ctx()
                        .load_texture("morphogenesis_plot", image.clone(), Default::default())
                });
                texture.set(image, Default::default());
            }

            // Draw
            if let Some(texture) = &self.texture {
                ui.image((texture.id(), texture.size_vec2()));
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

fn plot_concentration(data: &[f64], width: usize, height: usize) -> ColorImage {
    let mut pixels = Vec::with_capacity(width * height * 4);

    // Find range for auto-scaling
    let (min, max) = data
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &val| {
            (min.min(val), max.max(val))
        });

    let range = max - min;
    let inv_range = if range > 1e-6 { 1.0 / range } else { 1.0 };

    for &val in data {
        let norm = (val - min) * inv_range;
        // Simple heatmap: Blue -> Cyan -> Green -> Yellow -> Red
        let (r, g, b) = heatmap_color(norm);
        pixels.push(r);
        pixels.push(g);
        pixels.push(b);
        pixels.push(255);
    }

    ColorImage::from_rgba_unmultiplied([width, height], &pixels)
}

fn heatmap_color(t: f64) -> (u8, u8, u8) {
    let t = t.clamp(0.0, 1.0);
    // Plasma-like colormap approximation
    // t=0: Dark Blue (0,0,128)
    // t=0.5: Red (255,0,0)
    // t=1: Yellow (255,255,0)

    if t < 0.5 {
        let t2 = t * 2.0;
        let r = (255.0 * t2) as u8;
        let g = 0;
        let b = (128.0 * (1.0 - t2)) as u8;
        (r, g, b)
    } else {
        let t2 = (t - 0.5) * 2.0;
        let r = 255;
        let g = (255.0 * t2) as u8;
        let b = 0;
        (r, g, b)
    }
}

/// A lightweight, deterministic pseudo-random number generator.
///
/// **Architectural Decision:**
/// We implement this simple Linear Congruential Generator (LCG) / PCG variant locally to avoid
/// pulling in the heavy `rand` crate tree for the GUI. The GUI only needs noise for visualization
/// initialization, not cryptographic security or statistical rigor (which should be handled in `math_explorer`).
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_f64(&mut self) -> f64 {
        // Simple LCG
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let x = self.state;
        // let count = (x >> 59) as u32;
        let x = x ^ x >> 18;
        let rot = (x >> 27) as u32;
        let val = (x as u32).rotate_right(rot);
        (val as f64) / (u32::MAX as f64)
    }

    fn range(&mut self, min: f64, max: f64) -> f64 {
        min + self.next_f64() * (max - min)
    }
}

// [cite:graph_parameters_rust]
