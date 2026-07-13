use super::types::Mesh;
use std::fmt::Write;

pub fn export_mesh_to_obj_string(mesh: &Mesh) -> Result<String, std::io::Error> {
    let estimated_size = mesh.vertices.len() * 25 + mesh.normals.len() * 25 + (mesh.indices.len() / 3) * 30 + 100;
    let mut out = String::with_capacity(estimated_size);

    out.push_str("o oxidize_mesh\n");
    for v in &mesh.vertices {
        let _ = writeln!(out, "v {} {} {}", v.x, v.y, v.z);
    }
    for n in &mesh.normals {
        let _ = writeln!(out, "vn {} {} {}", n.x, n.y, n.z);
    }
    for chunk in mesh.indices.chunks(3) {
        if chunk.len() == 3 {
            let i1 = chunk[0] + 1;
            let i2 = chunk[1] + 1;
            let i3 = chunk[2] + 1;
            let _ = writeln!(out, "f {}//{} {}//{} {}//{}", i1, i1, i2, i2, i3, i3);
        }
    }

    Ok(out)
}
