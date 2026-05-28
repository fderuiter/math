use math_explorer_core::{
    discovery::{GenericSimulation, Parameter, ParameterValue},
    state::StateData,
};

#[derive(Clone)]
pub struct GameOfLifeConfig {
    pub width: usize,
    pub height: usize,
}

pub struct GameOfLifeState {
    pub grid: Vec<i64>, // Discrete state, no VectorOperations
}

pub struct GameOfLifeSim {
    width: usize,
    height: usize,
    grid: Vec<i64>,
}

impl GameOfLifeSim {
    pub fn new(width: usize, height: usize) -> Self {
        let mut grid = vec![0; width * height];
        // simple glider
        if width > 3 && height > 3 {
            grid[1 * width + 2] = 1;
            grid[2 * width + 3] = 1;
            grid[3 * width + 1] = 1;
            grid[3 * width + 2] = 1;
            grid[3 * width + 3] = 1;
        }
        Self { width, height, grid }
    }
}

impl GenericSimulation for GameOfLifeSim {
    fn name(&self) -> &str { "Game of Life" }
    fn description(&self) -> &str { "Discrete cellular automata" }
    
    fn get_parameters(&self) -> Vec<Parameter> {
        vec![]
    }
    
    fn set_parameter(&mut self, _name: &str, _value: ParameterValue) {}
    
    fn reset(&mut self) {
        *self = Self::new(self.width, self.height);
    }
    
    fn step(&mut self, _dt: f64, _input: Option<f64>) {
        let mut next = self.grid.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let mut alive_neighbors = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 { continue; }
                        let nx = (x as isize + dx).rem_euclid(self.width as isize) as usize;
                        let ny = (y as isize + dy).rem_euclid(self.height as isize) as usize;
                        alive_neighbors += self.grid[ny * self.width + nx];
                    }
                }
                let idx = y * self.width + x;
                if self.grid[idx] == 1 {
                    next[idx] = if alive_neighbors == 2 || alive_neighbors == 3 { 1 } else { 0 };
                } else {
                    next[idx] = if alive_neighbors == 3 { 1 } else { 0 };
                }
            }
        }
        self.grid = next;
    }
    
    fn get_state(&self) -> StateData {
        StateData::Discrete(self.grid.clone())
    }
}
