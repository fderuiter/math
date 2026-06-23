use domain_applied::applied::isosurface::{extract_isosurface, Point3D, VoxelGrid};
use std::time::Instant;

#[test]
fn bench_isosurface_performance() {
    let size = 100; // 100^3 = 1M voxels
    let mut data = Vec::with_capacity(size * size * size);

    // Create a sphere function: x^2 + y^2 + z^2
    let center = size as f32 / 2.0;
    let radius = size as f32 / 3.0;

    for z in 0..size {
        for y in 0..size {
            for x in 0..size {
                let dx = x as f32 - center;
                let dy = y as f32 - center;
                let dz = z as f32 - center;
                let val = (dx * dx + dy * dy + dz * dz).sqrt();
                data.push(val);
            }
        }
    }

    let grid = VoxelGrid::builder()
        .dimensions(size, size, size)
        .data(data)
        .voxel_size(Point3D::new(1.0, 1.0, 1.0))
        .origin(Point3D::new(0.0, 0.0, 0.0))
        .build()
        .unwrap();

    let start = Instant::now();
    let iterations = 10;
    for _ in 0..iterations {
        let _ = extract_isosurface(&grid, radius).unwrap();
    }
    let duration = start.elapsed();

    println!("Total time for {} iterations: {:?}", iterations, duration);
    println!(
        "Average time per iteration: {:?}",
        duration / iterations as u32
    );
}
