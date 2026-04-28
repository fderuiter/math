use crate::tabs::ai::AiTool;
use eframe::egui;
use math_explorer::ai::reinforcement_learning::{
    algorithms::TabularQAgent, Action, MarkovDecisionProcess, State,
};
use std::hash::Hash;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GridState {
    pub x: i32,
    pub y: i32,
}

impl State for GridState {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Action for Move {}

pub struct GridWorldEnv {
    pub width: i32,
    pub height: i32,
    pub goal: GridState,
    pub start: GridState,
    pub traps: Vec<GridState>,
    pub gamma: f64,
}

impl GridWorldEnv {
    /// Applies an action and returns the next state, executing the environment transition logic.
    pub fn step(&self, current_state: &GridState, action: &Move) -> GridState {
        let mut expected_next = *current_state;
        match action {
            Move::Up => expected_next.y -= 1,
            Move::Down => expected_next.y += 1,
            Move::Left => expected_next.x -= 1,
            Move::Right => expected_next.x += 1,
        }

        let is_valid = expected_next.x >= 0
            && expected_next.x < self.width
            && expected_next.y >= 0
            && expected_next.y < self.height;

        if is_valid {
            expected_next
        } else {
            *current_state
        }
    }
}

impl MarkovDecisionProcess for GridWorldEnv {
    type S = GridState;
    type A = Move;

    fn transition_probability(
        &self,
        next_state: &Self::S,
        current_state: &Self::S,
        action: &Self::A,
    ) -> f64 {
        // Simplified deterministic transition for the tool
        let actual_next = self.step(current_state, action);
        if *next_state == actual_next {
            1.0
        } else {
            0.0
        }
    }

    fn reward(&self, _current_state: &Self::S, _action: &Self::A, next_state: &Self::S) -> f64 {
        if *next_state == self.goal {
            10.0
        } else if self.traps.contains(next_state) {
            -10.0
        } else {
            -0.1
        }
    }

    fn actions(&self, state: &Self::S) -> Vec<Self::A> {
        if self.is_terminal(state) {
            vec![]
        } else {
            vec![Move::Up, Move::Down, Move::Left, Move::Right]
        }
    }

    fn discount_factor(&self) -> f64 {
        self.gamma
    }

    fn is_terminal(&self, state: &Self::S) -> bool {
        *state == self.goal || self.traps.contains(state)
    }
}

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

    fn show(&mut self, ctx: &egui::Context) {
        egui::Window::new("Grid World Navigation (Q-Learning)").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Step").clicked() {
                    self.step_agent();
                }
                if ui.button("Train (100 Episodes)").clicked() {
                    for _ in 0..100 {
                        let mut temp_steps = 0;
                        while !self.env.is_terminal(&self.current_state) && temp_steps < 100 {
                            self.step_agent();
                            temp_steps += 1;
                        }
                        self.reset_episode();
                    }
                }
                if ui.button("Reset Agent").clicked() {
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
                        egui::Stroke::new(1.0, egui::Color32::BLACK),
                        egui::StrokeKind::Middle,
                    );
                }
            }
        });
    }
}
