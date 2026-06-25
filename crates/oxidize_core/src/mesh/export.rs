use super::types::Mesh;
use obj_exporter::{Geometry, ObjSet, Object, Shape, Vertex};

pub fn mesh_to_obj_set(mesh: &Mesh) -> ObjSet {
    let mut vertices = Vec::new();
    let mut normals = Vec::new();
    let mut shapes = Vec::new();

    for (i, tri) in mesh.triangles.iter().enumerate() {
        let v_idx = i * 3;

        vertices.push(Vertex {
            x: tri.v1.x as f64,
            y: tri.v1.y as f64,
            z: tri.v1.z as f64,
        });
        vertices.push(Vertex {
            x: tri.v2.x as f64,
            y: tri.v2.y as f64,
            z: tri.v2.z as f64,
        });
        vertices.push(Vertex {
            x: tri.v3.x as f64,
            y: tri.v3.y as f64,
            z: tri.v3.z as f64,
        });

        normals.push(Vertex {
            x: tri.n1.x as f64,
            y: tri.n1.y as f64,
            z: tri.n1.z as f64,
        });
        normals.push(Vertex {
            x: tri.n2.x as f64,
            y: tri.n2.y as f64,
            z: tri.n2.z as f64,
        });
        normals.push(Vertex {
            x: tri.n3.x as f64,
            y: tri.n3.y as f64,
            z: tri.n3.z as f64,
        });

        shapes.push(Shape {
            primitive: obj_exporter::Primitive::Triangle(
                (v_idx, None, Some(v_idx)),
                (v_idx + 1, None, Some(v_idx + 1)),
                (v_idx + 2, None, Some(v_idx + 2)),
            ),
            groups: vec![],
            smoothing_groups: vec![],
        });
    }

    let geometry = Geometry {
        material_name: None,
        shapes,
    };

    let object = Object {
        name: "oxidize_mesh".to_string(),
        vertices,
        tex_vertices: vec![],
        normals,
        geometry: vec![geometry],
    };

    ObjSet {
        material_library: None,
        objects: vec![object],
    }
}

pub fn export_mesh_to_obj_string(mesh: &Mesh) -> Result<String, std::io::Error> {
    let obj_set = mesh_to_obj_set(mesh);
    let mut buffer = Vec::new();
    obj_exporter::export(&obj_set, &mut buffer).map_err(std::io::Error::other)?;
    Ok(String::from_utf8_lossy(&buffer).to_string())
}
