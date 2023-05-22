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

impl Vector2f {
    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Copy, Clone)]
pub struct Vector2d {
    pub x: f64,
    pub y: f64,
}

impl Vector2d {
    #[inline]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
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

use substring::Substring;

impl RGBA8 {
    #[inline]
    pub fn new (r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {r, g, b, a}
    }

    pub fn from_hex_string(input: &String) -> Self {
        let r = input.substring(0, 2);
        let r = hex::decode(r).expect("Failed to decode the red component of the color");
        let r = r.iter().enumerate().map(|(i, v)| v << (i * 8)).sum();
        let g = input.substring(2, 4);
        let g = hex::decode(g).expect("Failed to decode the green component of the color");
        let g = g.iter().enumerate().map(|(i, v)| v << (i * 8)).sum();
        let b = input.substring(4, 6);
        let b = hex::decode(b).expect("Failed to decode the blue component of the color");
        let b = b.iter().enumerate().map(|(i, v)| v << (i * 8)).sum();
        RGBA8 {r, g, b, a: 255}
    }
}

impl Into<[f32; 4]> for RGBA8 {
    fn into(self) -> [f32; 4] {
        [self.r as f32 / 255.0, self.g as f32 / 255.0, self.b as f32 / 255.0, self.a as f32 / 255.0]
    }
}