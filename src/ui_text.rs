use crate::box_drawing::BoxDrawing;
use crate::{Application, Grid};
use crate::ui_element::UiElement;
use crate::util::{Vector2};

pub struct UiText {
    pub text: String,
    pub pos: Vector2,
    pub size_limit: Option<i32>,
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
        UiText {
            text,
            pos,
            size_limit: None,
            fg_color: [1.0, 1.0, 1.0, 1.0],
            bg_color: [0.0, 0.0, 0.0, 1.0],
            box_around: false,
            box_type: BoxDrawing::Light,
            update_function: |_ui_text: &mut UiText, _app: &Application, _grid: &Grid| {},
            is_highlighted: false,
            highlight_on_hover: true,
        }
    }

    fn truncate_text(&self, max_size: i32) -> String {
        let mut truncated_text = self.text.clone();
        if self.text.len() as i32 > max_size {
            truncated_text.truncate(max_size as usize);
        }
        truncated_text
    }

    fn get_text_len(&self, grid_width: i32) -> i32 {
        let mut max_size = grid_width - self.pos.x;
        if let Some(size_limit) = self.size_limit {
            if size_limit < max_size {
                max_size = size_limit;
            }
        }
        max_size.min(self.text.len() as i32)
    }
}

impl UiElement for UiText {
    fn draw(&self, grid: &mut Grid) {
        let len = self.get_text_len(grid.width as i32);
        let text = self.truncate_text(len);
        grid.write_at(self.pos.x, self.pos.y, &text);
        grid.set_fg_from_to(self.pos.x, self.pos.y, self.pos.x + len, self.pos.y + 1, self.fg_color);
        grid.set_bg_from_to(self.pos.x, self.pos.y, self.pos.x + len, self.pos.y + 1, self.bg_color);
        if self.is_highlighted {
            grid.inverse_color_from_to(self.pos.x, self.pos.y, self.pos.x + len, self.pos.y + 1);
        }
        if self.box_around {
            grid.write_box(self.pos.x - 1, self.pos.y - 1, self.pos.x + len, self.pos.y + 1, self.box_type);
        }
    }

    fn update(&mut self, app: &Application, grid: &Grid) {
        self.update_function.call_once((self, app, grid));
        if self.highlight_on_hover {
            self.is_highlighted = self.is_mouse_on_element(app, grid);
        }
    }

    fn is_mouse_on_element(&self, app: &Application, grid: &Grid) -> bool {
        let len = self.get_text_len(grid.width as i32);
        self.pos.x <= app.grid_position.0 && app.grid_position.0 < self.pos.x + len
            && self.pos.y <= app.grid_position.1 && app.grid_position.1 < self.pos.y + 1
    }
}