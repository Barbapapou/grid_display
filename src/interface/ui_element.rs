use std::collections::VecDeque;
use crate::{Application, Grid};
use crate::interface::ui_action::UiAction;
use crate::interface::ui_error::UiError;
use crate::util::vector2::{Vector2};

pub trait UiElement {
    fn draw(&self, grid: &mut Grid) -> Result<(), UiError>;
    fn update(&mut self, app: &Application, grid: &Grid, action_queue: &mut VecDeque<UiAction>) -> Result<(), UiError>;
    fn is_mouse_on_element(&self, app: &Application, grid: &Grid) -> bool;
    fn set_id(&mut self, id: u64);
    // give the position of the bottom left corner
    fn get_pos(&self) -> Vector2;
    fn get_size(&self) -> Vector2;
    fn get_max_size(&self) -> Vector2;
    fn get_offset(&self) -> Vector2;
    fn set_offset(&mut self, offset: Vector2);
}