use crate::accessibility::AccessibleHoverText;
use crate::tabs::ai::AiTool;
use eframe::egui;
use math_explorer::ai::reinforcement_learning::{
    algorithms::TabularQAgent,
    grid_world::{GridState, GridWorldEnv, Move},
    MarkovDecisionProcess,
};

pub struct GridWorldTool {
    env: GridWorldEnv,
    agent: TabularQAgent<GridState, Move>,
    current_state: GridState,
    episodes: u32,
    total_reward: f64,
    steps: u32,
}

impl Default for GridWorldTool {
    fn default() -> Self {
        let env = GridWorldEnv {
            width: 5,
            height: 5,
            start: GridState { x: 0, y: 0 },
            goal: GridState { x: 4, y: 4 },
            traps: vec![GridState { x: 2, y: 2 }, GridState { x: 3, y: 2 }],
            gamma: 0.9,
        };
        let agent = TabularQAgent::new(0.1, 0.9, 0.1);
        Self {
            current_state: env.start,
            env,
            agent,
            episodes: 0,
            total_reward: 0.0,
            steps: 0,
        }
    }
}

impl GridWorldTool {
    fn step_agent(&mut self) {
        if self.env.is_terminal(&self.current_state) {
            self.reset_episode();
            return;
        }

        let actions = self.env.actions(&self.current_state);
        if actions.is_empty() {
            return;
        }

        if let Some(action) = self.agent.select_action(&self.current_state, &actions) {
            let next_state = self.env.step(&self.current_state, &action);
            let reward = self.env.reward(&self.current_state, &action, &next_state);
            let next_actions = self.env.actions(&next_state);

            self.agent.update(
                &self.current_state,
                &action,
                reward,
                &next_state,
                &next_actions,
            );

            self.current_state = next_state;
            self.total_reward += reward;
            self.steps += 1;
        }
    }

    fn reset_episode(&mut self) {
        self.current_state = self.env.start;
        self.episodes += 1;
        self.total_reward = 0.0;
        self.steps = 0;
    }
}

impl AiTool for GridWorldTool {
    fn name(&self) -> &'static str {
        "Grid World (RL)"
    }

    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    fn show(&mut self, ctx: &egui::Context) {
        egui::Window::new("Grid World Navigation (Q-Learning)").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .button("▶ Step")
                    .accessible_hover_text("Advance the agent by one step")
                    .clicked()
                {
                    self.step_agent();
                }
                if ui
                    .button("▶ Train (100 Episodes)")
                    .accessible_hover_text("Train the agent for 100 episodes instantly")
                    .clicked()
                {
                    for _ in 0..100 {
                        let mut temp_steps = 0;
                        while !self.env.is_terminal(&self.current_state) && temp_steps < 100 {
                            self.step_agent();
                            temp_steps += 1;
                        }
                        self.reset_episode();
                    }
                }
                if ui
                    .button("↻ Reset Agent")
                    .accessible_hover_text("Clear the Q-table and reset the agent's knowledge")
                    .clicked()
                {
                    self.agent = TabularQAgent::new(0.1, 0.9, 0.1);
                    self.reset_episode();
                    self.episodes = 0;
                }
            });

            ui.horizontal(|ui| {
                ui.label(format!("Episode: {}", self.episodes));
                ui.label(format!("Steps: {}", self.steps));
                ui.label(format!("Reward: {:.2}", self.total_reward));
            });

            let cell_size = 40.0;
            let grid_size = egui::vec2(
                self.env.width as f32 * cell_size,
                self.env.height as f32 * cell_size,
            );

            let (response, painter) = ui.allocate_painter(grid_size, egui::Sense::hover());
            let _ = response
                .clone()
                .accessible_hover_text("Grid World Visualization");
            let rect = response.rect;

            for x in 0..self.env.width {
                for y in 0..self.env.height {
                    let top_left =
                        rect.min + egui::vec2(x as f32 * cell_size, y as f32 * cell_size);
                    let cell_rect =
                        egui::Rect::from_min_size(top_left, egui::vec2(cell_size, cell_size));

                    let state = GridState { x, y };
                    let mut fill_color = egui::Color32::from_gray(200);

                    if state == self.env.goal {
                        fill_color = egui::Color32::GREEN;
                    } else if self.env.traps.contains(&state) {
                        fill_color = egui::Color32::RED;
                    } else if state == self.current_state {
                        fill_color = egui::Color32::BLUE;
                    } else if state == self.env.start {
                        fill_color = egui::Color32::LIGHT_BLUE;
                    }

                    painter.rect_filled(cell_rect, 0.0, fill_color);
                    painter.rect_stroke(
                        cell_rect,
                        0.0,
                        egui::Stroke::new(1.0_f32, egui::Color32::BLACK),
                        egui::StrokeKind::Middle,
                    );
                }
            }
        });
    }
}

// [cite:graph_parameters_rust]
