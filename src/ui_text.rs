use crate::Grid;
use crate::ui_element::UiElement;

pub struct UiText {
    text: String,
    pos: (i32, i32)
}

impl UiText {
    pub fn new(text: String, pos: (i32, i32)) -> UiText {
        UiText {
            text,
            pos
        }
    }
}

impl UiElement for UiText {
    fn draw(&self, grid: &mut Grid) {
        grid.write_at(self.pos.0, self.pos.1, &self.text);
    }
}