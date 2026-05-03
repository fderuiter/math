use math_explorer::applied::isosurface::{Point3D, VoxelGrid};

#[test]
fn test_voxelgrid_panic_on_malformed_data() {
    // Create a VoxelGrid with insufficient data buffer
    // 10x10x10 = 1000 voxels required, but only 1 provided.
    let grid = VoxelGrid {
        width: 10,
        height: 10,
        depth: 10,
        data: vec![0.0; 1], // MALFORMED
        voxel_size: Point3D::new(1.0, 1.0, 1.0),
        origin: Point3D::new(0.0, 0.0, 0.0),
    };

    // This call should NOT panic anymore. It should return 0.0.
    let val = grid.get(5, 5, 5);
    assert_eq!(val, 0.0);
}
