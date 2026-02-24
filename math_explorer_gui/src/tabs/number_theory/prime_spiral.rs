use eframe::egui::{self, Color32, ColorImage, TextureHandle, TextureOptions, Ui};
use math_explorer::pure_math::number_theory::primes::primes_up_to;

pub struct PrimeSpiralWidget {
    grid_size: usize,
    texture: Option<TextureHandle>,
}

impl Default for PrimeSpiralWidget {
    fn default() -> Self {
        Self {
            grid_size: 200,
            texture: None,
        }
    }
}

impl PrimeSpiralWidget {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label("Grid Size:");
                if ui
                    .add(egui::Slider::new(&mut self.grid_size, 10..=1000).text("side length"))
                    .changed()
                {
                    self.texture = None; // Invalidate texture
                }
            });

            if self.texture.is_none() {
                self.regenerate_texture(ui.ctx());
            }

            if let Some(texture) = &self.texture {
                // Correctly use ui.image with 1 argument (impl Into<ImageSource>)
                ui.image(texture);
            }
        });
    }

    fn regenerate_texture(&mut self, ctx: &egui::Context) {
        let size = self.grid_size;
        let num_pixels = size * size;
        let mut pixels = vec![Color32::BLACK; num_pixels];

        // Ulam Spiral Logic
        let center_x = (size / 2) as i32;
        let center_y = (size / 2) as i32;

        let mut x = 0;
        let mut y = 0;
        let mut dx = 0;
        let mut dy = -1;
        // removed unused 't' and 'max_i'

        let limit = (size * size) as u64;
        let primes = primes_up_to(limit);

        let mut is_prime_lookup = vec![false; (limit + 1) as usize];
        for &p in &primes {
            is_prime_lookup[p as usize] = true;
        }

        for i in 1..=num_pixels {
            // Fix unary negation on usize by casting to i32 first
            if (-(size as i32) / 2 <= x)
                && (x <= size as i32 / 2)
                && (-(size as i32) / 2 <= y)
                && (y <= size as i32 / 2)
            {
                let px = (center_x + x) as usize;
                let py = (center_y + y) as usize;

                if px < size && py < size {
                    if is_prime_lookup[i] {
                        pixels[py * size + px] = Color32::WHITE;
                    }
                     if i == 1 {
                         pixels[py * size + px] = Color32::RED;
                     }
                }
            }

            if x == y || (x < 0 && x == -y) || (x > 0 && x == 1 - y) {
                let temp = dx;
                dx = -dy;
                dy = temp;
            }

            x += dx;
            y += dy;
        }

        // Use ..Default::default() to fill potentially new fields like source_size
        let image = ColorImage {
            size: [size, size],
            pixels,
            ..Default::default()
        };

        self.texture = Some(ctx.load_texture(
            "ulam_spiral",
            image,
            TextureOptions::NEAREST, // Nearest neighbor for crisp pixels
        ));
    }
}
