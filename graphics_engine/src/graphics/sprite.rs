use crate::graphics::transform2d::Transform2D;
use crate::math::Vec2;

#[derive(Debug)]
pub struct Sprite {
    pub transform: Transform2D,
    pub width: f32,
    pub height: f32,
    pub vertices: Vec<Vec2>,
}

impl Sprite {
    pub fn new(width: f32, height: f32) -> Self {
        let half_w = width / 2.0;
        let half_h = height / 2.0;
        
        Sprite {
            transform: Transform2D::new(),
            width,
            height,
            vertices: vec![
                Vec2::new(-half_w, -half_h),
                Vec2::new(half_w, -half_h),
                Vec2::new(half_w, half_h),
                Vec2::new(-half_w, half_h),
            ],
        }
    }

    pub fn get_transformed_vertices(&self) -> Vec<Vec2> {
        self.vertices
            .iter()
            .map(|v| self.transform.transform_point(*v))
            .collect()
    }
}
