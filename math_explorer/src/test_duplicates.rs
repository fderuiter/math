pub trait NewAwesomeSolver {
    fn solve(&self);
}

pub struct Camera {
    fovy: f32,
    aspect: f32,
}
