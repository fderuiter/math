use crate::async_sim::declarative::{DeclarativeSimulation, DeclarativeTab};
use crate::declare_params;
use crate::async_sim::StateSnapshot;
use crate::tabs::fluid_dynamics::FluidDynamicsTool;
use crate::tabs::ExplorerTab;
use eframe::egui;
use egui::Color32;
use math_explorer::physics::fluid_dynamics::lattice_boltzmann::{
    BgkCollision, LatticeBoltzmannD2Q9,
};
use std::sync::Arc;

declare_params! {
    pub struct LbmParams {
        #[param(name = "Viscosity", min = 0.005, max = 0.2)]
        pub viscosity: f64,
    }
}

pub struct LbmRunner {
    solver: LatticeBoltzmannD2Q9<BgkCollision>,
}

impl Default for LbmRunner {
    fn default() -> Self {
        let width = 100;
        let height = 50;
        let tau = 3.0 * 0.02 + 0.5;

        let mut solver = LatticeBoltzmannD2Q9::new(width, height, tau);
        solver.set_inlet(0, 20, 5, 10, 0.1, 0.0);

        let center_x = 30;
        let center_y = 25;
        let radius = 5;
        for y in 0..height {
            for x in 0..width {
                let dx = x as i32 - center_x;
                let dy = y as i32 - center_y;
                if dx * dx + dy * dy <= radius * radius {
                    solver.set_obstacle(x, y, true);
                }
            }
        }

        Self { solver }
    }
}

impl DeclarativeSimulation for LbmRunner {
    type Params = LbmParams;

    fn name(&self) -> &'static str {
        "Lattice Boltzmann (Demo)"
    }

    fn default_params(&self) -> Self::Params {
        LbmParams { viscosity: 0.02 }
    }

    fn param_descriptors(&self) -> Vec<crate::async_sim::declarative::ParamDescriptor<Self::Params>> {
        LbmParams::descriptors()
    }

    fn setup(&mut self, params: &Self::Params) {
        let _width = self.solver.width();
        let _height = self.solver.height();
        let tau = 3.0 * params.viscosity + 0.5;
        
        self.solver.collision_model.tau = tau;
    }

    fn step(&mut self, params: &Self::Params) {
        self.solver.collision_model.tau = 3.0 * params.viscosity + 0.5;
        self.solver.set_inlet(0, 0, 2, self.solver.height(), 0.1, 0.0);
        self.solver.step();
    }

    fn get_snapshot(&self) -> StateSnapshot {
        let width = self.solver.width();
        let height = self.solver.height();
        let mut pixels = vec![Color32::BLACK; width * height];

        for y in 0..height {
            for x in 0..width {
                let pixel_idx = y * width + x;
                if self.solver.is_obstacle(x, height - 1 - y) {
                    pixels[pixel_idx] = Color32::from_gray(50);
                } else {
                    let v = self.solver.get_velocity_magnitude(x, height - 1 - y);
                    let intensity = (v * 1000.0).clamp(0.0, 255.0) as u8;
                    pixels[pixel_idx] = Color32::from_rgb(
                        intensity,
                        (intensity as f32 * 0.5) as u8,
                        255 - intensity,
                    );
                }
            }
        }

        StateSnapshot {
            width,
            height,
            pixels: Arc::new(pixels),
            custom_data: Vec::new(),
            structured_data: None,
        }
    }
}

pub struct LatticeBoltzmannTool {
    inner: DeclarativeTab<LbmRunner>,
}

impl Default for LatticeBoltzmannTool {
    fn default() -> Self {
        Self {
            inner: DeclarativeTab::new(LbmRunner::default(), 5),
        }
    }
}

impl FluidDynamicsTool for LatticeBoltzmannTool {
    fn name(&self) -> &'static str {
        self.inner.name()
    }

    fn show(&mut self, ctx: &egui::Context) {
        self.inner.show_ctx(ctx);
    }
}
