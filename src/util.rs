#[derive(Copy, Clone)]
pub struct Vector2 {
    pub x: i32,
    pub y: i32,
}

impl Vector2 {
    #[inline]
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
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

#[derive(Copy, Clone)]
pub struct RGBA8
{
    pub r: u8,
    pub b: u8,
    pub g: u8,
    pub a: u8,
}