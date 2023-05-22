use crate::{Application, Grid};
use crate::interface::box_drawing::BoxDrawing;
use crate::interface::ui_element::UiElement;
use crate::util::vector::{Vector2};
use crate::interface::word::Word;
use crate::util::rgba8::RGBA8;

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
    pub highlight_word: bool,
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
            highlight_on_hover: false,
            highlight_word: true,
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
        let start = Vector2::new(self.pos.x, self.pos.y - self.size.y + 1);
        let end = Vector2::new(self.pos.x + self.size.x, self.pos.y + 1);
        // set color
        grid.set_fg_from_to(start, end, self.fg_color);
        grid.set_bg_from_to(start, end, self.bg_color);
        // draw words
        for word in self.words.iter() {
            if word.pos.y < self.pos.y - self.size.y + 1 {
                break;
            }
            let start_word = Vector2::new(word.pos.x, word.pos.y);
            let end_word = Vector2::new(word.pos.x + word.text.len() as i32, word.pos.y + 1);
            grid.write_at(word.pos, &word.text);
            if let Some(color) = word.fg_color {
                grid.set_fg_from_to(start_word, end_word, color.into());
            }
            if word.highlight {
                grid.inverse_color_from_to(start_word, end_word);
            }
        }

        if self.box_around {
            let start_box = Vector2::new(self.pos.x - 1, self.pos.y - self.size.y);
            let end_box = Vector2::new(self.pos.x + self.size.x, self.pos.y + 1);
            grid.write_box(start_box, end_box, self.box_type);
        }

        if self.is_highlighted {
            grid.inverse_color_from_to(start, end);
        }
    }

    fn update(&mut self, app: &Application, grid: &Grid) {
        self.update_function.call_once((self, app, grid));
        // reset element highlight
        self.is_highlighted = false;
        // reset words highlight
        for word in self.words.iter_mut() {
            word.highlight = false;
        }
        // check if mouse is on element and perform related actions
        if self.is_mouse_on_element(app, grid) {
            // highlight on hover whole element
            if self.highlight_on_hover {
                self.is_highlighted = true;
            }
            // find hovered word if any
            let mut hovered_word = None;
            for word in self.words.iter_mut() {
                if word.pos.x <= app.grid_position.x && word.pos.x + (word.text.len() as i32) > app.grid_position.x && word.pos.y == app.grid_position.y {
                    hovered_word = Some(word);
                }
            }
            if let Some(word) = hovered_word {
                // highlight word
                if self.highlight_word {
                    word.highlight = true;
                }
                // check if mouse is clicked and if yes trigger word action
                if let Some(action) = word.action {
                    if app.mouse_left == 1 {
                        println!("Word action: {}", action)
                    }
                }
            }
        }
    }

    fn is_mouse_on_element(&self, app: &Application, grid: &Grid) -> bool {
        app.grid_position.x >= self.pos.x && app.grid_position.x < self.pos.x + self.size.x &&
        app.grid_position.y > self.pos.y - self.size.y && app.grid_position.y <= self.pos.y
    }
}