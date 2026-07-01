use crate::tabs::ExplorerTab;
use eframe::egui;
use math_explorer::diagnostics::{global_bus, DiagnosticEvent, Severity};

pub struct MathExplorerApp {
    tabs: Vec<Box<dyn ExplorerTab>>,
    selected_tab: usize,
    diagnostic_events: Vec<DiagnosticEvent>,
    show_info: bool,
    show_warnings: bool,
    show_errors: bool,
}

impl Default for MathExplorerApp {
    #[allow(clippy::vec_init_then_push)]
    fn default() -> Self {
        let tabs: Vec<Box<dyn ExplorerTab>> = crate::tabs::instantiate_tabs();

        Self {
            tabs,
            selected_tab: 0,
            diagnostic_events: Vec::new(),
            show_info: true,
            show_warnings: true,
            show_errors: true,
        }
    }
}

impl MathExplorerApp {
    #[allow(clippy::vec_init_then_push)]
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MathExplorerApp {
    #[allow(clippy::too_many_lines)]
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if let Some(msg) = ctx.data_mut(|d| {
            let msg = d.get_temp::<String>(egui::Id::new("aria_live_message"));
            d.remove::<String>(egui::Id::new("aria_live_message"));
            msg
        }) {
            crate::accessibility::announce_status(&msg);
        }

        if let Some(msg) = ctx.data_mut(|d| {
            let msg = d.get_temp::<String>(egui::Id::new("aria_live_assertive_message"));
            d.remove::<String>(egui::Id::new("aria_live_assertive_message"));
            msg
        }) {
            crate::accessibility::announce_status_with_priority(&msg, "assertive");
        }

        // Fetch new events
        let new_events = global_bus().try_recv_all();
        for event in &new_events {
            if matches!(event.severity, Severity::Error | Severity::Fatal) {
                crate::accessibility::announce_status_with_priority(
                    &format!("{}: {}", event.severity, event.message),
                    "assertive",
                );
            }
        }
        self.diagnostic_events.extend(new_events);

        // Issues & Diagnostics Panel
        egui::TopBottomPanel::bottom("issues_panel")
            .resizable(true)
            .min_height(100.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("Issues & Diagnostics");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Clear").clicked() {
                            self.diagnostic_events.clear();
                        }
                        ui.checkbox(&mut self.show_errors, "Errors/Fatal");
                        ui.checkbox(&mut self.show_warnings, "Warnings");
                        ui.checkbox(&mut self.show_info, "Info");
                    });
                });
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for event in &self.diagnostic_events {
                        let show = match event.severity {
                            Severity::Info => self.show_info,
                            Severity::Warning => self.show_warnings,
                            Severity::Error | Severity::Fatal => self.show_errors,
                        };
                        if !show {
                            continue;
                        }

                        let color = match event.severity {
                            Severity::Info => egui::Color32::LIGHT_BLUE,
                            Severity::Warning => egui::Color32::YELLOW,
                            Severity::Error => egui::Color32::RED,
                            Severity::Fatal => egui::Color32::DARK_RED,
                        };

                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new(format!("[{}]", event.severity))
                                        .color(color)
                                        .strong(),
                                );
                                if let Some(thread) = &event.thread_name {
                                    ui.label(
                                        egui::RichText::new(format!("(Thread: {})", thread))
                                            .italics(),
                                    );
                                }
                                ui.label(&event.message);
                            });
                            if !event.metadata.is_empty() {
                                ui.horizontal_wrapped(|ui| {
                                    for (k, v) in &event.metadata {
                                        ui.label(
                                            egui::RichText::new(format!("{}: {}", k, v))
                                                .monospace()
                                                .size(10.0),
                                        );
                                    }
                                });
                            }
                        });
                    }
                });
            });

        // Render Tab Bar
        egui::TopBottomPanel::top("main_menu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Math Explorer");
                ui.separator();
                egui::ScrollArea::horizontal().show(ui, |ui| {
                    for (i, tab) in self.tabs.iter().enumerate() {
                        let name = tab.name();
                        if ui.selectable_label(self.selected_tab == i, name).clicked() {
                            self.selected_tab = i;
                        }
                    }
                });
            });
        });

        // Render Active Tab
        if let Some(tab) = self.tabs.get_mut(self.selected_tab) {
            tab.show(ctx, frame);
        } else {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.label("No module selected.");
                });
            });
        }
    }
}
