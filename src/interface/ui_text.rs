use std::cmp::max;
use std::collections::VecDeque;
use crate::{Application, Grid};
use crate::interface::box_drawing::BoxDrawing;
use crate::interface::ui_action::UiAction;
use crate::interface::ui_element::UiElement;
use crate::interface::ui_error::UiError;
use crate::util::vector::{Vector2};
use crate::interface::word::Word;
use crate::util::rgba8::RGBA8;

pub struct UiText {
    id: u64,
    text: String,
    words: Vec<Word>,
    pos: Vector2,
    size: Vector2,
    max_size: Vector2,
    offset: Vector2,
    pub fg_color: RGBA8,
    pub bg_color: RGBA8,
    box_around: bool,
    box_type: BoxDrawing,
    pub update_function: fn(&mut UiText, &Application, &Grid) -> Result<(), UiError>,
    is_highlighted: bool,
    pub highlight_on_hover: bool,
    pub highlight_word: bool,
    pub actions: Vec<UiAction>
}

impl UiText {
    pub fn new(text: String, pos: Vector2, size: Vector2) -> Result<UiText, UiError> {
        let (words, max_size) = Word::get_word_vec_and_max_size(&text, pos, size)?;
        let ui_text = UiText {
            id: 0,
            text,
            words,
            pos,
            size,
            max_size,
            offset: Vector2::new(0, 0),
            fg_color: RGBA8::new(255, 255, 255, 255),
            bg_color: RGBA8::new(0, 0, 0, 255),
            box_around: false,
            box_type: BoxDrawing::Light,
            update_function: |_ui_text: &mut UiText, _app: &Application, _grid: &Grid| {Ok(())},
            is_highlighted: false,
            highlight_on_hover: false,
            highlight_word: true,
            actions: Vec::new(),
        };
        Ok(ui_text)
    }

    pub fn set_text(&mut self, text: String) -> Result<(), UiError>{
        self.text = text.clone();
        let (words, max_size) = Word::get_word_vec_and_max_size(&text, self.pos, self.size)?;
        self.words = words;
        self.max_size = max_size;
        Ok(())
    }

    pub fn set_box_drawing(&mut self, enable: bool, box_type: BoxDrawing) {
        self.box_around = enable;
        self.box_type = box_type;
    }
}

impl UiElement for UiText {
    fn draw(&self, grid: &mut Grid) -> Result<(), UiError> {
        let mut option = 0;
        if self.box_around {option = 1}
        let start = Vector2::new(self.pos.x - option, self.pos.y - self.size.y + 1 - option);
        let end = Vector2::new(self.pos.x + self.size.x + option, self.pos.y + 1 + option);
        // set color
        grid.set_fg_from_to(start, end, self.fg_color.into());
        grid.set_bg_from_to(start, end, self.bg_color.into());
        // draw words
        for word in self.words.iter() {
            if word.pos.y < self.pos.y - self.size.y + 1 - self.offset.y || word.pos.y > self.pos.y - self.offset.y {
                continue;
            }
            let start_word = Vector2::new(word.pos.x, word.pos.y) + self.offset;
            let end_word = Vector2::new(word.pos.x + word.text.len() as i32, word.pos.y + 1) + self.offset;
            grid.write_at(word.pos + self.offset, &word.text);
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

        Ok(())
    }

    fn update(&mut self, app: &Application, grid: &Grid, action_queue: &mut VecDeque<UiAction>) -> Result<(), UiError>{
        self.update_function.call_once((self, app, grid))?;
        // reset element highlight
        self.is_highlighted = false;
        self.offset.y = (self.offset.y + 1) % self.max_size.y;
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
                        action_queue.push_back(self.actions[action as usize].clone());
                    }
                }
            }
        }

        Ok(())
    }

    fn is_mouse_on_element(&self, app: &Application, _grid: &Grid) -> bool {
        app.grid_position.x >= self.pos.x && app.grid_position.x < self.pos.x + self.size.x &&
        app.grid_position.y > self.pos.y - self.size.y && app.grid_position.y <= self.pos.y
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_pos(&self) -> Vector2 {
        Vector2::new(self.pos.x, self.pos.y - self.size.y + 1)
    }

    fn get_size(&self) -> Vector2 {
        self.size
    }

    fn get_max_size(&self) -> Vector2 {
        self.max_size
    }

    fn get_offset(&self) -> Vector2 {
        self.offset
    }
}