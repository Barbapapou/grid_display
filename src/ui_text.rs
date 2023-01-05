use crate::box_drawing::BoxDrawing;
use crate::Grid;
use crate::ui_element::UiElement;
use crate::util::Vector2;

pub struct UiText {
    text: String,
    pos: Vector2,
    box_around: bool,
    box_type: BoxDrawing,
}

impl UiText {
    pub fn new_basic(text: String, pos: Vector2) -> UiText {
        UiText {
            text,
            pos,
            box_around: false,
            box_type: BoxDrawing::Light
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
}