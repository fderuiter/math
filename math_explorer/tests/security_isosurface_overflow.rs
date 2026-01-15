use math_explorer::applied::isosurface::{Point3D, VoxelGrid, extract_isosurface};

#[test]
fn test_security_buffer_overflow_prevention() {
    // Create a VoxelGrid with insufficient data buffer
    // Dimensions 10x10x10 require 1000 elements.
    // We only provide 10.
    let grid = VoxelGrid {
        width: 10,
        height: 10,
        depth: 10,
        data: vec![0.0; 10], // MALFORMED: Too small!
        voxel_size: Point3D::new(1.0, 1.0, 1.0),
        origin: Point3D::new(0.0, 0.0, 0.0),
    };

    // This call triggers unsafe code blocks that assume data.len() is sufficient.
    // Without a fix, this causes Undefined Behavior (segfault or out-of-bounds read).
    // With the fix, it should return an Err.
    let result = extract_isosurface(&grid, 0.5);

    // Assert that we get an error, not a crash
    assert!(
        result.is_err(),
        "Should return error for insufficient buffer size"
    );
    let err_msg = result.err().unwrap();
    assert!(
        err_msg.contains("Data buffer size mismatch"),
        "Error message should mention buffer mismatch. Got: {}",
        err_msg
    );
}
