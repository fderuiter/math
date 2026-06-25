use math_explorer::pure_math::differential_geometry::surface::ParametricSurface;
use oxidize_core::mesh::{Mesh, Point3D, Triangle};
use std::f64::consts::PI;

pub fn surface_to_mesh(surface: &dyn ParametricSurface, u_res: usize, v_res: usize) -> Mesh {
    let mut triangles = Vec::new();
    
    let u_min = 0.0;
    let u_max = 2.0 * PI;
    let v_min = 0.0;
    let v_max = 2.0 * PI;
    
    for i in 0..u_res {
        for j in 0..v_res {
            let u1 = u_min + (u_max - u_min) * (i as f64 / u_res as f64);
            let u2 = u_min + (u_max - u_min) * ((i + 1) as f64 / u_res as f64);
            let v1 = v_min + (v_max - v_min) * (j as f64 / v_res as f64);
            let v2 = v_min + (v_max - v_min) * ((j + 1) as f64 / v_res as f64);
            
            let p00 = surface.position(u1, v1);
            let p10 = surface.position(u2, v1);
            let p01 = surface.position(u1, v2);
            let p11 = surface.position(u2, v2);
            
            let compute_normal = |u, v| {
                let ru = surface.partial_u(u, v);
                let rv = surface.partial_v(u, v);
                ru.cross(&rv).normalize()
            };
            
            let n00 = compute_normal(u1, v1);
            let n10 = compute_normal(u2, v1);
            let n01 = compute_normal(u1, v2);
            let n11 = compute_normal(u2, v2);
            
            let to_pt = |p: nalgebra::Point3<f64>| Point3D::new(p.x as f32, p.y as f32, p.z as f32);
            let to_n = |n: nalgebra::Vector3<f64>| Point3D::new(n.x as f32, n.y as f32, n.z as f32);
            
            // Triangle 1: (u1, v1), (u2, v1), (u1, v2)
            triangles.push(Triangle {
                v1: to_pt(p00), v2: to_pt(p10), v3: to_pt(p01),
                n1: to_n(n00), n2: to_n(n10), n3: to_n(n01),
            });
            
            // Triangle 2: (u2, v1), (u2, v2), (u1, v2)
            triangles.push(Triangle {
                v1: to_pt(p10), v2: to_pt(p11), v3: to_pt(p01),
                n1: to_n(n10), n2: to_n(n11), n3: to_n(n01),
            });
        }
    }
    
    Mesh { triangles }
}
