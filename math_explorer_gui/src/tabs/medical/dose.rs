use crate::accessibility::PlotAccessibilityExt;
use super::MedicalTool;
use eframe::egui;
use egui::{Color32, ColorImage, TextureOptions};
use egui_plot::{Plot, PlotImage, PlotPoint};
use math_explorer::physics::medical::dose::kernel::{DoseKernel, ExponentialKernel};

pub struct DoseCalculationTool {
    amplitude: f64,
    beta: f64,
    width: usize,
    height: usize,
    texture: Option<egui::TextureHandle>,
}

impl Default for DoseCalculationTool {
    fn default() -> Self {
        Self {
            amplitude: 1.0,
            beta: 1.0,
            width: 200,
            height: 200,
            texture: None,
        }
    }
}

impl MedicalTool for DoseCalculationTool {
    fn name(&self) -> &'static str {
        "Dose Calculation"
    }

    fn show(&mut self, ctx: &egui::Context) {
        let mut changed = false;

        egui::SidePanel::left("dose_controls").show(ctx, |ui| {
            ui.heading("Dose Parameters");
            ui.separator();

            ui.label("Source Amplitude");
            if ui
                .add(egui::Slider::new(&mut self.amplitude, 0.1..=10.0))
                .changed()
            {
                changed = true;
            }

            ui.label("Attenuation (Beta)");
            if ui
                .add(egui::Slider::new(&mut self.beta, 0.1..=5.0))
                .changed()
            {
                changed = true;
            }

            ui.separator();
            ui.label("Kernel: Exponential");
            ui.label("K(r) = (A / r^2) * exp(-beta * r)");
            ui.small("Visualizing dose from a point source at (0,0)");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // Recompute texture if needed or if params changed
            if self.texture.is_none() || changed {
                self.recompute_texture(ctx);
            }

            if let Some(texture) = &self.texture {
                Plot::new("dose_heatmap")
                    .data_aspect(1.0)
                    .view_aspect(1.0)
                    .show_grid(false)
                    .show_axes([true, true])
                    .show_accessible(ui, "Dynamic state of dose_heatmap updated.", |plot_ui| {
                        plot_ui.image(PlotImage::new(
                            "dose_field",
                            texture.id(),
                            PlotPoint::new(0.0, 0.0),
                            [20.0, 20.0], // Physical dimensions (-10 to 10)
                        ));
                    });
            }
        });
    }
}

impl DoseCalculationTool {
    fn recompute_texture(&mut self, ctx: &egui::Context) {
        let width = self.width;
        let height = self.height;
        let mut pixels = vec![Color32::BLACK; width * height];

        let kernel = ExponentialKernel::new(self.amplitude, self.beta);

        // Physical range: -10.0 to 10.0
        let range = 20.0;
        let dx = range / width as f64;
        let dy = range / height as f64;

        // Find max value for normalization (approx at r -> slightly > 0)
        // Since 1/r^2 diverges at 0, we clamp/saturate.
        // Let's sample at a small radius to get a "max" reference.
        let max_dose = kernel.value_at(0.1).unwrap_or(10.0);

        for y in 0..height {
            let py = (y as f64 - height as f64 / 2.0) * dy;
            for x in 0..width {
                let px = (x as f64 - width as f64 / 2.0) * dx;
                let r = (px * px + py * py).sqrt();

                let dose = if r < 1e-3 {
                    max_dose // Singularity handling
                } else {
                    kernel.value_at(r).unwrap_or(0.0)
                };

                // Map dose to color (Heatmap: Black -> Blue -> Red -> White)
                let intensity = (dose / max_dose).clamp(0.0, 1.0);
                pixels[y * width + x] = map_value_to_color(intensity);
            }
        }

        let image = ColorImage::new([width, height], pixels);

        self.texture = Some(ctx.load_texture("dose_field", image, TextureOptions::LINEAR));
    }
}

fn map_value_to_color(t: f64) -> Color32 {
    // Simple heatmap gradient
    // t: 0.0 -> 1.0
    // 0.0 - 0.25: Black -> Blue
    // 0.25 - 0.5: Blue -> Cyan
    // 0.5 - 0.75: Cyan -> Yellow
    // 0.75 - 1.0: Yellow -> Red

    // Simplified:
    // Low: Blue, Mid: Green, High: Red
    let r = (t * 2.0 - 1.0).max(0.0);
    let g = (1.0 - (t * 2.0 - 1.0).abs()).max(0.0);
    let b = (1.0 - t * 2.0).max(0.0);

    // Apply gamma or brightness boost
    let r = (r * 255.0) as u8;
    let g = (g * 255.0) as u8;
    let b = (b * 255.0) as u8;

    Color32::from_rgb(r, g, b)
}

// [cite:graph_parameters_rust]
