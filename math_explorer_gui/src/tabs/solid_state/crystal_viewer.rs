use super::SolidStateTool;
use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints, Points};
use math_explorer::physics::solid_state::lattice::CrystalSystem;
use nalgebra::Vector3;

pub struct CrystalViewer {
    system: CrystalSystem,
    lattice_constant: f64,

    // Camera
    yaw: f32,
    pitch: f32,
    zoom: f32,
}

impl Default for CrystalViewer {
    fn default() -> Self {
        Self {
            system: CrystalSystem::FaceCenteredCubic,
            lattice_constant: 2.0,
            yaw: 0.5,
            pitch: 0.5,
            zoom: 50.0,
        }
    }
}

impl CrystalViewer {
    fn project(&self, p: Vector3<f64>) -> [f64; 2] {
        let center = Vector3::new(
            self.lattice_constant / 2.0,
            self.lattice_constant / 2.0,
            self.lattice_constant / 2.0,
        );
        let p_centered = p - center;

        // Rotation
        let cy = (self.yaw as f64).cos();
        let sy = (self.yaw as f64).sin();
        let cp = (self.pitch as f64).cos();
        let sp = (self.pitch as f64).sin();

        // Rotate around Y (Yaw)
        let x1 = p_centered.x * cy - p_centered.z * sy;
        let z1 = p_centered.x * sy + p_centered.z * cy;
        let y1 = p_centered.y;

        // Rotate around X (Pitch)
        let y2 = y1 * cp - z1 * sp;
        // let z2 = y1 * sp + z1 * cp; // Depth, ignored for 2D projection

        [x1 * (self.zoom as f64), y2 * (self.zoom as f64)]
    }

    fn draw_unit_cell_box(&self, ui: &mut egui_plot::PlotUi) {
        let a = self.lattice_constant;
        let corners = [
            Vector3::new(0.0, 0.0, 0.0), // 0
            Vector3::new(a, 0.0, 0.0),   // 1
            Vector3::new(a, a, 0.0),     // 2
            Vector3::new(0.0, a, 0.0),   // 3
            Vector3::new(0.0, 0.0, a),   // 4
            Vector3::new(a, 0.0, a),     // 5
            Vector3::new(a, a, a),       // 6
            Vector3::new(0.0, a, a),     // 7
        ];

        let edges = [
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 0), // Bottom face
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 4), // Top face
            (0, 4),
            (1, 5),
            (2, 6),
            (3, 7), // Verticals
        ];

        for (start_idx, end_idx) in edges {
            let start = self.project(corners[start_idx]);
            let end = self.project(corners[end_idx]);
            ui.line(
                Line::new("Edge", PlotPoints::new(vec![start, end]))
                    .color(egui::Color32::GRAY)
                    .width(1.0),
            );
        }
    }
}

impl SolidStateTool for CrystalViewer {
    fn name(&self) -> &'static str {
        "Crystal Lattice Viewer"
    }

    fn show(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("crystal_controls").show(ctx, |ui| {
            ui.heading("Crystal Lattice");
            ui.separator();

            ui.label("Crystal System");
            egui::ComboBox::from_label("System")
                .selected_text(format!("{:?}", self.system))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.system,
                        CrystalSystem::SimpleCubic,
                        "Simple Cubic",
                    );
                    ui.selectable_value(&mut self.system, CrystalSystem::BodyCenteredCubic, "BCC");
                    ui.selectable_value(&mut self.system, CrystalSystem::FaceCenteredCubic, "FCC");
                });

            ui.label("Lattice Constant (a)");
            ui.add(egui::Slider::new(&mut self.lattice_constant, 0.5..=5.0));

            ui.separator();
            ui.heading("Camera");
            ui.label("Yaw");
            ui.drag_angle(&mut self.yaw);
            ui.label("Pitch");
            ui.drag_angle(&mut self.pitch);
            ui.label("Zoom");
            ui.add(egui::Slider::new(&mut self.zoom, 10.0..=200.0));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            Plot::new("crystal_plot")
                .data_aspect(1.0)
                .show(ui, |plot_ui| {
                    // Draw Box
                    self.draw_unit_cell_box(plot_ui);

                    // Draw Atoms
                    let unit_cell = self.system.generate(self.lattice_constant);
                    let points: Vec<[f64; 2]> = unit_cell
                        .atomic_positions
                        .iter()
                        .map(|p| self.project(*p))
                        .collect();

                    plot_ui.points(
                        Points::new("Atoms", points)
                            .radius(5.0)
                            .color(egui::Color32::RED),
                    );
                });
            ui.label("Drag 'Yaw' and 'Pitch' to rotate.");
        });
    }
}
