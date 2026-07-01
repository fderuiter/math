use crate::framework::InteractiveTool;
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
    camera: crate::framework::Camera3D,
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
            camera: crate::framework::Camera3D::new(0.5, 0.5, 1.0),
        }
    }
}

impl SurfaceViewer {
    /// Projects a 3D point to 2D based on rotation and zoom
    fn project(&self, p: Point3<f64>) -> [f64; 2] {
        self.camera.project(&[p.x, p.y, p.z])
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
    fn create_surface(&self) -> Box<dyn ParametricSurface> {
        match self.surface_type {
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
        }
    }
}

impl InteractiveTool for SurfaceViewer {
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
                    ui.add(egui::Slider::new(&mut self.sphere_radius, 0.1..=5.0).text("Radius"));
                }
                SurfaceType::Torus => {
                    ui.add(egui::Slider::new(&mut self.torus_major_radius, 1.0..=5.0).text("Major Radius (R)"));
                    ui.add(egui::Slider::new(&mut self.torus_minor_radius, 0.1..=2.0).text("Minor Radius (r)"));
                }
                SurfaceType::KleinBottle => {
                    ui.add(egui::Slider::new(&mut self.klein_radius, 0.5..=5.0).text("Radius"));
                }
            });

            ui.collapsing("View", |ui| {
                self.camera.ui(ui);
            });

            ui.collapsing("Resolution", |ui| {
                ui.add(egui::Slider::new(&mut self.u_resolution, 10..=100).text("Grid Size U"));

                ui.add(egui::Slider::new(&mut self.v_resolution, 10..=100).text("Grid Size V"));
            });

            ui.separator();
            if ui.button("Export to OBJ").clicked() {
                let surface = self.create_surface();
                let mesh = super::export_utils::surface_to_mesh(
                    surface.as_ref(),
                    self.u_resolution,
                    self.v_resolution,
                );
                let obj = oxidize_core::mesh::export_mesh_to_obj_string(&mesh)
                    .unwrap_or_else(|_| String::new());
                let filename = "surface.obj";
                #[cfg(not(target_arch = "wasm32"))]
                {
                    use oxidize_core::vfs::VirtualFileSystem;
                    let vfs = oxidize_core::vfs::DefaultVfs;
                    let _ = vfs.write_to_file(filename, obj.as_bytes());
                }
                #[cfg(target_arch = "wasm32")]
                // theory validation
                {
                    use oxidize_core::vfs::VirtualFileSystem;
                    let vfs = oxidize_core::vfs::WasmVfs;
                    let _ = vfs.write_to_file(filename, obj.as_bytes());
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let surface = self.create_surface();

            let grid_lines = self.generate_grid_lines(surface.as_ref());

            let response = Plot::new("surface_plot")
                .data_aspect(1.0)
                .show(ui, |plot_ui| {
                    for (i, line_points) in grid_lines.into_iter().enumerate() {
                        plot_ui.line(
                            Line::new("", PlotPoints::new(line_points))
                                .name(format!("line_{}", i))
                                .color(egui::Color32::from_rgb(100, 200, 255)),
                        );
                    }
                })
                .response;

            self.camera.handle_interaction(&response, ui);

            ui.label("Drag 'Yaw' and 'Pitch' in the side panel to rotate the view.");
        });
    }
}

// [cite:modular_polynomials_review]
// theory validation
