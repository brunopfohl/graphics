use crate::math::{Vec2, trig};

#[derive(Debug, Clone)]
pub struct Transform2D {
    pub position: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}

impl Transform2D {
    pub fn new() -> Self {
        Transform2D {
            position: Vec2::zero(),
            rotation: 0.0,
            scale: Vec2::new(1.0, 1.0),
        }
    }

    pub fn translate(&mut self, offset: Vec2) {
        self.position = self.position + offset;
    }

    pub fn rotate(&mut self, angle_degrees: f32) {
        self.rotation += trig::to_radians(angle_degrees);
    }

    pub fn set_scale(&mut self, scale: Vec2) {
        self.scale = scale;
    }

    pub fn transform_point(&self, point: Vec2) -> Vec2 {
        // Scale
        let scaled = Vec2::new(point.x * self.scale.x, point.y * self.scale.y);
        
        // Rotate
        let cos_r = self.rotation.cos();
        let sin_r = self.rotation.sin();
        let rotated = Vec2::new(
            scaled.x * cos_r - scaled.y * sin_r,
            scaled.x * sin_r + scaled.y * cos_r
        );
        
        // Translate
        rotated + self.position
    }
}
