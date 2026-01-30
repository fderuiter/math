use math_explorer::ai::gaussian_splatting::rendering::{
    blend_gaussians, blend_gaussians_block_2x2,
};
use math_explorer::ai::gaussian_splatting::structs::Gaussian2D;
use nalgebra::{Matrix2, Point2, Vector3};
use std::time::Instant;

fn main() {
    let mut gaussians = Vec::new();
    let num_gaussians = 100;

    // Create gaussians
    // We use identity covariance for simplicity as it's invertible.
    for i in 0..num_gaussians {
        gaussians.push(Gaussian2D {
            mean: Point2::new((i as f64) % 10.0, (i as f64) / 10.0),
            conic: Matrix2::from_diagonal_element(-0.5),
            opacity: 0.5,
            color: Vector3::new(1.0, 0.0, 0.0),
            depth: 1.0,
        });
    }

    let width = 100;
    let height = 100;

    // Bench AoS (with inlining)
    let start = Instant::now();
    let mut black_hole = Vector3::zeros();
    for _ in 0..100 {
        for y in 0..height {
            for x in 0..width {
                let p = Point2::new(x as f64 / 10.0, y as f64 / 10.0);
                black_hole += blend_gaussians(&gaussians, &p);
            }
        }
    }
    let duration = start.elapsed();
    println!("Time AoS: {:?}", duration);

    // Bench Block 2x2
    // We must iterate in 2x2 blocks.
    // Assuming width/height are even (100).
    let start_block = Instant::now();
    let mut black_hole_block = Vector3::zeros();

    // We iterate in steps of 2
    for _ in 0..100 {
        for y in (0..height).step_by(2) {
            for x in (0..width).step_by(2) {
                let p = Point2::new(x as f64 / 10.0, y as f64 / 10.0);
                // Stride in coordinate space is 0.1 (since x is divided by 10)
                let results = blend_gaussians_block_2x2(&gaussians, &p, 0.1, 0.1);

                black_hole_block += results[0] + results[1] + results[2] + results[3];
            }
        }
    }
    let duration_block = start_block.elapsed();
    println!("Time Block 2x2: {:?}", duration_block);

    // Verify
    if (black_hole.x - black_hole_block.x).abs() > 1e-3 {
        println!(
            "Mismatch! AoS: {:?}, Block: {:?}",
            black_hole, black_hole_block
        );
    } else {
        println!("Results match.");
    }

    // Prevent optimization
    if black_hole.x > 1e9 {
        println!("Overflow");
    }
}
