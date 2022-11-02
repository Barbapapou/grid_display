use crate::Grid;

pub trait UiElement {
    fn draw(&self, grid: &mut Grid);
}