pub const PI: f32 = std::f32::consts::PI;

pub fn to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn to_degrees(radians: f32) -> f32 {
    radians * 180.0 / PI
}

pub fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + t * (end - start)
}
