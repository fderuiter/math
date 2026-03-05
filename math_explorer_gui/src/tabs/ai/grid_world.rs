use super::AiTool;
use eframe::egui;
use math_explorer::ai::reinforcement_learning::{Action, QLearningAgent, State, TabularQFunction};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GridState(pub i32, pub i32);

impl State for GridState {}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Action for Move {}

pub struct GridWorldTool {
    agent: QLearningAgent<GridState, Move, TabularQFunction<GridState, Move>>,
    current_state: GridState,
    start_state: GridState,
    goal_state: GridState,
    grid_width: i32,
    grid_height: i32,
    obstacles: Vec<GridState>,
    episodes_completed: u32,
    learning_rate: f64,
    discount_factor: f64,
    epsilon: f64,
    steps_in_episode: u32,
    max_steps: u32,
}

impl Default for GridWorldTool {
    fn default() -> Self {
        let learning_rate = 0.1;
        let discount_factor = 0.9;
        let epsilon = 0.1;

        let agent = QLearningAgent::new(learning_rate, discount_factor, epsilon);

        let start_state = GridState(0, 0);
        let goal_state = GridState(9, 9);

        // Simple obstacle wall
        let obstacles = vec![
            GridState(3, 2),
            GridState(3, 3),
            GridState(3, 4),
            GridState(3, 5),
            GridState(3, 6),
            GridState(7, 3),
            GridState(7, 4),
            GridState(7, 5),
            GridState(7, 6),
            GridState(7, 7),
        ];

        Self {
            agent,
            current_state: start_state.clone(),
            start_state,
            goal_state,
            grid_width: 10,
            grid_height: 10,
            obstacles,
            episodes_completed: 0,
            learning_rate,
            discount_factor,
            epsilon,
            steps_in_episode: 0,
            max_steps: 100,
        }
    }
}

impl GridWorldTool {
    fn available_actions(&self, state: &GridState) -> Vec<Move> {
        let mut moves = Vec::new();

        if state.1 > 0 {
            moves.push(Move::Up);
        }
        if state.1 < self.grid_height - 1 {
            moves.push(Move::Down);
        }
        if state.0 > 0 {
            moves.push(Move::Left);
        }
        if state.0 < self.grid_width - 1 {
            moves.push(Move::Right);
        }

        moves
    }

    fn next_state(&self, state: &GridState, action: &Move) -> GridState {
        let mut next = state.clone();
        match action {
            Move::Up => next.1 -= 1,
            Move::Down => next.1 += 1,
            Move::Left => next.0 -= 1,
            Move::Right => next.0 += 1,
        }

        // Keep within bounds
        next.0 = next.0.clamp(0, self.grid_width - 1);
        next.1 = next.1.clamp(0, self.grid_height - 1);

        next
    }

    fn step(&mut self) {
        let available = self.available_actions(&self.current_state);
        if available.is_empty() {
            return;
        }

        if let Some(action) = self.agent.select_action(&self.current_state, &available) {
            let mut next_s = self.next_state(&self.current_state, &action);

            let mut reward = -0.1; // Small penalty for each step

            if self.obstacles.contains(&next_s) {
                // Hit an obstacle, bounce back and get penalty
                reward = -5.0;
                next_s = self.current_state.clone();
            } else if next_s == self.goal_state {
                // Reached goal
                reward = 10.0;
            }

            let next_available = self.available_actions(&next_s);

            self.agent.update(
                &self.current_state,
                &action,
                reward,
                &next_s,
                &next_available,
            );

            self.current_state = next_s;
            self.steps_in_episode += 1;

            if self.current_state == self.goal_state || self.steps_in_episode >= self.max_steps {
                self.reset_episode();
            }
        }
    }

    fn reset_episode(&mut self) {
        self.current_state = self.start_state.clone();
        self.episodes_completed += 1;
        self.steps_in_episode = 0;
    }

    fn train_episodes(&mut self, n: u32) {
        let target_episodes = self.episodes_completed + n;
        while self.episodes_completed < target_episodes {
            self.step();
        }
    }
}

impl AiTool for GridWorldTool {
    fn name(&self) -> &'static str {
        "Grid World"
    }

    fn show(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("grid_world_controls").show(ctx, |ui| {
            ui.heading("Q-Learning Grid World");
            ui.separator();

            let mut changed = false;
            changed |= ui.add(egui::Slider::new(&mut self.learning_rate, 0.01..=1.0).text("Learning Rate")).changed();
            changed |= ui.add(egui::Slider::new(&mut self.discount_factor, 0.1..=1.0).text("Discount Factor")).changed();
            changed |= ui.add(egui::Slider::new(&mut self.epsilon, 0.0..=1.0).text("Exploration (Epsilon)")).changed();

            if changed {
                self.agent = QLearningAgent::new(self.learning_rate, self.discount_factor, self.epsilon);
                self.reset_episode();
                self.episodes_completed = 0; // Restart training on param change
            }

            ui.separator();
            ui.label(format!("Episodes: {}", self.episodes_completed));
            ui.label(format!("Steps this episode: {}", self.steps_in_episode));

            ui.separator();

            if ui.button("Step Agent").clicked() {
                self.step();
            }

            if ui.button("Train 100 Episodes").clicked() {
                self.train_episodes(100);
            }

            if ui.button("Reset Simulation").clicked() {
                *self = Self::default();
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size();
            let (rect, _response) = ui.allocate_exact_size(available_size, egui::Sense::hover());

            let min_dim = rect.width().min(rect.height());
            let cell_size = min_dim / self.grid_width.max(self.grid_height) as f32;

            let grid_offset = egui::vec2(
                (rect.width() - cell_size * self.grid_width as f32) / 2.0,
                (rect.height() - cell_size * self.grid_height as f32) / 2.0,
            );

            let painter = ui.painter();

            for y in 0..self.grid_height {
                for x in 0..self.grid_width {
                    let state = GridState(x, y);

                    let is_obstacle = self.obstacles.contains(&state);
                    let is_goal = state == self.goal_state;
                    let is_start = state == self.start_state;
                    let is_agent = state == self.current_state;

                    let color = if is_agent {
                        egui::Color32::BLUE
                    } else if is_goal {
                        egui::Color32::GREEN
                    } else if is_obstacle {
                        egui::Color32::DARK_GRAY
                    } else if is_start {
                        egui::Color32::LIGHT_BLUE
                    } else {
                        egui::Color32::WHITE
                    };

                    let cell_rect = egui::Rect::from_min_size(
                        rect.min
                            + grid_offset
                            + egui::vec2(x as f32 * cell_size, y as f32 * cell_size),
                        egui::vec2(cell_size, cell_size),
                    );

                    painter.rect_filled(cell_rect, 0.0, color);
                    painter.rect_stroke(
                        cell_rect,
                        0.0,
                        egui::Stroke::new(1.0, egui::Color32::GRAY),
                        egui::StrokeKind::Middle,
                    );

                    // Optional: Draw best Q-value direction
                    if !is_obstacle && !is_goal && !is_agent {
                        let available = self.available_actions(&state);
                        if !available.is_empty() {
                            let best_action = available.iter().max_by(|a, b| {
                                let q_a = self.agent.get_q_value(&state, a);
                                let q_b = self.agent.get_q_value(&state, b);
                                q_a.partial_cmp(&q_b).unwrap_or(std::cmp::Ordering::Equal)
                            });

                            if let Some(best) = best_action {
                                let max_q = self.agent.get_q_value(&state, best);
                                if max_q != 0.0 {
                                    // Only draw if learned
                                    let center = cell_rect.center();
                                    let line_len = cell_size * 0.3;
                                    let offset = match best {
                                        Move::Up => egui::vec2(0.0, -line_len),
                                        Move::Down => egui::vec2(0.0, line_len),
                                        Move::Left => egui::vec2(-line_len, 0.0),
                                        Move::Right => egui::vec2(line_len, 0.0),
                                    };
                                    painter.line_segment(
                                        [center, center + offset],
                                        egui::Stroke::new(2.0, egui::Color32::RED),
                                    );
                                }
                            }
                        }
                    }
                }
            }
        });
    }
}
