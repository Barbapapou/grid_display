use std::ops::{Add, Sub};

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

impl Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Self::Output {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub<Vector2> for Vector2 {
    type Output = Vector2;

    #[inline]
    fn sub(self, rhs: Vector2) -> Self::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
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
