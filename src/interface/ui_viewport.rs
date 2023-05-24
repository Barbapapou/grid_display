use std::collections::VecDeque;
use crate::Application;
use crate::interface::ui_action::UiAction;
use crate::interface::ui_element::UiElement;
use crate::interface::ui_error::UiError;
use crate::render::grid::Grid;
use crate::util::vector::Vector2;

pub struct UiViewport {
    id: u64,
    inside_element: Box<dyn UiElement>,
}

impl UiViewport {
    pub fn new(inside_element: Box<dyn UiElement>) -> UiViewport {
        UiViewport {
            id: 0,
            inside_element,
        }
    }
}

impl UiElement for UiViewport {
    fn draw(&self, grid: &mut Grid) -> Result<(), UiError> {
        self.inside_element.draw(grid)?;
        // draw vertical scrollbar
        let pos = self.inside_element.get_pos();
        let size = self.inside_element.get_size();
        let background_start = Vector2::new(pos.x + size.x, pos.y);
        let background_end = Vector2::new(pos.x + size.x + 1, pos.y + size.y);
        // draw background
        grid.write_from_to(background_start, background_end, '░');
        // draw handle
        let handle_start = Vector2::new(pos.x + size.x, pos.y + size.y / 2);
        let handle_end = Vector2::new(pos.x + size.x + 1, pos.y + size.y / 2 + 1);
        grid.write_from_to(handle_start, handle_end, '█');
        Ok(())
    }

    fn update(&mut self, app: &Application, grid: &Grid, action_queue: &mut VecDeque<UiAction>) -> Result<(), UiError> {
        self.inside_element.update(app, grid, action_queue)
    }

    fn is_mouse_on_element(&self, app: &Application, grid: &Grid) -> bool {
        todo!()
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_pos(&self) -> Vector2 {
        self.inside_element.get_pos()
    }

    fn get_size(&self) -> Vector2 {
        let mut size = self.inside_element.get_size();
        size.x += 1;
        size
    }

    fn get_max_size(&self) -> Vector2 {
        self.get_size()
    }

    fn get_offset(&self) -> Vector2 {
        Vector2::new(0, 0)
    }
}