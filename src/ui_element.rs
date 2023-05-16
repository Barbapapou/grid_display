use crate::{Application, Grid};

pub trait UiElement {
    fn draw(&self, grid: &mut Grid);
    fn update(&mut self, app: &Application, grid: &Grid);
    fn is_mouse_on_element(&self, app: &Application) -> bool;
}