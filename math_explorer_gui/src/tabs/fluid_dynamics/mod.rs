use crate::tabs::ExplorerTab;
use eframe::egui;

pub mod lattice_boltzmann;
pub mod potential_flow;
pub mod turbulence;

use lattice_boltzmann::LatticeBoltzmannTool;
use potential_flow::PotentialFlowTool;
use turbulence::TurbulenceTool;

#[derive(PartialEq)]
enum FluidMode {
    PotentialFlow,
    Turbulence,
    LatticeBoltzmann,
}

pub struct FluidDynamicsTab {
    mode: FluidMode,
    potential_flow: PotentialFlowTool,
    turbulence: TurbulenceTool,
    lattice_boltzmann: LatticeBoltzmannTool,
}

impl Default for FluidDynamicsTab {
    fn default() -> Self {
        Self {
            mode: FluidMode::PotentialFlow,
            potential_flow: PotentialFlowTool::default(),
            turbulence: TurbulenceTool::default(),
            lattice_boltzmann: LatticeBoltzmannTool::default(),
        }
    }
}

impl ExplorerTab for FluidDynamicsTab {
    fn name(&self) -> &'static str {
        "Fluid Dynamics"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("fluid_mode_selector").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Mode:");
                ui.selectable_value(&mut self.mode, FluidMode::PotentialFlow, "Potential Flow");
                ui.selectable_value(
                    &mut self.mode,
                    FluidMode::Turbulence,
                    "Turbulence / Reynolds Analysis",
                );
                ui.selectable_value(
                    &mut self.mode,
                    FluidMode::LatticeBoltzmann,
                    "Lattice Boltzmann (Demo)",
                );
            });
        });

        match self.mode {
            FluidMode::PotentialFlow => self.potential_flow.show(ctx),
            FluidMode::Turbulence => self.turbulence.show(ctx),
            FluidMode::LatticeBoltzmann => self.lattice_boltzmann.show(ctx),
        }
    }
}
