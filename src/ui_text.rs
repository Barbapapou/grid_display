use crate::{Application, Grid};
use crate::box_drawing::BoxDrawing;
use crate::ui_element::UiElement;
use crate::util::{Vector2};

pub struct UiText {
    text: String,
    lines: Vec<String>,
    max_line_len: i32,
    pub pos: Vector2,
    width_limit: Option<i32>,
    height_limit: Option<i32>,
    pub fg_color: [f32; 4],
    pub bg_color: [f32; 4],
    pub box_around: bool,
    pub box_type: BoxDrawing,
    pub update_function: fn(&mut UiText, &Application, &Grid),
    is_highlighted: bool,
    pub highlight_on_hover: bool,
}

impl UiText {
    pub fn new(text: String, pos: Vector2) -> UiText {
        let lines: Vec<String> = text.lines().map(|line| line.to_string()).collect();
        let max_line_len = lines.iter().map(|line| line.len()).max().unwrap_or(0) as i32;
        UiText {
            text,
            pos,
            lines,
            max_line_len,
            width_limit: None,
            height_limit: None,
            fg_color: [1.0, 1.0, 1.0, 1.0],
            bg_color: [0.0, 0.0, 0.0, 1.0],
            box_around: false,
            box_type: BoxDrawing::Light,
            update_function: |_ui_text: &mut UiText, _app: &Application, _grid: &Grid| {},
            is_highlighted: false,
            highlight_on_hover: true,
        }
    }

    fn truncate_text(&self, text: &String, max_size: i32) -> String {
        let mut truncated_text = text.clone();
        if self.text.len() as i32 > max_size {
            truncated_text.truncate(max_size as usize);
        }
        truncated_text
    }

    fn get_text_len(&self, text: &String, grid_width: i32) -> i32 {
        let mut max_size = grid_width - self.pos.x;
        if let Some(width_limit) = self.width_limit {
            if width_limit < max_size {
                max_size = width_limit;
            }
        }
        max_size.min(text.len() as i32)
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text.clone();
        self.lines = text.lines().map(|line| line.to_string()).collect();
        self.max_line_len = self.lines.iter().map(|line| line.len()).max().unwrap_or(0) as i32;
    }
}

impl UiElement for UiText {
    fn draw(&self, grid: &mut Grid) {
        for (index, line) in self.lines.iter().enumerate() {
            let y = self.pos.y - index as i32;
            let len = self.get_text_len(line, grid.width as i32);
            let text = self.truncate_text(line, len);
            grid.write_at(self.pos.x, y, &text);
            grid.set_fg_from_to(self.pos.x, y, self.pos.x + len, y + 1, self.fg_color);
            grid.set_bg_from_to(self.pos.x, y, self.pos.x + len, y + 1, self.bg_color);
            if self.is_highlighted {
                grid.inverse_color_from_to(self.pos.x, y, self.pos.x + len, y + 1);
            }
        }

        if self.box_around {
            let mut width = self.max_line_len;
            if let Some(width_limit) = self.width_limit {
                if width_limit < width {
                    width = width_limit;
                }
            }
            let mut height = self.lines.len() as i32;
            if let Some(height_limit) = self.height_limit {
                if height_limit < height {
                    height = height_limit;
                }
            }
            grid.write_box(self.pos.x - 1, self.pos.y - height, self.pos.x + width, self.pos.y + 1, self.box_type);
        }
    }

    fn update(&mut self, app: &Application, grid: &Grid) {
        self.update_function.call_once((self, app, grid));
        if self.highlight_on_hover {
            self.is_highlighted = self.is_mouse_on_element(app, grid);
        }
    }

    fn is_mouse_on_element(&self, app: &Application, grid: &Grid) -> bool {
        for (index, line) in self.lines.iter().enumerate() {
            let len = self.get_text_len(line, grid.width as i32);
            let y = self.pos.y - index as i32;
            if self.pos.x <= app.grid_position.0 && app.grid_position.0 < self.pos.x + len
                && y <= app.grid_position.1 && app.grid_position.1 < y + 1 {
                return true;
            }
        }
        false
    }
}