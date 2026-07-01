use crate::accessibility::AccessibleHoverText;
use crate::framework::{InputMode, InteractiveTool};
use eframe::egui;
use math_commons::theory::TheoryDescribable;
use math_explorer::ai::transformer::{Decoder, Encoder, Transformer};
use nalgebra::DMatrix;

pub struct AttentionMapsTool {
    q_matrix: DMatrix<f64>,
    k_matrix: DMatrix<f64>,
    v_matrix: DMatrix<f64>,
    output_matrix: DMatrix<f64>,
    attention_weights: DMatrix<f64>,
}

impl Default for AttentionMapsTool {
    fn default() -> Self {
        let seq_len = 4;
        let d_k = 4;

        let mut tool = Self {
            q_matrix: DMatrix::from_element(seq_len, d_k, 0.5),
            k_matrix: DMatrix::from_element(seq_len, d_k, 0.5),
            v_matrix: DMatrix::from_element(seq_len, d_k, 0.5),
            output_matrix: DMatrix::zeros(seq_len, d_k),
            attention_weights: DMatrix::zeros(seq_len, seq_len),
        };
        tool.recalculate();
        tool
    }
}

impl AttentionMapsTool {
    fn recalculate(&mut self) {
        let (output, weights) =
            math_explorer::ai::transformer::attention::scaled_dot_product_attention(
                &self.q_matrix,
                &self.k_matrix,
                &self.v_matrix,
                None,
            );
        self.output_matrix = output;
        self.attention_weights = weights;
    }

    fn draw_matrix_input(ui: &mut egui::Ui, name: &str, matrix: &mut DMatrix<f64>) -> bool {
        let mut changed = false;
        ui.label(name);
        egui::Grid::new(format!("{}_input_grid", name))
            .num_columns(matrix.ncols())
            .show(ui, |ui| {
                for r in 0..matrix.nrows() {
                    for c in 0..matrix.ncols() {
                        let mut val = matrix[(r, c)];
                        if ui
                            .add(
                                egui::DragValue::new(&mut val)
                                    .speed(0.1)
                                    .range(-10.0..=10.0),
                            )
                            .changed()
                        {
                            matrix[(r, c)] = val;
                            changed = true;
                        }
                    }
                    ui.end_row();
                }
            });
        changed
    }

    fn draw_heatmap(ui: &mut egui::Ui, name: &str, matrix: &DMatrix<f64>, input_mode: InputMode) {
        ui.label(name);

        let cell_size = match input_mode {
            InputMode::Touch => 44.0,
            InputMode::Mouse => 40.0,
        };

        let (min_val, max_val) = matrix
            .iter()
            .copied()
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), val| {
                (min.min(val), max.max(val))
            });

        egui::Grid::new(format!("{}_heatmap_grid", name))
            .num_columns(matrix.ncols())
            .spacing([2.0, 2.0])
            .show(ui, |ui| {
                for r in 0..matrix.nrows() {
                    for c in 0..matrix.ncols() {
                        let val = matrix[(r, c)];

                        let normalized = if (max_val - min_val).abs() > 1e-6 {
                            (val - min_val) / (max_val - min_val)
                        } else {
                            0.5
                        };

                        // Color ranges from dark blue (low) to yellow (high)
                        let color = egui::Color32::from_rgb(
                            (normalized * 255.0) as u8,
                            (normalized * 200.0) as u8,
                            ((1.0 - normalized) * 200.0 + 55.0) as u8,
                        );

                        let (rect, _response) =
                            ui.allocate_exact_size(egui::vec2(cell_size, cell_size), egui::Sense::hover());

                        ui.painter().rect_filled(rect, 2.0, color);

                        let text_color = if normalized > 0.5 {
                            egui::Color32::BLACK
                        } else {
                            egui::Color32::WHITE
                        };

                        ui.painter().text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            format!("{:.2}", val),
                            egui::FontId::proportional(12.0),
                            text_color,
                        );
                    }
                    ui.end_row();
                }
            });
    }
}

impl InteractiveTool for AttentionMapsTool {
    fn name(&self) -> &'static str {
        "Attention Maps"
    }

    fn show(&mut self, ctx: &egui::Context) {
        let input_mode = ctx.data(|d| {
            d.get_temp(egui::Id::new("INPUT_MODE"))
                .unwrap_or(InputMode::Mouse)
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let dummy_transformer = Transformer {
                encoder: Encoder::new(1, 4, 1, 4),
                decoder: Decoder::new(1, 4, 1, 4),
            };

            ui.heading("Self-Attention Mechanism")
                .accessible_hover_text(dummy_transformer.theory_description());

            ui.label("Explore how Queries (Q), Keys (K), and Values (V) interact.");

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        let mut changed = false;
                        ui.group(|ui| {
                            changed |=
                                Self::draw_matrix_input(ui, "Queries (Q)", &mut self.q_matrix);
                        });
                        ui.group(|ui| {
                            changed |= Self::draw_matrix_input(ui, "Keys (K)", &mut self.k_matrix);
                        });
                        ui.group(|ui| {
                            changed |=
                                Self::draw_matrix_input(ui, "Values (V)", &mut self.v_matrix);
                        });

                        if changed {
                            self.recalculate();
                        }
                    });

                    ui.separator();

                    ui.vertical(|ui| {
                        ui.group(|ui| {
                            Self::draw_heatmap(
                                ui,
                                "Attention Weights (softmax(Q * K^T / sqrt(d_k)))",
                                &self.attention_weights,
                                input_mode,
                            );
                        });

                        ui.add_space(20.0);

                        ui.group(|ui| {
                            Self::draw_heatmap(ui, "Output (Weights * V)", &self.output_matrix, input_mode);
                        });
                    });
                });
            });
        });
    }
}

// [cite:attention_is_all_you_need_rust]
