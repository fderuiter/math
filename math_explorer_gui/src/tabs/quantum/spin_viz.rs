use crate::accessibility::AccessibleHoverText;
use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints, Points};
use math_commons::theory::TheoryDescribable;
use math_commons::theory_verification;
use math_explorer::physics::quantum::{evolve_state, spin, QuantumOperator, QuantumState};
use num_complex::Complex;

use super::QuantumTool;

pub struct SpinVisualizer {
    psi: QuantumState,
    b_field: [f64; 3], // [Bx, By, Bz]
    time: f64,
    paused: bool,
    view_angle: [f64; 2], // [yaw, pitch] in radians
}

impl Default for SpinVisualizer {
    fn default() -> Self {
        // Initial state |0> (spin up)
        let psi = QuantumState::spin_zero();

        Self {
            psi,
            b_field: [0.0, 0.0, 1.0], // Default B field along Z
            time: 0.0,
            paused: true,
            view_angle: [0.5, 0.5], // Slight angle for 3D effect
        }
    }
}

impl SpinVisualizer {
    pub fn reset(&mut self) {
        // Reset to |0>
        self.psi = QuantumState::spin_zero();
        self.time = 0.0;
    }

    fn step(&mut self, dt: f64) {
        let sx = spin::sigma_x().matrix;
        let sy = spin::sigma_y().matrix;
        let sz = spin::sigma_z().matrix;

        let bx = self.b_field[0];
        let by = self.b_field[1];
        let bz = self.b_field[2];

        // H = 0.5 * (Bx*Sx + By*Sy + Bz*Sz)
        // Note: DMatrix * scalar works, but DMatrix<Complex> * Complex might need explicit loop or map if not implemented.
        // nalgebra usually implements mul for T: Scalar.
        // Let's rely on nalgebra's operator overloading.

        let h_matrix =
            (sx * Complex::new(bx, 0.0) + sy * Complex::new(by, 0.0) + sz * Complex::new(bz, 0.0))
                * Complex::new(0.5, 0.0);

        let hamiltonian = QuantumOperator::new(h_matrix);

        // Evolve
        self.psi = evolve_state(&self.psi, &hamiltonian, dt, 1.0);
        self.time += dt;
        self.psi = self.psi.normalize();
    }

    fn project(&self, point: [f64; 3]) -> [f64; 2] {
        let x = point[0];
        let y = point[1];
        let z = point[2];
        let yaw = self.view_angle[0];
        let pitch = self.view_angle[1];

        // Rotate around Z (yaw)
        let x1 = x * yaw.cos() - y * yaw.sin();
        let y1 = x * yaw.sin() + y * yaw.cos();
        let z1 = z;

        // Rotate around X (pitch)
        let x2 = x1;
        let y2 = y1 * pitch.cos() - z1 * pitch.sin();
        // let z2 = y1 * pitch.sin() + z1 * pitch.cos();

        // Project to 2D (orthographic along Z, so take X and Y)
        // Wait, standard convention: X right, Y up.
        // In 3D plot, Z is usually up.
        // So we want to project to screen X, Y.
        // Screen X = x2, Screen Y = z2 (vertical).
        // Let's use that convention.
        // Actually, let's just project x2, y2 for now.
        [x2, y2]
    }
}

impl QuantumTool for SpinVisualizer {
    fn name(&self) -> &'static str {
        "Spin Dynamics (Bloch Sphere)"
    }

    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    fn show(&mut self, ctx: &egui::Context) {
        if !self.paused {
            self.step(0.05);
            ctx.request_repaint();
        }

        egui::SidePanel::left("spin_controls").show(ctx, |ui| {
            ui.heading("Controls");
            ui.separator();

            ui.group(|ui| {
                ui.label("Magnetic Field B");
                ui.add(egui::Slider::new(&mut self.b_field[0], -5.0..=5.0).text("Bx"));
                ui.add(egui::Slider::new(&mut self.b_field[1], -5.0..=5.0).text("By"));
                ui.add(egui::Slider::new(&mut self.b_field[2], -5.0..=5.0).text("Bz"));
            });

            ui.separator();
            ui.horizontal(|ui| {
                if ui
                    .button(if self.paused { "▶ Play" } else { "⏸ Pause" })
                    .clicked()
                {
                    self.paused = !self.paused;
                }
                if ui
                    .button("🔄 Reset")
                    .accessible_hover_text(format!(
                        "Reset to {}",
                        theory_verification!(QuantumState::spin_zero())
                    ))
                    .clicked()
                {
                    self.reset();
                }
            });

            ui.separator();
            ui.heading("View");
            ui.add(
                egui::Slider::new(&mut self.view_angle[0], 0.0..=std::f64::consts::TAU).text("Yaw"),
            );
            ui.add(
                egui::Slider::new(&mut self.view_angle[1], 0.0..=std::f64::consts::TAU)
                    .text("Pitch"),
            );

            ui.separator();
            // Calculate expectation values
            let sx_op = spin::sigma_x();
            let sy_op = spin::sigma_y();
            let sz_op = spin::sigma_z();

            let ex = sx_op.expectation_value(&self.psi).re;
            let ey = sy_op.expectation_value(&self.psi).re;
            let ez = sz_op.expectation_value(&self.psi).re;

            ui.label(format!("Ex (x): {:.3}", ex));
            ui.label(format!("Ey (y): {:.3}", ey));
            ui.label(format!("Ez (z): {:.3}", ez));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let sx_op = spin::sigma_x();
            let sy_op = spin::sigma_y();
            let sz_op = spin::sigma_z();

            let ex = sx_op.expectation_value(&self.psi).re;
            let ey = sy_op.expectation_value(&self.psi).re;
            let ez = sz_op.expectation_value(&self.psi).re;

            // Generate sphere wireframe
            let mut circle_points = Vec::new();
            // Equator (z=0)
            let mut equator = Vec::new();
            for i in 0..=60 {
                let theta = i as f64 * std::f64::consts::TAU / 60.0;
                equator.push(self.project([theta.cos(), theta.sin(), 0.0]));
            }
            circle_points.push(("Equator", equator, egui::Color32::GRAY));

            // Meridian (x=0)
            let mut meridian = Vec::new();
            for i in 0..=60 {
                let theta = i as f64 * std::f64::consts::TAU / 60.0;
                meridian.push(self.project([0.0, theta.cos(), theta.sin()]));
            }
            circle_points.push(("Meridian YZ", meridian, egui::Color32::GRAY));

            // Meridian (y=0)
            let mut meridian2 = Vec::new();
            for i in 0..=60 {
                let theta = i as f64 * std::f64::consts::TAU / 60.0;
                meridian2.push(self.project([theta.cos(), 0.0, theta.sin()]));
            }
            circle_points.push(("Meridian XZ", meridian2, egui::Color32::GRAY));

            // Vector
            let start = self.project([0.0, 0.0, 0.0]);
            let end = self.project([ex, ey, ez]);
            let vec_line = vec![start, end];

            Plot::new("bloch_sphere")
                .data_aspect(1.0)
                .view_aspect(1.0)
                .show(ui, |plot_ui| {
                    for (name, points, color) in circle_points {
                        plot_ui.line(Line::new(name, PlotPoints::new(points)).color(color));
                    }
                    // Draw axes
                    let origin = self.project([0.0, 0.0, 0.0]);
                    let x_axis = vec![origin, self.project([1.2, 0.0, 0.0])];
                    let y_axis = vec![origin, self.project([0.0, 1.2, 0.0])];
                    let z_axis = vec![origin, self.project([0.0, 0.0, 1.2])];

                    plot_ui.line(Line::new("X", PlotPoints::new(x_axis)).color(egui::Color32::RED));
                    plot_ui
                        .line(Line::new("Y", PlotPoints::new(y_axis)).color(egui::Color32::GREEN));
                    plot_ui
                        .line(Line::new("Z", PlotPoints::new(z_axis)).color(egui::Color32::BLUE));

                    // Draw state vector
                    plot_ui.line(
                        Line::new("State", PlotPoints::new(vec_line))
                            .color(egui::Color32::YELLOW)
                            .width(3.0_f32),
                    );

                    // Draw point at tip
                    plot_ui.points(
                        Points::new("State Tip", PlotPoints::new(vec![end]))
                            .radius(5.0_f32)
                            .color(egui::Color32::YELLOW),
                    );
                });
        });
    }
}

// [cite:quantum_mechanics]
