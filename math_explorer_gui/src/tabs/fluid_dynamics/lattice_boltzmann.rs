use crate::accessibility::AccessibleHoverText;
use crate::async_sim::{SimCommand, SimulationController, SimulationRunner, StateSnapshot};
use eframe::egui;
use egui::{Color32, ColorImage, TextureOptions};
use egui_plot::{Plot, PlotImage, PlotPoint};
use math_explorer::physics::fluid_dynamics::lattice_boltzmann::{
    BgkCollision, LatticeBoltzmannD2Q9,
};
use std::sync::Arc;

#[derive(PartialEq, Clone, Copy)]
enum DrawMode {
    Obstacle,
    Clear,
}

pub struct LbmRunner {
    solver: LatticeBoltzmannD2Q9<BgkCollision>,
    steps_per_frame: usize,
    viscosity: f64,
}

impl LbmRunner {
    pub fn new(viscosity: f64) -> Self {
        let width = 100;
        let height = 50;
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
            steps_per_frame: 5,
            viscosity,
        }
    }
}

impl SimulationRunner for LbmRunner {
    fn process_command(&mut self, cmd: SimCommand) {
        match cmd {
            SimCommand::SetSpeed(speed) => self.steps_per_frame = speed,
            SimCommand::UpdateParam(name, val) if name == "viscosity" => {
                self.viscosity = val;
                self.solver.collision_model.tau = 3.0 * self.viscosity + 0.5;
            }
            SimCommand::UpdateParam(_, _) => {}
            SimCommand::ApplyBrush {
                cx,
                cy,
                r,
                is_obstacle,
            } => {
                let width = self.solver.width() as i32;
                let height = self.solver.height() as i32;
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
            SimCommand::ClearObstacles => {
                self.solver.clear_obstacles();
            }
            SimCommand::Reset => {
                let width = self.solver.width();
                let height = self.solver.height();
                self.solver = LatticeBoltzmannD2Q9::new(width, height, 3.0 * self.viscosity + 0.5);
                self.solver.set_inlet(0, 20, 5, 10, 0.1, 0.0);
            }
            _ => {}
        }
    }

    fn step(&mut self) {
        self.solver
            .set_inlet(0, 0, 2, self.solver.height(), 0.1, 0.0);
        self.solver.step();
    }

    fn get_snapshot(&self) -> StateSnapshot {
        let width = self.solver.width();
        let height = self.solver.height();
        let mut pixels = vec![Color32::BLACK; width * height];

        for y in 0..height {
            for x in 0..width {
                let pixel_idx = y * width + x;
                if self.solver.is_obstacle(x, height - 1 - y) {
                    pixels[pixel_idx] = Color32::from_gray(50);
                } else {
                    let v = self.solver.get_velocity_magnitude(x, height - 1 - y);
                    let intensity = (v * 1000.0).clamp(0.0, 255.0) as u8;
                    pixels[pixel_idx] = Color32::from_rgb(
                        intensity,
                        (intensity as f32 * 0.5) as u8,
                        255 - intensity,
                    );
                }
            }
        }

        StateSnapshot {
            width,
            height,
            pixels: Arc::new(pixels),
            custom_data: Vec::new(),
            structured_data: None,
        }
    }

    fn get_steps_per_frame(&self) -> usize {
        self.steps_per_frame
    }
}

pub struct LatticeBoltzmannTool {
    controller: SimulationController,

    // UI State
    texture: Option<egui::TextureHandle>,
    draw_mode: DrawMode,
    viscosity: f64,
    draw_radius: usize,
    steps_per_frame: usize,

    // Cached snapshot info for rendering
    last_width: usize,
    last_height: usize,
    last_pixels: Option<Arc<Vec<Color32>>>,
}

impl Default for LatticeBoltzmannTool {
    fn default() -> Self {
        let viscosity = 0.02;
        let runner = LbmRunner::new(viscosity);
        let controller = SimulationController::new(runner);

        Self {
            controller,
            texture: None,
            draw_mode: DrawMode::Obstacle,
            viscosity,
            draw_radius: 2,
            steps_per_frame: 5,
            last_width: 100,
            last_height: 50,
            last_pixels: None,
        }
    }
}

use crate::tabs::fluid_dynamics::FluidDynamicsTool;

impl FluidDynamicsTool for LatticeBoltzmannTool {
    fn name(&self) -> &'static str {
        "Lattice Boltzmann (Demo)"
    }

    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    fn show(&mut self, ctx: &egui::Context) {
        // Update state from background simulation
        if let Some(snapshot) = self.controller.update() {
            self.last_width = snapshot.width;
            self.last_height = snapshot.height;
            self.last_pixels = Some(Arc::clone(&snapshot.pixels));
            ctx.request_repaint(); // Keep repainting if we're receiving new frames
        } else if self.controller.running {
            // Still request repaint while running even if no new snapshot this exact frame
            ctx.request_repaint();
        }

        let is_running = self.controller.running;

        // Controls
        egui::SidePanel::left("lbm_controls").show(ctx, |ui| {
            ui.heading("Lattice Boltzmann");
            ui.separator();

            if ui
                .button(if is_running { "⏸ Pause" } else { "▶ Run" })
                .accessible_hover_text(if is_running {
                    "Pause the fluid simulation"
                } else {
                    "Start the fluid simulation"
                })
                .clicked()
            {
                if is_running {
                    self.controller.send_command(SimCommand::Pause);
                } else {
                    self.controller.send_command(SimCommand::Start);
                }
            }

            if ui
                .button("↻ Reset")
                .accessible_hover_text("Reset the fluid field and re-initialize the simulation")
                .clicked()
            {
                self.controller.send_command(SimCommand::Reset);
            }

            if ui
                .button("🔄 Clear Obstacles")
                .accessible_hover_text("Remove all obstacles from the fluid field")
                .clicked()
            {
                self.controller.send_command(SimCommand::ClearObstacles);
            }

            ui.separator();
            ui.label("Simulation Parameters");
            if ui
                .add(egui::Slider::new(&mut self.viscosity, 0.005..=0.2).text("Viscosity"))
                .changed()
            {
                self.controller.send_command(SimCommand::UpdateParam(
                    "viscosity".to_string(),
                    self.viscosity,
                ));
            }
            if ui
                .add(
                    egui::Slider::new(&mut self.steps_per_frame, 1..=20)
                        .text("Speed (Steps/Frame)"),
                )
                .changed()
            {
                self.controller
                    .send_command(SimCommand::SetSpeed(self.steps_per_frame));
            }

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
            let width = self.last_width;
            let height = self.last_height;

            if let Some(pixels) = &self.last_pixels {
                // Update texture if we have new pixels
                let image = ColorImage::new([width, height], pixels.as_ref().clone());
                let texture = ctx.load_texture("fluid_field", image, TextureOptions::NEAREST);
                self.texture = Some(texture);
            }

            // Render Plot
            if let Some(texture) = &self.texture {
                let _plot_response = Plot::new("lbm_plot")
                    .data_aspect(1.0)
                    .view_aspect(1.0)
                    .show_grid(false)
                    .show_axes([false, false])
                    .allow_drag(false)
                    .allow_zoom(false)
                    .allow_scroll(false)
                    .show(ui, |plot_ui| {
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

                                self.controller.send_command(SimCommand::ApplyBrush {
                                    cx: grid_x,
                                    cy: grid_y,
                                    r: self.draw_radius as i32,
                                    is_obstacle: self.draw_mode == DrawMode::Obstacle,
                                });
                            }
                        }
                    });
            }
        });
    }
}

// [cite:graph_parameters_rust]
