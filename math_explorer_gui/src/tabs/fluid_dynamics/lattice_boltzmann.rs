use eframe::egui;
use egui::{Color32, ColorImage, TextureOptions};
use egui_plot::{Plot, PlotImage, PlotPoint};
use math_explorer::physics::fluid_dynamics::lattice_boltzmann::{
    BgkCollision, LatticeBoltzmannD2Q9,
};

#[derive(PartialEq, Clone, Copy)]
enum DrawMode {
    Obstacle,
    Clear,
}

pub struct LatticeBoltzmannTool {
    solver: LatticeBoltzmannD2Q9<BgkCollision>,
    running: bool,
    steps_per_frame: usize,

    // UI State
    texture: Option<egui::TextureHandle>,
    draw_mode: DrawMode,
    viscosity: f64,
    draw_radius: usize,
}

impl Default for LatticeBoltzmannTool {
    fn default() -> Self {
        let width = 100;
        let height = 50;
        let viscosity = 0.02;
        let tau = 3.0 * viscosity + 0.5;

        let mut solver = LatticeBoltzmannD2Q9::new(width, height, tau);

        // Initial setup: flow from left
        solver.set_inlet(0, 20, 5, 10, 0.1, 0.0);

        // Some initial obstacles (cylinder-ish)
        let center_x = 30;
        let center_y = 25;
        let radius = 5;
        for y in 0..height {
            for x in 0..width {
                let dx = x as i32 - center_x;
                let dy = y as i32 - center_y;
                if dx * dx + dy * dy <= radius * radius {
                    solver.set_obstacle(x, y, true);
                }
            }
        }

        Self {
            solver,
            running: false,
            steps_per_frame: 5,
            texture: None,
            draw_mode: DrawMode::Obstacle,
            viscosity,
            draw_radius: 2,
        }
    }
}

impl LatticeBoltzmannTool {
    pub fn show(&mut self, ctx: &egui::Context) {
        // Update Simulation
        if self.running {
            for _ in 0..self.steps_per_frame {
                // Constant inlet flow to drive the simulation
                self.solver.set_inlet(0, 0, 2, self.solver.height(), 0.1, 0.0);
                self.solver.step();
            }
            ctx.request_repaint();
        }

        // Controls
        egui::SidePanel::left("lbm_controls").show(ctx, |ui| {
            ui.heading("Lattice Boltzmann");
            ui.separator();

            if self.running {
                if ui.button("Pause").clicked() {
                    self.running = false;
                }
            } else if ui.button("Run").clicked() {
                self.running = true;
            }

            if ui.button("Reset").clicked() {
                let width = self.solver.width();
                let height = self.solver.height();
                self.solver = LatticeBoltzmannD2Q9::new(width, height, 3.0 * self.viscosity + 0.5);
                self.solver.set_inlet(0, 20, 5, 10, 0.1, 0.0);
            }

            if ui.button("Clear Obstacles").clicked() {
                self.solver.clear_obstacles();
            }

            ui.separator();
            ui.label("Simulation Parameters");
            if ui
                .add(egui::Slider::new(&mut self.viscosity, 0.005..=0.2).text("Viscosity"))
                .changed()
            {
                self.solver.collision_model.tau = 3.0 * self.viscosity + 0.5;
            }
            ui.add(
                egui::Slider::new(&mut self.steps_per_frame, 1..=20).text("Speed (Steps/Frame)"),
            );

            ui.separator();
            ui.label("Drawing");
            ui.horizontal(|ui| {
                ui.radio_value(&mut self.draw_mode, DrawMode::Obstacle, "Draw Wall");
                ui.radio_value(&mut self.draw_mode, DrawMode::Clear, "Erase");
            });
            ui.add(egui::Slider::new(&mut self.draw_radius, 1..=5).text("Brush Size"));

            ui.label("Instructions:");
            ui.small("Left Click + Drag on plot to draw/erase obstacles.");
        });

        // Visualization
        egui::CentralPanel::default().show(ctx, |ui| {
            // 1. Generate Image from Solver State
            let width = self.solver.width();
            let height = self.solver.height();
            // FIXED: Initialize with vector of black pixels
            let mut image = ColorImage::new([width, height], vec![Color32::BLACK; width * height]);

            for y in 0..height {
                for x in 0..width {
                    let pixel_idx = y * width + x;

                    if self.solver.is_obstacle(x, height - 1 - y) {
                        image.pixels[pixel_idx] = Color32::from_gray(50); // Obstacle color
                    } else {
                        // Color by velocity magnitude
                        // Max expected velocity approx 0.1 - 0.2
                        let v = self.solver.get_velocity_magnitude(x, height - 1 - y);
                        let intensity = (v * 1000.0).clamp(0.0, 255.0) as u8;
                        // Simple heatmap: Blue -> Red
                        // r: intensity, b: 255 - intensity
                        image.pixels[pixel_idx] = Color32::from_rgb(
                            intensity,
                            (intensity as f32 * 0.5) as u8,
                            255 - intensity,
                        );
                    }
                }
            }

            // 2. Load Texture
            let texture = ctx.load_texture("fluid_field", image, TextureOptions::NEAREST);
            self.texture = Some(texture.clone());

            // 3. Render Plot
            let _plot_response = Plot::new("lbm_plot")
                .data_aspect(1.0)
                .view_aspect(1.0)
                .show_grid(false)
                .show_axes([false, false])
                .allow_drag(false)
                .allow_zoom(false)
                .allow_scroll(false)
                .show(ui, |plot_ui| {
                    // FIXED: PlotImage::new takes 4 args: name, texture_id, center, size
                    plot_ui.image(PlotImage::new(
                        "fluid_field",
                        texture.id(),
                        PlotPoint::new(width as f64 / 2.0, height as f64 / 2.0),
                        [width as f32, height as f32],
                    ));

                    // Interaction: Get pointer coordinates
                    if plot_ui.response().hovered() && ctx.input(|i| i.pointer.primary_down()) {
                        if let Some(pos) = plot_ui.pointer_coordinate() {
                            let grid_x = pos.x.round() as i32;
                            let grid_y = pos.y.round() as i32;

                            self.apply_brush(grid_x, grid_y);
                        }
                    }
                });
        });
    }

    fn apply_brush(&mut self, cx: i32, cy: i32) {
        let r = self.draw_radius as i32;
        let width = self.solver.width() as i32;
        let height = self.solver.height() as i32;
        let is_obstacle = self.draw_mode == DrawMode::Obstacle;

        for y in (cy - r)..=(cy + r) {
            for x in (cx - r)..=(cx + r) {
                if x >= 0
                    && x < width
                    && y >= 0
                    && y < height
                    && (x - cx) * (x - cx) + (y - cy) * (y - cy) <= r * r
                {
                    self.solver
                        .set_obstacle(x as usize, y as usize, is_obstacle);
                }
            }
        }
    }
}
