use crate::{Application, Grid};
use crate::box_drawing::BoxDrawing;
use crate::ui_element::UiElement;
use crate::ui_text::UiText;
use crate::util::Vector2;

pub struct Screen {
    pub grid: Grid,
    grid_width: u32,
    grid_height: u32,
    pub ui_elements: Vec<Box<dyn UiElement>>,
}

impl Screen {
    pub fn new(shader_program: u32) -> Screen {
        let mul = 5;
        let grid_width = 16 * 2 * mul;
        let grid_height = 9 * mul;

        let grid = Grid::new(grid_width, grid_height, shader_program);
        let ui_elements: Vec<Box<dyn UiElement>> = vec![
            Box::new(UiText::new_basic(String::from("<Hello world>"), Vector2 {x: 15, y: 15})),
            Box::new(UiText::new_basic(String::from("<Hello from the whole world>"), Vector2 {x: 15, y: 18})),
        ];

        Screen {
            grid,
            grid_width,
            grid_height,
            ui_elements
        }
    }

    pub fn update(&mut self, delta_time: u128, app: &Application, cursor_position: (f64, f64)) {
        self.grid.clear();

        for ui_element in self.ui_elements.as_slice() {
            ui_element.draw(&mut self.grid);
        }

        let delta_time_str = format!("{delta_time} ms");
        let delta_time_ui_text = UiText::new_box(delta_time_str, Vector2 {x:1, y:1}, BoxDrawing::Arc);
        delta_time_ui_text.draw(&mut self.grid);

        let mouse_pos_x = cursor_position.0;
        let mouse_pos_y = cursor_position.1;
        let mouse_pos_str = format!("Mouse coordinate: {mouse_pos_x}, {mouse_pos_y}");
        let mouse_pos_ui_text = UiText::new_basic(mouse_pos_str, Vector2 {x:0, y:3});
        mouse_pos_ui_text.draw(&mut self.grid);

        let grid_pos_x = (mouse_pos_x / app.width as f64 * self.grid_width as f64).floor() as i32;
        let grid_pos_y = (mouse_pos_y / app.height as f64 * self.grid_height as f64).floor() as i32;
        let mouse_pos_str = format!("Grid coordinate: {grid_pos_x}, {grid_pos_y}");
        let mouse_pos_ui_text = UiText::new_basic(mouse_pos_str, Vector2 {x:0, y:4});
        mouse_pos_ui_text.draw(&mut self.grid);

        if grid_pos_x >= 0 && grid_pos_x < self.grid_width as i32 && grid_pos_y >= 0 && grid_pos_y < self.grid_height as i32 {
            self.grid.inverse_color_at(grid_pos_x as i32, grid_pos_y as i32);
        }
    }
}