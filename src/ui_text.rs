use crate::box_drawing::BoxDrawing;
use crate::{Application, Grid};
use crate::ui_element::UiElement;
use crate::util::Vector2;

pub struct UiText {
    pub text: String,
    pos: Vector2,
    box_around: bool,
    box_type: BoxDrawing,
    pub update_function: fn(&mut UiText, &Application, &Grid),
}

impl UiText {
    pub fn new_basic(text: String, pos: Vector2) -> UiText {
        UiText {
            text,
            pos,
            box_around: false,
            box_type: BoxDrawing::Light,
            update_function: |ui_text: &mut UiText, app: &Application, grid: &Grid| {}
        }
    }

    pub fn new_box(text: String, pos: Vector2, box_type: BoxDrawing) -> UiText {
        UiText {
            text,
            pos,
            box_around: true,
            box_type,
            update_function: |ui_text: &mut UiText, app: &Application, grid: &Grid| {}
        }
    }
}

impl UiElement for UiText {
    fn draw(&self, grid: &mut Grid) {
        grid.write_at(self.pos.x, self.pos.y, &self.text);
        if self.box_around {
            grid.write_box(self.pos.x - 1, self.pos.y - 1, self.pos.x + self.text.len() as i32, self.pos.y + 1, self.box_type);
        }
    }

    fn update(&mut self, app: &Application, grid: &Grid) {
        self.update_function.call_once((self, app, grid));
    }
}