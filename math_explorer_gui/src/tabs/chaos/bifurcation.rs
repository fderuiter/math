use crate::framework::InteractiveTool;
use crate::accessibility::AccessibleHoverText;
use eframe::egui;
use egui_plot::{Plot, PlotPoints, Points};
use math_explorer::physics::chaos::logistic;

pub struct BifurcationDiagram {
    r_min: f64,
    r_max: f64,
    steps: usize,
    points: Vec<[f64; 2]>,
}

impl Default for BifurcationDiagram {
    fn default() -> Self {
        let mut tool = Self {
            r_min: 2.5,
            r_max: 4.0,
            steps: 500,
            points: Vec::new(),
        };
        tool.recompute();
        tool
    }
}

impl BifurcationDiagram {
    fn recompute(&mut self) {
        let raw_points = logistic::generate_bifurcation_diagram(self.r_min, self.r_max, self.steps);
        self.points = raw_points.into_iter().map(|(r, x)| [r, x]).collect();
    }
}

impl InteractiveTool for BifurcationDiagram {
    fn name(&self) -> &'static str {
        "Bifurcation Diagram"
    }

    fn show(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("bifurcation_controls").show(ctx, |ui| {
            ui.heading("Logistic Map");
            ui.separator();

            let mut changed = false;
            ui.label("Min r");
            changed |= ui
                .add(egui::Slider::new(&mut self.r_min, 0.0..=4.0))
                .changed();

            ui.label("Max r");
            changed |= ui
                .add(egui::Slider::new(&mut self.r_max, self.r_min..=4.0))
                .changed();

            ui.label("Resolution (steps)");
            changed |= ui
                .add(egui::Slider::new(&mut self.steps, 100..=2000))
                .changed();

            if ui
                .button("▶ Recompute")
                .accessible_hover_text(
                    "Regenerate the bifurcation diagram with the current parameters",
                )
                .clicked()
                || changed
            {
                self.recompute();
            }

            ui.separator();
            ui.label("Zoom: Use mouse wheel to zoom, drag to pan.");
            ui.label("Double-click to reset view.");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            Plot::new("bifurcation_plot")
                .view_aspect(2.0)
                .x_axis_label("Growth Rate (r)")
                .y_axis_label("Population (x)")
                .show(ui, |plot_ui| {
                    plot_ui.points(
                        Points::new("Attractor", PlotPoints::new(self.points.clone()))
                            .radius(1.0_f32)
                            .color(egui::Color32::from_rgb(100, 200, 255)),
                    );
                });
        });
    }
}

// [cite:algorithmic_information_rust]
