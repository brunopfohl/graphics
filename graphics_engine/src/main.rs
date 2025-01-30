mod math;
mod graphics;

use math::{Vec3, Mat4, Vec2};
use graphics::{Sprite, Transform2D};

fn main() {
    // Test vector operations
    let v1 = Vec3::new(1.0, 0.0, 0.0);
    let v2 = Vec3::new(0.0, 1.0, 0.0);
    let cross = v1.cross(&v2);

    println!("Cross product of v1 x v2: {:?}", cross);

    // Test matrix operations
    let m = Mat4::identity();
    println!("Identity matrix: {:?}", m);

    // Create a sprite
    let mut sprite = Sprite::new(100.0, 100.0);
    
    // Apply some transformations
    sprite.transform.translate(Vec2::new(50.0, 50.0));
    sprite.transform.rotate(45.0);
    sprite.transform.set_scale(Vec2::new(1.5, 1.5));
    
    // Get transformed vertices
    let transformed = sprite.get_transformed_vertices();
    println!("Transformed vertices: {:?}", transformed);
}
