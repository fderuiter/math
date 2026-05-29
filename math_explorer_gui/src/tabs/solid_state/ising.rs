use super::SolidStateTool;
use crate::accessibility::AccessibleHoverText;
use eframe::egui;
use physics::stat_mech::ising::SpinLattice;
use physics::stat_mech::KB;

pub struct IsingModelTool {
    lattice: SpinLattice,
    temperature: f64,
    j_coupling: f64,
    h_field: f64,
    running: bool,
    steps_per_frame: usize,
    texture: Option<egui::TextureHandle>,
}

impl Default for IsingModelTool {
    fn default() -> Self {
        Self {
            lattice: SpinLattice::new(100, 100),
            temperature: 2.269, // Near Critical Temp for J=1
            j_coupling: 1.0,
            h_field: 0.0,
            running: false,
            steps_per_frame: 5000,
            texture: None,
        }
    }
}

impl SolidStateTool for IsingModelTool {
    fn name(&self) -> &'static str {
        "Ising Model"
    }

    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    fn show(&mut self, ctx: &egui::Context) {
        // Controls
        egui::SidePanel::left("ising_controls")
            .resizable(false)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Ising Model Controls");
                ui.separator();

                ui.label("Temperature (T)");
                ui.add(egui::Slider::new(&mut self.temperature, 0.1..=10.0).text("K (relative)"));
                ui.small("Critical Temp Tc ≈ 2.269 (for J=1)");

                ui.label("Coupling Constant (J)");
                ui.add(egui::Slider::new(&mut self.j_coupling, -2.0..=2.0).text("J"));
                ui.small("Positive: Ferromagnetic\nNegative: Antiferromagnetic");

                ui.label("External Field (H)");
                ui.add(egui::Slider::new(&mut self.h_field, -2.0..=2.0).text("H"));

                ui.separator();

                ui.label("Simulation Speed");
                ui.add(
                    egui::Slider::new(&mut self.steps_per_frame, 100..=50000)
                        .text("Steps/Frame")
                        .logarithmic(true),
                );

                ui.separator();

                ui.horizontal(|ui| {
                    if ui
                        .button(if self.running {
                            "⏸ Pause"
                        } else {
                            "▶ Play"
                        })
                        .accessible_hover_text(if self.running {
                            "Pause the Ising model simulation"
                        } else {
                            "Resume the Ising model simulation"
                        })
                        .clicked()
                    {
                        self.running = !self.running;
                    }

                    if ui
                        .button("🔄 Reset")
                        .accessible_hover_text("Reset the lattice spin state")
                        .clicked()
                    {
                        self.lattice = SpinLattice::new(100, 100);
                        self.texture = None; // Force texture recreation
                    }
                });

                ui.separator();

                let m = self.lattice.magnetization();
                let max_m = (self.lattice.width() * self.lattice.height()) as f64;
                let m_avg = m as f64 / max_m;
                ui.label(format!("Magnetization: {:.3}", m_avg));

                let energy = self.lattice.hamiltonian(self.j_coupling, self.h_field);
                ui.label(format!("Energy: {:.3}", energy));
            });

        // Simulation Update
        if self.running {
            // Convert dimensionless T to physical T if needed.
            // The `evolve` method takes T. `SpinLattice` uses `beta = 1.0 / (KB * temperature)`.
            // So if `self.temperature` is dimensionless (relative to J/KB), pass `self.temperature * J / KB`?
            // The example in `ising.rs` says: `let temp = 1.0 * j_coupling / KB;`
            // So if `self.temperature` is meant to be in units of J/kB, then pass `self.temperature / KB`?
            // Wait.
            // Beta = 1 / (KB * T_phys).
            // We want Beta * J = 1/T_dim.
            // So 1 / (KB * T_phys) * J = 1/T_dim.
            // T_phys = T_dim * J / KB.
            // Let's assume J=1 for the unit scale. Then T_phys = T_dim / KB.
            // If the slider `self.temperature` is T_dim (e.g., 2.269), then pass `self.temperature / KB`.
            // Wait, if J != 1, then T_c shifts. T_c is roughly 2.269 J/kB.
            // So T_phys = T_dim * |J| / KB.
            // Let's implement that logic to keep the slider meaning consistent relative to J.
            // If J=0, use 1.0 as scale fallback to avoid div by zero or weirdness.
            let j_scale = if self.j_coupling.abs() < 1e-6 {
                1.0
            } else {
                self.j_coupling.abs()
            };
            let t_phys = self.temperature * j_scale / KB;

            self.lattice
                .evolve(self.steps_per_frame, t_phys, self.j_coupling, self.h_field);
            ctx.request_repaint();
        }

        // Visualization
        egui::CentralPanel::default().show(ctx, |ui| {
            let width = self.lattice.width();
            let height = self.lattice.height();

            let image = egui::ColorImage::from_rgba_unmultiplied(
                [width, height],
                &self
                    .lattice
                    .spins()
                    .iter()
                    .flat_map(|&s| {
                        if s > 0 {
                            [200, 50, 50, 255] // Red (Up)
                        } else {
                            [50, 50, 200, 255] // Blue (Down)
                        }
                    })
                    .collect::<Vec<u8>>(),
            );

            let texture_opts = egui::TextureOptions::NEAREST; // Pixelated look
            if let Some(texture) = &mut self.texture {
                texture.set(image, texture_opts);
            } else {
                self.texture = Some(ctx.load_texture("ising_lattice", image, texture_opts));
            }

            if let Some(texture) = &self.texture {
                ui.image(texture); // Show the texture
            }
        });
    }
}

// [cite:clinical_trials_statistics]
