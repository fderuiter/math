use crate::async_sim::unified::{UnifiedModel, UnifiedSimTool};
use crate::async_sim::{SimCommand, StateSnapshot};
use eframe::egui::ColorImage;
use math_commons::math_kernel::colormap::heatmap_color;
use math_commons::theory::{ParameterConstraint, TheoryDescribable};
use math_explorer::biology::diffusion::FiniteDifference2D;
use math_explorer::biology::morphogenesis::{SchnakenbergKinetics, TuringSystem};
use math_explorer::biology::reaction_diffusion::ReactionDiffusionModel;
use std::collections::HashMap;
use std::sync::Arc;

pub struct MorphogenesisUnified {
    system: TuringSystem<2, SchnakenbergKinetics, FiniteDifference2D>,
    dt: f64,
    width: usize,
    height: usize,
}

impl UnifiedModel for MorphogenesisUnified {
    fn new(params: &HashMap<String, f64>) -> Self {
        let width = 100;
        let height = 100;
        let a = *params.get("a").unwrap_or(&0.1);
        let b = *params.get("b").unwrap_or(&0.9);
        let d_u = *params.get("d_u").unwrap_or(&1.0);
        let d_v = *params.get("d_v").unwrap_or(&100.0);
        
        let kinetics = SchnakenbergKinetics { a, b };
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

        initialize_system(&mut system, width, height);

        Self {
            system,
            dt: 0.05,
            width,
            height,
        }
    }

    fn step(&mut self, params: &HashMap<String, f64>) {
        self.system.kinetics.a = *params.get("a").unwrap_or(&0.1);
        self.system.kinetics.b = *params.get("b").unwrap_or(&0.9);
        self.system.diffusion_coeffs[0] = *params.get("d_u").unwrap_or(&1.0);
        self.system.diffusion_coeffs[1] = *params.get("d_v").unwrap_or(&100.0);
        self.system.step(self.dt);
    }

    fn get_snapshot(&self) -> StateSnapshot {
        let image = plot_concentration(self.system.u(), self.width, self.height);
        let pixels = Arc::new(std::sync::RwLock::new(
            image
                .pixels
                .iter()
                .map(|c| eframe::egui::Color32::from_rgba_unmultiplied(c[0], c[1], c[2], c[3]))
                .collect()),
        );
        StateSnapshot {
            width: self.width,
            height: self.height,
            pixels,
            custom_data: Vec::new(),
            structured_data: None,
        }
    }

    fn process_command(&mut self, cmd: SimCommand, _params: &HashMap<String, f64>) {
        if let SimCommand::Reset = cmd {
            initialize_system(&mut self.system, self.width, self.height);
        } else if let SimCommand::Custom(action) = cmd {
            if action == "Reset Map" {
                initialize_system(&mut self.system, self.width, self.height);
            }
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

    fn presets() -> Vec<(&'static str, HashMap<String, f64>)> {
        let mut p = Vec::new();
        
        let mut spots = HashMap::new();
        spots.insert("a".to_string(), 0.1);
        spots.insert("b".to_string(), 0.9);
        spots.insert("d_u".to_string(), 1.0);
        spots.insert("d_v".to_string(), 100.0);
        p.push(("Spots", spots));

        let mut stripes = HashMap::new();
        stripes.insert("a".to_string(), 0.14);
        stripes.insert("b".to_string(), 0.86);
        stripes.insert("d_u".to_string(), 1.0);
        stripes.insert("d_v".to_string(), 50.0);
        p.push(("Stripes", stripes));

        p
    }

    fn custom_actions() -> Vec<&'static str> {
        vec!["Reset Map"]
    }

    fn name() -> &'static str {
        "Morphogenesis (Turing Patterns)"
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
        let rgb = heatmap_color(norm);
        pixels.push(rgb[0]);
        pixels.push(rgb[1]);
        pixels.push(rgb[2]);
        pixels.push(255);
    }

    ColorImage::from_rgba_unmultiplied([width, height], &pixels)
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

inventory::submit! {
    crate::framework::ToolMetadata {
        name: "MorphogenesisTool",
        domain: "morphogenesis",
        tags: &[],
        build: || Box::new(UnifiedSimTool::<MorphogenesisUnified>::new()),
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
