use rand::Rng;
use std::f32::consts::TAU;

/// Calculates R0 for a heterogeneous network.
///
/// $R_0 = \frac{\beta}{\gamma} \frac{\langle k^2 \rangle - \langle k \rangle}{\langle k \rangle}$
pub fn heterogeneous_r0(beta: f64, gamma: f64, mean_degree: f64, degree_variance: f64) -> f64 {
    if mean_degree == 0.0 || gamma == 0.0 {
        return 0.0;
    }

    // Var(k) = E[k^2] - (E[k])^2
    // E[k^2] = Var(k) + (E[k])^2

    let mean_k_sq = degree_variance + mean_degree.powi(2);
    let factor = (mean_k_sq - mean_degree) / mean_degree;

    (beta / gamma) * factor
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum NodeState {
    Susceptible,
    Infected,
    Recovered,
}

pub struct NetworkEpidemicModel {
    pub num_nodes: usize,
    pub states: Vec<NodeState>,
    pub positions: Vec<[f32; 2]>,
    pub adjacency: Vec<Vec<usize>>,
    pub beta: f64,
    pub gamma: f64,
}

impl NetworkEpidemicModel {
    /// Creates a new network epidemic model and initializes it with a geometric graph.
    ///
    /// This method uses the default thread-local RNG. For deterministic behavior,
    /// use `new_with_rng`.
    pub fn new(num_nodes: usize, beta: f64, gamma: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self::new_with_rng(num_nodes, beta, gamma, &mut rng)
    }

    /// Creates a new network epidemic model and initializes it with a geometric graph
    /// using an injected RNG.
    pub fn new_with_rng<R: Rng>(num_nodes: usize, beta: f64, gamma: f64, rng: &mut R) -> Self {
        let mut model = Self {
            num_nodes,
            states: vec![NodeState::Susceptible; num_nodes],
            positions: vec![[0.0, 0.0]; num_nodes],
            adjacency: vec![vec![]; num_nodes],
            beta,
            gamma,
        };
        model.initialize_geometric_graph_with_rng(rng);
        model
    }

    /// Initializes a geometric graph. This method uses the default thread-local RNG.
    pub fn initialize_geometric_graph(&mut self) {
        let mut rng = rand::thread_rng();
        self.initialize_geometric_graph_with_rng(&mut rng);
    }

    /// Initializes a geometric graph using an injected RNG.
    pub fn initialize_geometric_graph_with_rng<R: Rng>(&mut self, rng: &mut R) {
        self.states = vec![NodeState::Susceptible; self.num_nodes];
        self.positions = vec![[0.0, 0.0]; self.num_nodes];
        self.adjacency = vec![vec![]; self.num_nodes];

        // Random geometric graph
        let radius = 200.0;
        for i in 0..self.num_nodes {
            let angle = rng.r#gen_range(0.0..TAU);
            let r = radius * rng.r#gen_range(0.0f32..1.0f32).sqrt();
            self.positions[i] = [r * angle.cos(), r * angle.sin()];
        }

        let connection_radius = 60.0;
        for i in 0..self.num_nodes {
            for j in (i + 1)..self.num_nodes {
                let dx = self.positions[i][0] - self.positions[j][0];
                let dy = self.positions[i][1] - self.positions[j][1];
                let dist = (dx * dx + dy * dy).sqrt();
                if dist < connection_radius {
                    self.adjacency[i].push(j);
                    self.adjacency[j].push(i);
                }
            }
        }

        // Start with one infected
        if self.num_nodes > 0 {
            let start_idx = rng.r#gen_range(0..self.num_nodes);
            self.states[start_idx] = NodeState::Infected;
        }
    }

    /// Steps the simulation forward using the default thread-local RNG.
    pub fn step(&mut self) {
        let mut rng = rand::thread_rng();
        self.step_with_rng(&mut rng);
    }

    /// Steps the simulation forward using an injected RNG.
    pub fn step_with_rng<R: Rng>(&mut self, rng: &mut R) {
        let mut next_states = self.states.clone();

        for (i, next_state) in next_states.iter_mut().enumerate().take(self.num_nodes) {
            match self.states[i] {
                NodeState::Susceptible => {
                    // Check infected neighbors
                    let infected_neighbors = self.adjacency[i]
                        .iter()
                        .filter(|&&j| self.states[j] == NodeState::Infected)
                        .count();
                    for _ in 0..infected_neighbors {
                        if rng.r#gen::<f64>() < self.beta {
                            *next_state = NodeState::Infected;
                            break;
                        }
                    }
                }
                NodeState::Infected => {
                    if rng.r#gen::<f64>() < self.gamma {
                        *next_state = NodeState::Recovered;
                    }
                }
                NodeState::Recovered => {}
            }
        }
        self.states = next_states;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heterogeneous_r0() {
        let beta = 0.5;
        let gamma = 0.1;
        // Homogeneous network: Variance = 0. Factor = (k^2 - k)/k = (k^2 - k)/k = k - 1?
        // Wait, if Var=0, then <k^2> = <k>^2.
        // Factor = (<k>^2 - <k>)/<k> = <k> - 1.
        // Standard formula usually assumes contact rate is proportional to k.

        // Using provided formula:
        let mean_k = 4.0;
        let var_k = 0.0;
        let r0 = heterogeneous_r0(beta, gamma, mean_k, var_k);

        // R0 = (beta/gamma) * (16 - 4)/4 = 5 * 3 = 15.
        assert!((r0 - 15.0).abs() < 1e-6);
    }
}
