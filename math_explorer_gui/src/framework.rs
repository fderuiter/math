use eframe::egui;
use math_commons::theory::TheoryDescribable;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputMode {
    Mouse,
    Touch,
}

/// Event context provided to interaction hooks.
pub struct InteractionContext<'a> {
    pub pointer_pos: Option<egui::Pos2>,
    pub delta: egui::Vec2,
    pub is_dragging: bool,
    pub is_clicked: bool,
    pub response: &'a egui::Response,
    pub input_mode: InputMode,
    pub multi_touch: Option<egui::MultiTouchInfo>,
    pub keys_down: std::collections::HashSet<egui::Key>,
    pub modifiers: egui::Modifiers,
}

pub trait InteractiveTool {
    fn name(&self) -> &'static str;

    /// Optional: Provide theoretical context.
    fn theory(&self) -> Option<&dyn TheoryDescribable> {
        None
    }

    /// Show the tool. Tools can implement this to take full control over rendering.
    /// The default implementation delegates to `show_ui` and then sets up a CentralPanel
    /// with an allocated painter to call the normalized event hooks and `draw`.
    fn show(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left(format!("{}_controls", self.name())).show(ctx, |ui| {
            self.show_ui(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let (response, painter) =
                ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());

            let multi_touch = ui.input(|i| i.multi_touch());

            let input_mode = ctx.data(|d| {
                d.get_temp(egui::Id::new("INPUT_MODE"))
                    .unwrap_or(InputMode::Mouse)
            });

            let keys_down = ui.input(|i| i.keys_down.clone());
            let modifiers = ui.input(|i| i.modifiers);

            let interaction_ctx = InteractionContext {
                pointer_pos: response.interact_pointer_pos(),
                delta: response.drag_delta(),
                is_dragging: response.dragged(),
                is_clicked: response.clicked(),
                response: &response,
                input_mode,
                multi_touch,
                keys_down,
                modifiers,
            };

            if interaction_ctx.response.has_focus() {
                self.on_keyboard(&interaction_ctx);
            }

            if interaction_ctx.multi_touch.is_some() {
                self.on_gesture(&interaction_ctx);
            } else if let Some(_pos) = interaction_ctx.pointer_pos {
                if interaction_ctx.is_dragging {
                    self.on_drag(&interaction_ctx);
                    self.on_brush(&interaction_ctx);
                } else if interaction_ctx.is_clicked {
                    self.on_click(&interaction_ctx);
                } else {
                    self.on_hover(&interaction_ctx);
                }
            }

            self.draw(ui, &response, &painter);
        });
    }

    /// Optional: UI rendering for the tool's specific side-panel controls.
    fn show_ui(&mut self, _ui: &mut egui::Ui) {}

    /// Optional: Context-based drawing.
    fn draw(&mut self, _ui: &mut egui::Ui, _response: &egui::Response, _painter: &egui::Painter) {}

    // Normalized event hooks
    fn on_hover(&mut self, _ctx: &InteractionContext) {}
    fn on_drag(&mut self, _ctx: &InteractionContext) {}
    fn on_click(&mut self, _ctx: &InteractionContext) {}
    fn on_brush(&mut self, _ctx: &InteractionContext) {}
    fn on_gesture(&mut self, _ctx: &InteractionContext) {}
    fn on_keyboard(&mut self, _ctx: &InteractionContext) {}
}

pub struct SimulationFramework {
    pub tools: Vec<Box<dyn InteractiveTool>>,
    pub selected_tool_index: usize,
    pub input_mode: InputMode,
    pub show_theory_portal: bool,
}

impl SimulationFramework {
    pub fn new(tools: Vec<Box<dyn InteractiveTool>>) -> Self {
        Self {
            tools,
            selected_tool_index: 0,
            input_mode: InputMode::Mouse,
            show_theory_portal: false,
        }
    }

    fn show_theory_portal(&self, ctx: &egui::Context, id_source: &str) {
        if !self.show_theory_portal {
            return;
        }
        egui::SidePanel::right(format!("{}_theory_portal", id_source))
            .resizable(true)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    if let Some(tool) = self.tools.get(self.selected_tool_index) {
                        ui.heading(format!("Theory: {}", tool.name()));
                        ui.separator();

                        if let Some(theory) = tool.theory() {
                            // Requirement 5: Screen readers can successfully navigate the theory text and citations
                            use crate::accessibility::AccessibleHoverText;

                            ui.label(theory.theory_description())
                                .accessible_hover_text("Theoretical background description");

                            ui.separator();
                            ui.heading("Citations");
                            ui.label(theory.theory_citation())
                                .accessible_hover_text("Academic citations");

                            let available = theory.available_descriptions();
                            if !available.is_empty() {
                                ui.separator();
                                ui.heading("Additional Context");
                                for (key, desc) in available {
                                    ui.label(format!("{}: {}", key, desc))
                                        .accessible_hover_text(format!(
                                            "Additional context for {}",
                                            key
                                        ));
                                }
                            }
                        } else {
                            ui.label("No theoretical context available for this tool.");
                        }
                    }
                });
            });
    }

    pub fn show(&mut self, ctx: &egui::Context, id_source: &str) {
        // Update global input mode
        ctx.input(|i| {
            if i.any_touches() || i.multi_touch().is_some() {
                self.input_mode = InputMode::Touch;
            } else if i.pointer.is_moving() && !i.any_touches() {
                // Heuristic: if pointer is moving but no touches, likely mouse
                self.input_mode = InputMode::Mouse;
            }
        });
        ctx.data_mut(|d| d.insert_temp(egui::Id::new("INPUT_MODE"), self.input_mode));

        egui::SidePanel::right(format!("{}_tool_selector", id_source))
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Tools");
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (i, tool) in self.tools.iter().enumerate() {
                        if ui
                            .selectable_label(self.selected_tool_index == i, tool.name())
                            .clicked()
                        {
                            self.selected_tool_index = i;
                        }
                    }
                });
                ui.separator();
                ui.checkbox(&mut self.show_theory_portal, "Theory Context Portal");
            });

        self.show_theory_portal(ctx, id_source);

        if let Some(tool) = self.tools.get_mut(self.selected_tool_index) {
            tool.show(ctx);
        } else {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.label("No tool selected");
                });
            });
        }
    }
}

pub struct CoordinateMapper {
    pub screen_rect: egui::Rect,
    pub sim_rect: egui::Rect,
}

#[derive(Clone, Copy, Debug)]
pub struct Camera3D {
    pub pitch: f32,
    pub yaw: f32,
    pub zoom: f32,
}

impl Default for Camera3D {
    fn default() -> Self {
        Self {
            pitch: 0.5,
            yaw: 0.5,
            zoom: 1.0,
        }
    }
}

impl Camera3D {
    pub fn new(pitch: f32, yaw: f32, zoom: f32) -> Self {
        Self { pitch, yaw, zoom }
    }

    pub fn handle_interaction(&mut self, response: &egui::Response, ui: &egui::Ui) {
        let multi_touch = ui.input(|i| i.multi_touch());

        if let Some(touch) = multi_touch {
            if touch.zoom_delta != 1.0 {
                // Dampen zoom velocity
                let zoom_factor = 1.0 + (touch.zoom_delta - 1.0) * 0.5;
                self.zoom *= zoom_factor;
                self.zoom = self.zoom.clamp(0.01, 100.0);
            }
            if touch.translation_delta != egui::Vec2::ZERO {
                self.yaw -= touch.translation_delta.x * 0.01;
                self.pitch -= touch.translation_delta.y * 0.01;
            }
        } else if response.dragged() {
            let delta = response.drag_delta();
            self.yaw -= delta.x * 0.01;
            self.pitch -= delta.y * 0.01;
        }

        let scroll = ui.input(|i| i.raw_scroll_delta.y);
        if scroll != 0.0 && response.hovered() && multi_touch.is_none() {
            self.zoom *= 1.0 + (scroll * 0.001);
            self.zoom = self.zoom.clamp(0.01, 100.0);
        }

        if response.has_focus() {
            let mut yaw_delta = 0.0;
            let mut pitch_delta = 0.0;
            if ui.input(|i| i.key_down(egui::Key::ArrowLeft)) {
                yaw_delta += 0.05;
            }
            if ui.input(|i| i.key_down(egui::Key::ArrowRight)) {
                yaw_delta -= 0.05;
            }
            if ui.input(|i| i.key_down(egui::Key::ArrowUp)) {
                pitch_delta += 0.05;
            }
            if ui.input(|i| i.key_down(egui::Key::ArrowDown)) {
                pitch_delta -= 0.05;
            }
            self.yaw -= yaw_delta;
            self.pitch -= pitch_delta;
        }
    }

    pub fn project(&self, point: &[f64; 3]) -> [f64; 2] {
        let cy = (self.yaw as f64).cos();
        let sy = (self.yaw as f64).sin();
        let cp = (self.pitch as f64).cos();
        let sp = (self.pitch as f64).sin();

        // Apply yaw (rotate around Z)
        let x1 = point[0] * cy - point[1] * sy;
        let y1 = point[0] * sy + point[1] * cy;
        let z1 = point[2];

        // Apply pitch (rotate around X)
        let x2 = x1;
        let y2 = y1 * cp - z1 * sp;

        [x2 * (self.zoom as f64), y2 * (self.zoom as f64)]
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Yaw");
        ui.drag_angle(&mut self.yaw);

        ui.label("Pitch");
        ui.drag_angle(&mut self.pitch);

        ui.add(egui::Slider::new(&mut self.zoom, 0.1..=5.0).text("Zoom"));
    }
}

impl CoordinateMapper {
    pub fn new(screen_rect: egui::Rect, sim_rect: egui::Rect) -> Self {
        Self {
            screen_rect,
            sim_rect,
        }
    }

    pub fn screen_to_sim(&self, pos: egui::Pos2) -> egui::Pos2 {
        let x_norm = (pos.x - self.screen_rect.min.x) / self.screen_rect.width();
        let y_norm = (pos.y - self.screen_rect.min.y) / self.screen_rect.height();

        egui::Pos2::new(
            self.sim_rect.min.x + x_norm * self.sim_rect.width(),
            self.sim_rect.min.y + y_norm * self.sim_rect.height(),
        )
    }

    pub fn sim_to_screen(&self, pos: egui::Pos2) -> egui::Pos2 {
        let x_norm = (pos.x - self.sim_rect.min.x) / self.sim_rect.width();
        let y_norm = (pos.y - self.sim_rect.min.y) / self.sim_rect.height();

        egui::Pos2::new(
            self.screen_rect.min.x + x_norm * self.screen_rect.width(),
            self.screen_rect.min.y + y_norm * self.screen_rect.height(),
        )
    }
}
