use crate::framework::InteractiveTool;
use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use math_explorer::pure_math::differential_geometry::surface::{
    KleinBottle, ParametricSurface, Sphere, SurfaceAnalysis, Torus,
};
use nalgebra::Point3;
use std::f64::consts::PI;

#[derive(PartialEq)]
enum SurfaceType {
    Sphere,
    Torus,
    KleinBottle,
}

#[derive(PartialEq)]
enum CurvatureType {
    Gaussian,
    Mean,
}

pub struct CurvatureHeatmap {
    surface_type: SurfaceType,
    curvature_type: CurvatureType,

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

impl Default for CurvatureHeatmap {
    fn default() -> Self {
        Self {
            surface_type: SurfaceType::Torus,
            curvature_type: CurvatureType::Gaussian,
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

impl CurvatureHeatmap {
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

        // Apply zoom
        [x1 * (self.zoom as f64), y2 * (self.zoom as f64)]
    }

    /// Maps a curvature value to a color.
    fn map_color(&self, value: f64, min_val: f64, max_val: f64) -> egui::Color32 {
        // Normalize value to 0.0 .. 1.0
        let range = max_val - min_val;
        let mut normalized = if range == 0.0 {
            0.5
        } else {
            (value - min_val) / range
        };

        normalized = normalized.clamp(0.0, 1.0);

        // Simple colormap: Blue -> Cyan -> Green -> Yellow -> Red
        let r = if normalized < 0.5 {
            0
        } else if normalized < 0.75 {
            ((normalized - 0.5) * 4.0 * 255.0) as u8
        } else {
            255
        };

        let g = if normalized < 0.25 {
            (normalized * 4.0 * 255.0) as u8
        } else if normalized < 0.75 {
            255
        } else {
            ((1.0 - normalized) * 4.0 * 255.0) as u8
        };

        let b = if normalized < 0.25 {
            255
        } else if normalized < 0.5 {
            ((0.5 - normalized) * 4.0 * 255.0) as u8
        } else {
            0
        };

        egui::Color32::from_rgb(r, g, b)
    }

    fn generate_colored_lines<T: ParametricSurface>(
        &self,
        surface: &T,
    ) -> Vec<(Vec<[f64; 2]>, egui::Color32)> {
        let mut lines = Vec::new();

        let u_min = 0.0;
        let u_max = 2.0 * PI;
        let v_min = 0.0;
        let v_max = 2.0 * PI;

        // Collect all curvature values to find min/max for color mapping
        let mut min_curvature = f64::INFINITY;
        let mut max_curvature = f64::NEG_INFINITY;

        let mut curvature_map = vec![vec![0.0; self.v_resolution + 1]; self.u_resolution + 1];

        for (i, row) in curvature_map
            .iter_mut()
            .enumerate()
            .take(self.u_resolution + 1)
        {
            let u = u_min + (u_max - u_min) * (i as f64 / self.u_resolution as f64);
            for (j, item) in row.iter_mut().enumerate().take(self.v_resolution + 1) {
                let v = v_min + (v_max - v_min) * (j as f64 / self.v_resolution as f64);

                let c = match self.curvature_type {
                    CurvatureType::Gaussian => surface.gaussian_curvature(u, v),
                    CurvatureType::Mean => surface.mean_curvature(u, v),
                };

                *item = c;

                if c < min_curvature {
                    min_curvature = c;
                }
                if c > max_curvature {
                    max_curvature = c;
                }
            }
        }

        // Generate lines of constant u
        for i in 0..self.u_resolution {
            let u_start = u_min + (u_max - u_min) * (i as f64 / self.u_resolution as f64);
            let u_end = u_min + (u_max - u_min) * ((i + 1) as f64 / self.u_resolution as f64);

            for j in 0..self.v_resolution {
                let v_start = v_min + (v_max - v_min) * (j as f64 / self.v_resolution as f64);
                let v_end = v_min + (v_max - v_min) * ((j + 1) as f64 / self.v_resolution as f64);

                // Segment 1: u_start, v_start to u_start, v_end
                let p1 = surface.position(u_start, v_start);
                let p2 = surface.position(u_start, v_end);

                let c1 = curvature_map[i][j];
                let c2 = curvature_map[i][j + 1];
                let avg_curvature = (c1 + c2) / 2.0;

                lines.push((
                    vec![self.project(p1), self.project(p2)],
                    self.map_color(avg_curvature, min_curvature, max_curvature),
                ));

                // Segment 2: u_start, v_start to u_end, v_start
                let p3 = surface.position(u_end, v_start);

                let c3 = curvature_map[i + 1][j];
                let avg_curvature2 = (c1 + c3) / 2.0;

                lines.push((
                    vec![self.project(p1), self.project(p3)],
                    self.map_color(avg_curvature2, min_curvature, max_curvature),
                ));
            }
        }

        lines
    }
}

impl InteractiveTool for CurvatureHeatmap {
    fn name(&self) -> &'static str {
        "Curvature Heatmap"
    }

    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    fn show(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("curvature_heatmap_controls").show(ctx, |ui| {
            ui.heading("Curvature Heatmap");
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

            ui.label("Select Curvature Type:");
            ui.horizontal(|ui| {
                ui.radio_value(
                    &mut self.curvature_type,
                    CurvatureType::Gaussian,
                    "Gaussian",
                );
                ui.radio_value(&mut self.curvature_type, CurvatureType::Mean, "Mean");
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
                ui.label("Yaw");
                ui.drag_angle(&mut self.yaw);

                ui.label("Pitch");
                ui.drag_angle(&mut self.pitch);

                ui.add(egui::Slider::new(&mut self.zoom, 0.1..=5.0).text("Zoom"));
            });

            ui.collapsing("Resolution", |ui| {
                ui.add(egui::Slider::new(&mut self.u_resolution, 10..=50).text("Grid Size U"));

                ui.add(egui::Slider::new(&mut self.v_resolution, 10..=50).text("Grid Size V"));
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let lines = match self.surface_type {
                SurfaceType::Sphere => {
                    let surface = Sphere {
                        radius: self.sphere_radius,
                    };
                    self.generate_colored_lines(&surface)
                }
                SurfaceType::Torus => {
                    let surface = Torus {
                        major_radius: self.torus_major_radius,
                        minor_radius: self.torus_minor_radius,
                    };
                    self.generate_colored_lines(&surface)
                }
                SurfaceType::KleinBottle => {
                    let surface = KleinBottle {
                        radius: self.klein_radius,
                    };
                    self.generate_colored_lines(&surface)
                }
            };

            Plot::new("curvature_plot")
                .data_aspect(1.0)
                .show(ui, |plot_ui| {
                    for (i, (line_points, color)) in lines.into_iter().enumerate() {
                        plot_ui.line(
                            Line::new("", PlotPoints::new(line_points))
                                .name(format!("line_{}", i))
                                .color(color),
                        );
                    }
                });

            ui.label("Drag 'Yaw' and 'Pitch' in the side panel to rotate the view. Blue = Low Curvature, Red = High Curvature.");
        });
    }
}

// [cite:graph_parameters_rust]
