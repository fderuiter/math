use crate::tabs::ExplorerTab;
use eframe::egui;
use crate::framework::SimulationFramework;

pub mod band_structure;
pub mod crystal_viewer;
pub mod ising;


pub struct SolidStateTab {
    framework: crate::framework::SimulationFramework,
}

impl Default for SolidStateTab {
    fn default() -> Self {
        Self {
            framework: crate::framework::SimulationFramework::new(vec![

                Box::new(crystal_viewer::CrystalViewer::default()),
                Box::new(ising::IsingModelTool::default()),
                Box::new(band_structure::BandStructureTool::default()),
            
            ]),
        }
    }
}

impl ExplorerTab for SolidStateTab {
    fn name(&self) -> &'static str {
        "Solid State Physics"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.framework.show(ctx, "solid_state");
    }
}

// [cite:graph_parameters_rust]
