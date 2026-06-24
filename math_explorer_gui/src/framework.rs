use eframe::egui;

/// Event context provided to interaction hooks.
pub struct InteractionContext<'a> {
    pub pointer_pos: Option<egui::Pos2>,
    pub delta: egui::Vec2,
    pub is_dragging: bool,
    pub is_clicked: bool,
    pub response: &'a egui::Response,
}

pub trait InteractiveTool {
    fn name(&self) -> &'static str;

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

            let interaction_ctx = InteractionContext {
                pointer_pos: response.interact_pointer_pos(),
                delta: response.drag_delta(),
                is_dragging: response.dragged(),
                is_clicked: response.clicked(),
                response: &response,
            };

            if let Some(_pos) = interaction_ctx.pointer_pos {
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
}

pub struct SimulationFramework {
    pub tools: Vec<Box<dyn InteractiveTool>>,
    pub selected_tool_index: usize,
}

impl SimulationFramework {
    pub fn new(tools: Vec<Box<dyn InteractiveTool>>) -> Self {
        Self {
            tools,
            selected_tool_index: 0,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, id_source: &str) {
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
            });

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
