use super::{Action, MarkovDecisionProcess, State};

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
        let actual_next = self.step(current_state, action);
        if *next_state == actual_next { 1.0 } else { 0.0 }
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
