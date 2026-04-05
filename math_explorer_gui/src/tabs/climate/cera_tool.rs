use crate::tabs::climate::ClimateTool;
use eframe::egui;
use math_explorer::climate::{
    cera::Cera,
    config::CeraConfig,
};

pub struct CeraTool {
    config: CeraConfig,
    status: String,
    // Note: To keep the UI responsive, real training should happen in a background thread.
    // Since math_explorer_gui shouldn't crash with unwraps, we store the result.
    model_initialized: bool,
}

impl Default for CeraTool {
    fn default() -> Self {
        Self {
            config: CeraConfig {
                learning_rate: 0.001,
                lambda_pred: 1.0,
                lambda_emd: 0.1,
                epochs: 10,
                batch_size: 32,
                in_channels: 1,
                latent_channels: 16,
                aligned_channels: 8,
                num_levels: 10,
                output_size: 1,
            },
            status: "Waiting to initialize model...".to_string(),
            model_initialized: false,
        }
    }
}

impl ClimateTool for CeraTool {
    fn name(&self) -> &'static str {
        "CERA Model"
    }

    fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("Coupled Energy-Resource-Atmosphere (CERA)");
        ui.separator();

        ui.label("Configure CERA hyperparameters:");

        egui::Grid::new("cera_config_grid").num_columns(2).show(ui, |ui| {
            ui.label("Learning Rate:");
            ui.add(egui::Slider::new(&mut self.config.learning_rate, 0.0001..=0.1).logarithmic(true));
            ui.end_row();

            ui.label("Lambda Pred:");
            ui.add(egui::Slider::new(&mut self.config.lambda_pred, 0.1..=10.0));
            ui.end_row();

            ui.label("Lambda EMD:");
            ui.add(egui::Slider::new(&mut self.config.lambda_emd, 0.01..=2.0));
            ui.end_row();

            ui.label("Epochs:");
            ui.add(egui::Slider::new(&mut self.config.epochs, 1..=100));
            ui.end_row();

            ui.label("Batch Size:");
            ui.add(egui::Slider::new(&mut self.config.batch_size, 1..=128));
            ui.end_row();

            ui.label("Latent Channels:");
            ui.add(egui::Slider::new(&mut self.config.latent_channels, 4..=64));
            ui.end_row();

            ui.label("Aligned Channels:");
            ui.add(egui::Slider::new(&mut self.config.aligned_channels, 2..=32));
            ui.end_row();
        });

        ui.add_space(10.0);

        if ui.button("Initialize Model").clicked() {
            match Cera::new(self.config.clone()) {
                Ok(_) => {
                    self.status = "Model initialized successfully.".to_string();
                    self.model_initialized = true;
                }
                Err(e) => {
                    self.status = format!("Failed to initialize: {}", e);
                    self.model_initialized = false;
                }
            }
        }

        ui.add_space(10.0);

        ui.label("Status:");
        ui.label(
            egui::RichText::new(&self.status)
                .color(if self.model_initialized { egui::Color32::GREEN } else { egui::Color32::RED })
        );

        if self.model_initialized {
            ui.add_space(10.0);
            ui.label("Model is ready. (Prediction and training would be executed here).");
        }
    }
}
