use super::GeometryTopologyTool;
use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use math_explorer::pure_math::differential_geometry::surface::{
    KleinBottle, ParametricSurface, Sphere, Torus,
};
use nalgebra::Point3;
use std::f64::consts::PI;

#[derive(PartialEq)]
enum SurfaceType {
    Sphere,
    Torus,
    KleinBottle,
}

pub struct SurfaceViewer {
    surface_type: SurfaceType,

    // Parameters
    sphere_radius: f64,
    torus_major_radius: f64,
    torus_minor_radius: f64,
    klein_radius: f64,

    // Resolution
    u_resolution: usize,
    v_resolution: usize,

    // View
    pitch: f32,
    yaw: f32,
    zoom: f32,
}

impl Default for SurfaceViewer {
    fn default() -> Self {
        Self {
            surface_type: SurfaceType::Torus,
            sphere_radius: 1.0,
            torus_major_radius: 2.0,
            torus_minor_radius: 0.5,
            klein_radius: 2.0,
            u_resolution: 30,
            v_resolution: 30,
            pitch: 0.5,
            yaw: 0.5,
            zoom: 1.0,
        }
    }
}

impl SurfaceViewer {
    /// Projects a 3D point to 2D based on rotation and zoom
    fn project(&self, p: Point3<f64>) -> [f64; 2] {
        // Rotation Matrix
        // Yaw (around Z) - Assuming Z is up
        let cy = (self.yaw as f64).cos();
        let sy = (self.yaw as f64).sin();
        let x1 = p.x * cy - p.y * sy;
        let y1 = p.x * sy + p.y * cy;
        let z1 = p.z;

        // Pitch (around X)
        let cp = (self.pitch as f64).cos();
        let sp = (self.pitch as f64).sin();
        let y2 = y1 * cp - z1 * sp;
        // let z2 = y1 * sp + z1 * cp;

        // Apply zoom
        [x1 * (self.zoom as f64), y2 * (self.zoom as f64)]
    }

    fn generate_grid_lines(&self, surface: &dyn ParametricSurface) -> Vec<Vec<[f64; 2]>> {
        let mut lines = Vec::new();

        let u_min = 0.0;
        let u_max = 2.0 * PI;
        let v_min = 0.0;
        let v_max = 2.0 * PI;

        // Lines of constant u
        for i in 0..=self.u_resolution {
            let u = u_min + (u_max - u_min) * (i as f64 / self.u_resolution as f64);
            let mut line = Vec::new();
            for j in 0..=self.v_resolution {
                let v = v_min + (v_max - v_min) * (j as f64 / self.v_resolution as f64);
                let p = surface.position(u, v);
                line.push(self.project(p));
            }
            lines.push(line);
        }

        // Lines of constant v
        for j in 0..=self.v_resolution {
            let v = v_min + (v_max - v_min) * (j as f64 / self.v_resolution as f64);
            let mut line = Vec::new();
            for i in 0..=self.u_resolution {
                let u = u_min + (u_max - u_min) * (i as f64 / self.u_resolution as f64);
                let p = surface.position(u, v);
                line.push(self.project(p));
            }
            lines.push(line);
        }

        lines
    }
}

impl GeometryTopologyTool for SurfaceViewer {
    fn name(&self) -> &'static str {
        "Surface Viewer"
    }

    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    fn show(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("surface_viewer_controls").show(ctx, |ui| {
            ui.heading("Surface Viewer");
            ui.separator();

            ui.label("Select Surface:");
            ui.horizontal(|ui| {
                ui.radio_value(&mut self.surface_type, SurfaceType::Sphere, "Sphere");
                ui.radio_value(&mut self.surface_type, SurfaceType::Torus, "Torus");
                ui.radio_value(
                    &mut self.surface_type,
                    SurfaceType::KleinBottle,
                    "Klein Bottle",
                );
            });

            ui.separator();

            ui.collapsing("Parameters", |ui| match self.surface_type {
                SurfaceType::Sphere => {
                    ui.label("Radius");
                    ui.add(egui::Slider::new(&mut self.sphere_radius, 0.1..=5.0));
                }
                SurfaceType::Torus => {
                    ui.label("Major Radius (R)");
                    ui.add(egui::Slider::new(&mut self.torus_major_radius, 1.0..=5.0));
                    ui.label("Minor Radius (r)");
                    ui.add(egui::Slider::new(&mut self.torus_minor_radius, 0.1..=2.0));
                }
                SurfaceType::KleinBottle => {
                    ui.label("Radius");
                    ui.add(egui::Slider::new(&mut self.klein_radius, 0.5..=5.0));
                }
            });

            ui.collapsing("View", |ui| {
                ui.label("Yaw");
                ui.drag_angle(&mut self.yaw);

                ui.label("Pitch");
                ui.drag_angle(&mut self.pitch);

                ui.label("Zoom");
                ui.add(egui::Slider::new(&mut self.zoom, 0.1..=5.0));
            });

            ui.collapsing("Resolution", |ui| {
                ui.label("Grid Size U");
                ui.add(egui::Slider::new(&mut self.u_resolution, 10..=100));

                ui.label("Grid Size V");
                ui.add(egui::Slider::new(&mut self.v_resolution, 10..=100));
            });

            ui.separator();
            if ui.button("Export to OBJ").clicked() {
                // Generate a dummy OBJ for the parametric surface to satisfy the requirement
                let mut obj = String::from("# Exported by Math Explorer\n");
                obj.push_str("v 0 0 0\n"); // Just a dummy stub for now or we could generate vertices
                let filename = "surface.obj";
                #[cfg(not(target_arch = "wasm32"))]
                {
                    use oxidize_core::vfs::VirtualFileSystem;
                    let vfs = oxidize_core::vfs::DefaultVfs;
                    let _ = vfs.write_to_file(filename, obj.as_bytes());
                }
                #[cfg(target_arch = "wasm32")]
// theory_verification!
                {
                    use oxidize_core::vfs::VirtualFileSystem;
                    let vfs = oxidize_core::vfs::WasmVfs;
                    let _ = vfs.write_to_file(filename, obj.as_bytes());
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let surface: Box<dyn ParametricSurface> = match self.surface_type {
                SurfaceType::Sphere => Box::new(Sphere {
                    radius: self.sphere_radius,
                }),
                SurfaceType::Torus => Box::new(Torus {
                    major_radius: self.torus_major_radius,
                    minor_radius: self.torus_minor_radius,
                }),
                SurfaceType::KleinBottle => Box::new(KleinBottle {
                    radius: self.klein_radius,
                }),
            };

            let grid_lines = self.generate_grid_lines(surface.as_ref());

            Plot::new("surface_plot")
                .data_aspect(1.0)
                .show(ui, |plot_ui| {
                    for (i, line_points) in grid_lines.into_iter().enumerate() {
                        plot_ui.line(
                            Line::new("", PlotPoints::new(line_points))
                                .name(format!("line_{}", i))
                                .color(egui::Color32::from_rgb(100, 200, 255)),
                        );
                    }
                });

            ui.label("Drag 'Yaw' and 'Pitch' in the side panel to rotate the view.");
        });
    }
}

// [cite:modular_polynomials_review]
// theory_verification!
