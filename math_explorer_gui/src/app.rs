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
        let app = Self::default();

        #[cfg(target_os = "macos")]
        {
            use muda::{Menu, Submenu, MenuItem, PredefinedMenuItem, MenuId};
            let menu = Menu::new();
            
            // App Menu
            let app_menu = Submenu::new("Math Explorer", true);
            app_menu.append(&PredefinedMenuItem::quit(None)).unwrap();
            menu.append(&app_menu).unwrap();
            
            // View Menu
            let view_menu = Submenu::new("View", true);
            for (i, tab) in app.tabs.iter().enumerate() {
                let item = MenuItem::new(tab.name(), true, None);
                item.set_id(MenuId::new(format!("tab_{}", i)));
                view_menu.append(&item).unwrap();
            }
            menu.append(&view_menu).unwrap();
            
            let _ = menu.init_for_nsapp();
        }

        app
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

        // Fetch new events
        self.diagnostic_events.extend(global_bus().try_recv_all());

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

        #[cfg(target_os = "macos")]
        {
            if let Ok(event) = muda::MenuEvent::receiver().try_recv() {
                if event.id.0.starts_with("tab_") {
                    if let Ok(idx) = event.id.0[4..].parse::<usize>() {
                        self.selected_tab = idx;
                    }
                }
            }
        }

        // Render Menu Bar (Native fallback for non-macOS or inside the app)
        #[cfg(not(target_os = "macos"))]
        egui::TopBottomPanel::top("main_menu").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("View", |ui| {
                    for (i, tab) in self.tabs.iter().enumerate() {
                        if ui.radio_value(&mut self.selected_tab, i, tab.name()).clicked() {
                            ui.close_menu();
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
