use crate::tabs::geometry_topology::GeometryTopologyTool;
use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints, Points, Polygon};
use math_explorer::pure_math::statistics::tda::{vietoris_rips_complex, Point2D, PointCloud};

pub struct SimplicialComplexesTool {
    points: Vec<Point2D>,
    radius: f64,
}

impl Default for SimplicialComplexesTool {
    fn default() -> Self {
        // Generate some interesting points in a circle with some inner points
        let n = 20;
        let mut points = Vec::new();
        for i in 0..n {
            let angle = 2.0 * std::f64::consts::PI * (i as f64) / (n as f64);
            points.push(Point2D::new(angle.cos() * 2.0, angle.sin() * 2.0));
        }
        points.push(Point2D::new(0.0, 0.0));
        points.push(Point2D::new(0.5, 0.5));
        points.push(Point2D::new(-0.5, -0.5));

        Self {
            points,
            radius: 1.0,
        }
    }
}

impl GeometryTopologyTool for SimplicialComplexesTool {
    fn name(&self) -> &'static str {
        "Simplicial Complexes (TDA)"
    }

    fn show(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("simplicial_complex_panel").show(ctx, |ui| {
            ui.heading("Vietoris-Rips Complex");
            ui.separator();

            ui.label("Radius (\u{03B5}):");
            ui.add(egui::Slider::new(&mut self.radius, 0.0..=5.0).text("Radius"));

            ui.separator();
            if ui.button("Regenerate Points (Circle)").clicked() {
                *self = Self::default();
            }
            if ui.button("Regenerate Points (Grid)").clicked() {
                let mut grid = Vec::new();
                for i in 0..5 {
                    for j in 0..5 {
                        grid.push(Point2D::new(i as f64 - 2.0, j as f64 - 2.0));
                    }
                }
                self.points = grid;
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let cloud_result = PointCloud::new(self.points.clone());

            match cloud_result {
                Ok(cloud) => {
                    let complex_result = vietoris_rips_complex(&cloud, self.radius);
                    match complex_result {
                        Ok(complex) => {
                            Plot::new("simplicial_complex_plot")
                                .view_aspect(1.0)
                                .data_aspect(1.0)
                                .show(ui, |plot_ui| {
                                    // Plot 2-simplices (triangles)
                                    let triangles = complex.get_simplices(2);
                                    for t in triangles {
                                        if t.vertices.len() == 3 {
                                            let p1 = &cloud.points[t.vertices[0]];
                                            let p2 = &cloud.points[t.vertices[1]];
                                            let p3 = &cloud.points[t.vertices[2]];
                                            let poly_points =
                                                vec![[p1.x, p1.y], [p2.x, p2.y], [p3.x, p3.y]];
                                            plot_ui.polygon(
                                                Polygon::new("", PlotPoints::new(poly_points))
                                                    .stroke(egui::Stroke::new(
                                                        1.0,
                                                        egui::Color32::from_rgba_unmultiplied(
                                                            0, 150, 255, 50,
                                                        ),
                                                    ))
                                                    .fill_color(
                                                        egui::Color32::from_rgba_unmultiplied(
                                                            0, 150, 255, 50,
                                                        ),
                                                    )
                                                    .name("Triangle"),
                                            );
                                        }
                                    }

                                    // Plot 1-simplices (edges)
                                    let edges = complex.get_simplices(1);
                                    for e in edges {
                                        if e.vertices.len() == 2 {
                                            let p1 = &cloud.points[e.vertices[0]];
                                            let p2 = &cloud.points[e.vertices[1]];
                                            plot_ui.line(
                                                Line::new(
                                                    "",
                                                    PlotPoints::new(vec![
                                                        [p1.x, p1.y],
                                                        [p2.x, p2.y],
                                                    ]),
                                                )
                                                .color(egui::Color32::from_rgb(100, 100, 255))
                                                .width(2.0),
                                            );
                                        }
                                    }

                                    // Plot 0-simplices (vertices)
                                    let mut vertex_coords = Vec::new();
                                    let vertices = complex.get_simplices(0);
                                    for v in vertices {
                                        if v.vertices.len() == 1 {
                                            let p = &cloud.points[v.vertices[0]];
                                            vertex_coords.push([p.x, p.y]);
                                        }
                                    }

                                    plot_ui.points(
                                        Points::new("", PlotPoints::new(vertex_coords))
                                            .radius(4.0)
                                            .color(egui::Color32::RED),
                                    );
                                });
                        }
                        Err(e) => {
                            ui.label(format!("Error generating complex: {:?}", e));
                        }
                    }
                }
                Err(e) => {
                    ui.label(format!("Error generating point cloud: {:?}", e));
                }
            }
        });
    }
}
