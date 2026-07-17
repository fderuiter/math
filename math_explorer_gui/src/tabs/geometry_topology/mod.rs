// @explorer_feature = "pure_math"
use crate::tabs::ExplorerTab;
use eframe::egui;

pub mod curvature_heatmap;
pub mod export_utils;
pub mod surface_viewer;
pub mod vietoris_rips;

pub struct GeometryTopologyTab {
    framework: crate::framework::SimulationFramework,
}

impl Default for GeometryTopologyTab {
    fn default() -> Self {
        Self {
            framework: crate::framework::SimulationFramework::new("geometry_topology"),
        }
    }
}

impl ExplorerTab for GeometryTopologyTab {
    fn name(&self) -> &'static str {
        "Geometry & Topology"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.framework.show(ctx, "geometry_topology");
    }
}

// [cite:stat_mech]
