use crate::async_sim::declarative::{DeclarativeSimulation, ParamDescriptor, ParamType, DeclarativeTab};
use crate::declare_params;
use crate::async_sim::StateSnapshot;
use crate::tabs::ExplorerTab;
use eframe::egui;
use egui::ColorImage;
use math_explorer::biology::diffusion::FiniteDifference2D;
use math_explorer::biology::morphogenesis::{SchnakenbergKinetics, TuringSystem};
use std::sync::Arc;

declare_params! {
    pub struct MorphogenesisParams {
        #[param(name = "a (Feed)", min = 0.0, max = 1.0)]
        pub a: f64,
        #[param(name = "b (Kill)", min = 0.0, max = 2.0)]
        pub b: f64,
        #[param(name = "D_u (Activator)", min = 0.1, max = 5.0)]
        pub d_u: f64,
        #[param(name = "D_v (Inhibitor)", min = 10.0, max = 200.0)]
        pub d_v: f64,
        #[param(name = "dt", min = 0.01, max = 0.1)]
        pub dt: f64,
    }
}

pub struct MorphogenesisSim {
    system: TuringSystem<2, SchnakenbergKinetics, FiniteDifference2D>,
    width: usize,
    height: usize,
}

impl Default for MorphogenesisSim {
    fn default() -> Self {
        let width = 100;
        let height = 100;
        
        let kinetics = SchnakenbergKinetics { a: 0.1, b: 0.9 };
        let diffusion = FiniteDifference2D::new(
            math_explorer::math_kernel::types::Dimension(width),
            math_explorer::math_kernel::types::Dimension(height),
            math_explorer::math_kernel::types::StepSize(1.0),
            math_explorer::math_kernel::types::StepSize(1.0)
        );

        let system = TuringSystem::new_with_kinetics(
            math_explorer::math_kernel::types::Dimension(width * height),
            math_explorer::biology::morphogenesis::DiffusionCoeff(1.0),
            math_explorer::biology::morphogenesis::DiffusionCoeff(100.0),
            kinetics,
            diffusion
        );

        Self {
            system,
            width,
            height,
        }
    }
}

impl DeclarativeSimulation for MorphogenesisSim {
    type Params = MorphogenesisParams;

    fn name(&self) -> &'static str {
        "Morphogenesis"
    }

    fn description(&self) -> &'static str {
        "Turing patterns arise from the interaction of two diffusing substances: an activator and an inhibitor. The inhibitor must diffuse significantly faster than the activator."
    }

    fn default_params(&self) -> Self::Params {
        MorphogenesisParams {
            a: 0.1,
            b: 0.9,
            d_u: 1.0,
            d_v: 100.0,
            dt: 0.05,
        }
    }

    fn param_descriptors(&self) -> Vec<ParamDescriptor<Self::Params>> {
        MorphogenesisParams::descriptors()
    }

    fn setup(&mut self, params: &Self::Params) {
        self.system.kinetics.a = params.a;
        self.system.kinetics.b = params.b;
        self.system.diffusion_coeffs[0] = params.d_u;
        self.system.diffusion_coeffs[1] = params.d_v;
        initialize_system(&mut self.system, self.width, self.height);
    }

    fn step(&mut self, params: &Self::Params) {
        // We sync params every step in case they changed during execution
        self.system.kinetics.a = params.a;
        self.system.kinetics.b = params.b;
        self.system.diffusion_coeffs[0] = params.d_u;
        self.system.diffusion_coeffs[1] = params.d_v;
        
        self.system.step(params.dt);
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
}

pub struct MorphogenesisTab {
    inner: DeclarativeTab<MorphogenesisSim>,
}

impl Default for MorphogenesisTab {
    fn default() -> Self {
        Self {
            inner: DeclarativeTab::new(MorphogenesisSim::default(), 10),
        }
    }
}

impl ExplorerTab for MorphogenesisTab {
    fn name(&self) -> &'static str {
        self.inner.name()
    }

    fn show(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.inner.show(ctx, frame);
    }
}

fn initialize_system(
    system: &mut TuringSystem<2, SchnakenbergKinetics, FiniteDifference2D>,
    width: usize,
    height: usize,
) {
    let n = width * height;
    let mut rng = SimpleRng::new(12345);

    let a = system.kinetics.a;
    let b = system.kinetics.b;
    let u_eq = a + b;
    let v_eq = b / (u_eq * u_eq);

    for i in 0..n {
        system.u_mut()[i] = u_eq + rng.range(-0.1, 0.1);
        system.v_mut()[i] = v_eq + rng.range(-0.1, 0.1);
    }
}

fn plot_concentration(data: &[f64], width: usize, height: usize) -> ColorImage {
    let mut pixels = Vec::with_capacity(width * height * 4);

    let (min, max) = data
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &val| {
            (min.min(val), max.max(val))
        });

    let range = max - min;
    let inv_range = if range > 1e-6 { 1.0 / range } else { 1.0 };

    for &val in data {
        let norm = (val - min) * inv_range;
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
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let x = self.state;
        let x = x ^ x >> 18;
        let rot = (x >> 27) as u32;
        let val = (x as u32).rotate_right(rot);
        (val as f64) / (u32::MAX as f64)
    }

    fn range(&mut self, min: f64, max: f64) -> f64 {
        min + self.next_f64() * (max - min)
    }
}
// [cite:cannibalism] [cite:favorite_child] [cite:self_calibration_paper]
