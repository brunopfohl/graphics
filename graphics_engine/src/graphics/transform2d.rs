use crate::math::Vec2;

#[derive(Clone, Debug)]
pub struct Transform2D {
    position: Vec2,
    rotation: f32,  // in degrees
    scale: Vec2,
}

impl Transform2D {
    pub fn new() -> Self {
        Transform2D {
            position: Vec2::new(0.0, 0.0),
            rotation: 0.0,
            scale: Vec2::new(1.0, 1.0),
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.position = Vec2::new(x, y);
    }

    pub fn set_rotation(&mut self, degrees: f32) {
        self.rotation = degrees;
    }

    pub fn rotate(&mut self, degrees: f32) {
        self.rotation += degrees;
        if self.rotation >= 360.0 {
            self.rotation -= 360.0;
        }
    }

    pub fn set_scale(&mut self, x: f32, y: f32) {
        self.scale = Vec2::new(x, y);
    }

    pub fn transform_point(&self, point: Vec2) -> Vec2 {
        // Scale
        let scaled = Vec2::new(
            point.x * self.scale.x,
            point.y * self.scale.y
        );

        // Rotate
        let rad = self.rotation.to_radians();
        let cos = rad.cos();
        let sin = rad.sin();
        let rotated = Vec2::new(
            scaled.x * cos - scaled.y * sin,
            scaled.x * sin + scaled.y * cos
        );

        // Translate
        Vec2::new(
            rotated.x + self.position.x,
            rotated.y + self.position.y
        )
    }
}

impl Default for Transform2D {
    fn default() -> Self {
        Self::new()
    }
}
