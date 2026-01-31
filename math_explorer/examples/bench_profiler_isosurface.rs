use math_explorer::applied::isosurface::{Point3D, VoxelGrid, extract_isosurface};
use std::time::Instant;

fn main() {
    println!("⏱️  Profiler Benchmark: Isosurface Extraction");

    let size = 200; // 8M voxels
    println!("Grid size: {}x{}x{}", size, size, size);

    let mut data = Vec::with_capacity(size * size * size);
    let center = size as f32 / 2.0;
    let radius = size as f32 / 3.0;
    let radius_sq = radius * radius;

    // Generate Sphere SDF
    for z in 0..size {
        for y in 0..size {
            for x in 0..size {
                let dx = x as f32 - center;
                let dy = y as f32 - center;
                let dz = z as f32 - center;
                let dist_sq = dx * dx + dy * dy + dz * dz;
                data.push(dist_sq - radius_sq);
            }
        }
    }

    let grid = VoxelGrid {
        width: size,
        height: size,
        depth: size,
        data,
        voxel_size: Point3D::new(1.0, 1.0, 1.0),
        origin: Point3D::new(0.0, 0.0, 0.0),
    };

    // Warmup
    println!("Warming up...");
    for _ in 0..5 {
        let _ = extract_isosurface(&grid, 0.0);
    }

    println!("Benchmarking...");
    let iterations = 20;
    let start = Instant::now();
    let mut total_tris = 0;

    for _ in 0..iterations {
        let mesh = extract_isosurface(&grid, 0.0).expect("Failed to extract surface");
        total_tris += mesh.triangles.len();
    }
    let duration = start.elapsed();
    let avg_time = duration / iterations as u32;

    println!(
        "Processed {} iterations in {:?}",
        iterations,
        duration
    );
    println!(
        "Average time per iteration: {:.2} ms",
        avg_time.as_secs_f64() * 1000.0
    );
    println!("Total triangles generated: {}", total_tris);
}
