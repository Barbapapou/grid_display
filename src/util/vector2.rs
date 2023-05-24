use std::ops::{Add, Div, Mul, Sub};

pub type Vector2 = Vector2Generic<i32>;
pub type Vector2f = Vector2Generic<f32>;
pub type Vector2d = Vector2Generic<f64>;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector2Generic<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2Generic<T> {
    #[inline]
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Add<Output = T>> Add<Vector2Generic<T>> for Vector2Generic<T> {
    type Output = Vector2Generic<T>;

    fn add(self, rhs: Vector2Generic<T>) -> Self::Output {
        Vector2Generic::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Sub<Output = T>> Sub<Vector2Generic<T>> for Vector2Generic<T> {
    type Output = Vector2Generic<T>;

    #[inline]
    fn sub(self, rhs: Vector2Generic<T>) -> Self::Output {
        Vector2Generic::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vector2Generic<T> {
    type Output = Vector2Generic<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector2Generic::new(self.x * rhs, self.y * rhs)
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vector2Generic<T> {
    type Output = Vector2Generic<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vector2Generic::new(self.x / rhs, self.y / rhs)
    }
}

