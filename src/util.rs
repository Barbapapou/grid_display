#[derive(Copy, Clone)]
pub struct Vector2 {
    pub x: i32,
    pub y: i32,
}

#[derive(Copy, Clone)]
pub struct Vector2f {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone)]
pub struct UvLayout {
    pub min: Vector2f,
    pub max: Vector2f
}