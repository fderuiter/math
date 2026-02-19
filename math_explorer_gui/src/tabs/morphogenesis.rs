use crate::tabs::ExplorerTab;
use eframe::egui;
use egui::ColorImage;
use math_explorer::biology::diffusion::FiniteDifference2D;
use math_explorer::biology::morphogenesis::{SchnakenbergKinetics, TuringSystem};

pub struct MorphogenesisTab {
    system: TuringSystem<SchnakenbergKinetics, FiniteDifference2D>,
    texture: Option<egui::TextureHandle>,
    // Simulation parameters
    a: f64,
    b: f64,
    d_u: f64,
    d_v: f64,
    dt: f64,
    paused: bool,
    width: usize,
    height: usize,
    simulation_speed: usize,
}

impl Default for MorphogenesisTab {
    fn default() -> Self {
        let width = 100;
        let height = 100;
        let d_u = 1.0;
        let d_v = 100.0; // Needs significant difference for Turing patterns
        let kinetics = SchnakenbergKinetics { a: 0.1, b: 0.9 }; // Classic spot/stripe params
        let diffusion = FiniteDifference2D::new(width, height, 1.0, 1.0);

        let mut system =
            TuringSystem::new_with_kinetics(width * height, d_u, d_v, kinetics, diffusion);

        // Initialize with noise
        initialize_system(&mut system, width, height);

        Self {
            system,
            texture: None,
            a: 0.1,
            b: 0.9,
            d_u,
            d_v,
            dt: 0.05,
            paused: false,
            width,
            height,
            simulation_speed: 10,
        }
    }
}

fn initialize_system(
    system: &mut TuringSystem<SchnakenbergKinetics, FiniteDifference2D>,
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

impl ExplorerTab for MorphogenesisTab {
    fn name(&self) -> &'static str {
        "Morphogenesis"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("morphogenesis_controls").show(ctx, |ui| {
            ui.heading("Turing Patterns");
            ui.label("Schnakenberg Kinetics");
            ui.separator();

            ui.collapsing("Parameters", |ui| {
                let mut changed = false;
                changed |= ui.add(egui::Slider::new(&mut self.a, 0.0..=1.0).text("a (Feed)")).changed();
                changed |= ui.add(egui::Slider::new(&mut self.b, 0.0..=2.0).text("b (Kill)")).changed();

                ui.separator();
                changed |= ui.add(egui::Slider::new(&mut self.d_u, 0.1..=5.0).text("D_u (Activator)")).changed();
                changed |= ui.add(egui::Slider::new(&mut self.d_v, 10.0..=200.0).text("D_v (Inhibitor)")).changed();

                if changed {
                    self.system.kinetics.a = self.a;
                    self.system.kinetics.b = self.b;
                    self.system.d_u = self.d_u;
                    self.system.d_v = self.d_v;
                }
            });

            ui.separator();
            ui.add(egui::Slider::new(&mut self.simulation_speed, 1..=50).text("Speed (steps/frame)"));

            if ui.button(if self.paused { "Resume" } else { "Pause" }).clicked() {
                self.paused = !self.paused;
            }

            if ui.button("Reset / Randomize").clicked() {
                 initialize_system(&mut self.system, self.width, self.height);
            }

            ui.separator();
            ui.label("Description:");
            ui.label("Turing patterns arise from the interaction of two diffusing substances: an activator and an inhibitor. The inhibitor must diffuse significantly faster than the activator.");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if !self.paused {
                for _ in 0..self.simulation_speed {
                    self.system.step(self.dt);
                }
                // Request repaint to keep animation going
                ui.ctx().request_repaint();
            }

            // Update texture
            let image = plot_concentration(self.system.u(), self.width, self.height);
            let texture = self.texture.get_or_insert_with(|| {
                ui.ctx()
                    .load_texture("morphogenesis_plot", image.clone(), Default::default())
            });
            texture.set(image, Default::default());

            // Draw
            ui.image((texture.id(), texture.size_vec2()));
        });
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
