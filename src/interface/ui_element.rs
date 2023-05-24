use std::collections::VecDeque;
use crate::{Application, Grid};
use crate::interface::ui_action::UiAction;
use crate::interface::ui_error::UiError;

pub trait UiElement {
    fn draw(&self, grid: &mut Grid) -> Result<(), UiError>;
    fn update(&mut self, app: &Application, grid: &Grid, action_queue: &mut VecDeque<UiAction>) -> Result<(), UiError>;
    fn is_mouse_on_element(&self, app: &Application, grid: &Grid) -> bool;
    fn set_id(&mut self, id: u64);
}