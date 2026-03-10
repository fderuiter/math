use crate::tabs::ai::AiTool;
use eframe::egui;
use egui_plot::{MarkerShape, Plot, PlotPoint, PlotPoints, Points, Text};
use math_explorer::ai::transformer::attention::scaled_dot_product_attention;
use nalgebra::DMatrix;

pub struct AttentionMapsTool {
    seq_len: usize,
    d_k: usize,
    q_data: Vec<f64>,
    k_data: Vec<f64>,
    v_data: Vec<f64>,
}

impl Default for AttentionMapsTool {
    fn default() -> Self {
        let seq_len = 5;
        let d_k = 4;
        let n_elements = seq_len * d_k;
        Self {
            seq_len,
            d_k,
            q_data: vec![1.0; n_elements],
            k_data: vec![1.0; n_elements],
            v_data: vec![1.0; n_elements],
        }
    }
}

impl AiTool for AttentionMapsTool {
    fn name(&self) -> &'static str {
        "Attention Maps"
    }

    fn show(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("attention_maps_controls").show(ctx, |ui| {
            ui.heading("Settings");

            let mut changed = false;
            changed |= ui.add(egui::Slider::new(&mut self.seq_len, 2..=10).text("Sequence Length")).changed();
            changed |= ui.add(egui::Slider::new(&mut self.d_k, 2..=8).text("Embedding Dim (d_k)")).changed();

            // Resize data vectors if dimensions changed
            if changed {
                let expected_size = self.seq_len * self.d_k;
                self.q_data.resize(expected_size, 1.0);
                self.k_data.resize(expected_size, 1.0);
                self.v_data.resize(expected_size, 1.0);
            }

            ui.separator();
            ui.heading("Inputs (Q, K)");
            ui.label("Randomize or set values manually to see how attention weights change.");

            if ui.button("Randomize Inputs").clicked() {
                use rand::Rng;
                let mut rng = rand::thread_rng();
                for x in self.q_data.iter_mut() {
                    *x = rng.r#gen_range(-1.0..=1.0);
                }
                for x in self.k_data.iter_mut() {
                    *x = rng.r#gen_range(-1.0..=1.0);
                }
            }

            ui.separator();
            ui.label("Values (V) matrix is mostly irrelevant for visualizing attention weights.");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Self-Attention Heatmap");
            ui.label("Visualizing the attention weights: softmax(Q * K^T / sqrt(d_k))");
            ui.add_space(10.0);

            // Need to convert flat Vecs to column-major or row-major for DMatrix.
            // from_vec creates column-major by default. Let's assume it's row-major
            // so we'll use from_row_iterator.
            let q = DMatrix::from_row_iterator(self.seq_len, self.d_k, self.q_data.clone());
            let k = DMatrix::from_row_iterator(self.seq_len, self.d_k, self.k_data.clone());
            let v = DMatrix::from_row_iterator(self.seq_len, self.d_k, self.v_data.clone());

            let (_, attention_weights) = scaled_dot_product_attention(&q, &k, &v, None);

            // To plot heatmap, we will map weight to a color for a square marker
            Plot::new("attention_heatmap")
                .view_aspect(1.0)
                .show_axes([false, false])
                .show_grid(false)
                .allow_drag(false)
                .allow_zoom(false)
                .allow_scroll(false)
                .show(ui, |plot_ui| {
                    for row in 0..self.seq_len {
                        for col in 0..self.seq_len {
                            let weight = attention_weights[(row, col)];
                            // clamp weight to [0, 1] just in case
                            let weight = weight.clamp(0.0, 1.0);

                            // Yellow/Orange heatmap color map
                            let red = 255;
                            let green = (255.0 * (1.0 - weight)) as u8;
                            let blue = (50.0 * (1.0 - weight)) as u8;
                            let color = egui::Color32::from_rgb(red, green, blue);

                            // Invert Y axis so token 0 is at top
                            let y = (self.seq_len - 1 - row) as f64;
                            let x = col as f64;

                            let points = Points::new(
                                format!("cell_{}_{}", row, col),
                                PlotPoints::new(vec![[x, y]])
                            )
                            .color(color)
                            .shape(MarkerShape::Square)
                            .radius(30.0); // Size of the square

                            plot_ui.points(points);

                            // Draw the weight text over the cell
                            plot_ui.text(Text::new(format!("text_{}_{}", row, col),
                                PlotPoint::new(x, y),
                                format!("{:.2}", weight)
                            ).color(if weight > 0.5 { egui::Color32::WHITE } else { egui::Color32::BLACK }));
                        }
                    }
                });

            ui.add_space(20.0);
            ui.label("Tokens (Queries) on Y-axis attending to Tokens (Keys) on X-axis.");
            ui.label("Top row = Token 0 Query, Left column = Token 0 Key");
        });
    }
}
