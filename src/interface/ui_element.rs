use std::collections::VecDeque;
use crate::{Application, Grid};
use crate::interface::ui_action::UiAction;

pub trait UiElement {
    fn draw(&self, grid: &mut Grid);
    fn update(&mut self, app: &Application, grid: &Grid, action_queue: &mut VecDeque<UiAction>);
    fn is_mouse_on_element(&self, app: &Application, grid: &Grid) -> bool;
    fn set_id(&mut self, id: u64);
}