use crate::{Application, Grid};
use crate::box_drawing::BoxDrawing;
use crate::ui_element::UiElement;
use crate::util::{Vector2};
use crate::word::Word;

pub struct UiText {
    text: String,
    words: Vec<Word>,
    pos: Vector2,
    size: Vector2,
    pub fg_color: [f32; 4],
    pub bg_color: [f32; 4],
    box_around: bool,
    box_type: BoxDrawing,
    pub update_function: fn(&mut UiText, &Application, &Grid),
    is_highlighted: bool,
    pub highlight_on_hover: bool,
}

impl UiText {
    pub fn new(text: String, pos: Vector2, size: Vector2) -> UiText {
        let words = Word::get_word_vec(&text, pos, size);
        UiText {
            text,
            words,
            pos,
            size,
            fg_color: [1.0, 1.0, 1.0, 1.0],
            bg_color: [0.0, 0.0, 0.0, 1.0],
            box_around: false,
            box_type: BoxDrawing::Light,
            update_function: |_ui_text: &mut UiText, _app: &Application, _grid: &Grid| {},
            is_highlighted: false,
            highlight_on_hover: true,
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text.clone();
        self.words = Word::get_word_vec(&text, self.pos, self.size);
    }

    pub fn set_box_drawing(&mut self, enable: bool, box_type: BoxDrawing) {
        self.box_around = enable;
        self.box_type = box_type;
    }
}

impl UiElement for UiText {
    fn draw(&self, grid: &mut Grid) {
        for word in self.words.iter() {
            grid.write_at(word.pos, &word.text);
        }
        //
        // if self.box_around {
        //     let mut width = self.max_line_len;
        //     if let Some(width_limit) = self.width_limit {
        //         if width_limit < width {
        //             width = width_limit;
        //         }
        //     }
        //     let mut height = y_offset;
        //     if let Some(height_limit) = self.height_limit {
        //         if height_limit < height {
        //             height = height_limit;
        //         }
        //     }
        //     grid.write_box(self.pos.x - 1, self.pos.y - height, self.pos.x + width, self.pos.y + 1, self.box_type);
        // }
    }

    fn update(&mut self, app: &Application, grid: &Grid) {
        self.update_function.call_once((self, app, grid));
        if self.highlight_on_hover {
            self.is_highlighted = self.is_mouse_on_element(app, grid);
        }
    }

    fn is_mouse_on_element(&self, app: &Application, grid: &Grid) -> bool {
        // for (index, line) in self.lines.iter().enumerate() {
        //     let len = self.get_text_len(line, grid.width as i32);
        //     let y = self.pos.y - index as i32;
        //     if self.pos.x <= app.grid_position.0 && app.grid_position.0 < self.pos.x + len
        //         && y <= app.grid_position.1 && app.grid_position.1 < y + 1 {
        //         return true;
        //     }
        // }
        false
    }
}