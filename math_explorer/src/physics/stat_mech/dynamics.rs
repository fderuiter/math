use rand::Rng;

/// Simulates a 1D Random Walk.
///
/// # Arguments
/// * `steps` - Number of steps N.
///
/// # Returns
/// * `f64` - Final position.
pub fn random_walk_1d(steps: usize) -> f64 {
    let mut rng = rand::thread_rng();
    let mut position = 0.0;
    for _ in 0..steps {
        if rng.gen_bool(0.5) {
            position += 1.0;
        } else {
            position -= 1.0;
        }
    }
    position
}

/// Estimates the Diffusion Coefficient D.
///
/// Formula: D ~ <x^2> / (2t)
///
/// # Arguments
/// * `num_walks` - Number of walks M to average over.
/// * `time_steps` - Duration t of each walk.
///
/// # Returns
/// * `f64` - Estimated Diffusion Coefficient D.
pub fn estimate_diffusion_coefficient(num_walks: usize, time_steps: usize) -> f64 {
    let mut sum_sq_displacement = 0.0;

    for _ in 0..num_walks {
        let final_pos = random_walk_1d(time_steps);
        sum_sq_displacement += final_pos * final_pos;
    }

    let msd = sum_sq_displacement / num_walks as f64;
    // Time t corresponds to number of steps if we assume dt=1.
    msd / (2.0 * time_steps as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diffusion() {
        // For Random Walk D ~ 0.5 (since dx=1, dt=1).
        let d = estimate_diffusion_coefficient(1000, 100);
        assert!((d - 0.5).abs() < 0.1, "Diffusion coefficient should be approx 0.5. Got {}", d);
    }
}
