use math_explorer::biology::diffusion::FiniteDifference1D;
use math_explorer::biology::reaction_diffusion::{
    ChemicalState, ReactionDiffusionSystem, ReactionModel,
};
use math_explorer::pure_math::analysis::ode::solvers::Euler;
use std::time::Instant;

struct LinearDecay {
    decay_rate: f64,
}

impl ReactionModel for LinearDecay {
    fn reaction(&self, concentrations: &[f64], rates: &mut [f64]) {
        for (c, r) in concentrations.iter().zip(rates.iter_mut()) {
            *r = -self.decay_rate * c;
        }
    }
}

#[test]
fn bench_reaction_diffusion_performance() {
    let num_species = 50;
    let grid_size = 5000;
    let steps = 50;
    let dt = 0.01;

    let reaction = LinearDecay { decay_rate: 0.1 };
    let diffusion = FiniteDifference1D::new(0.1);
    let diffusion_coeffs = vec![0.1; num_species];

    let mut system = ReactionDiffusionSystem::new(
        num_species,
        grid_size,
        reaction,
        diffusion,
        diffusion_coeffs,
    );

    // Initialize with some data
    for s in 0..num_species {
        let species = system.state.species_mut(s);
        for i in 0..grid_size {
            species[i] = (i as f64).sin();
        }
    }

    let start = Instant::now();
    for _ in 0..steps {
        system.step(dt);
    }
    let duration = start.elapsed();

    println!(
        "Benchmark: {} species, {} grid points, {} steps",
        num_species, grid_size, steps
    );
    println!("Time: {:?}", duration);
    println!("Time per step: {:?}", duration / steps as u32);
}
