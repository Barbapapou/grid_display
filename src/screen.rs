use crate::{Application, Grid};
use crate::ui_element::UiElement;
use crate::ui_text::UiText;

pub struct Screen {
    pub grid: Grid,
    grid_width: i32,
    grid_height: i32,
    pub ui_elements: Vec<Box<dyn UiElement>>,
}

impl Screen {
    pub fn new(shader_program: u32) -> Screen {
        let mul = 5;
        let grid_width = 16 * 2 * mul;
        let grid_height = 9 * mul;

        let grid = Grid::new(grid_width, grid_height, shader_program);
        let mut ui_elements: Vec<Box<dyn UiElement>> = vec![
            Box::new(UiText::new(String::from("Hello world"), (15, 15))),
            Box::new(UiText::new(String::from("Hello from the whole world"), (25, 25))),
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
        self.grid.write_at(1, 1, &delta_time_str);
        self.grid.write_box(0, 0, delta_time_str.len() as i32 + 1, 2);
        self.grid.write_at(5, 5, "Hello world!");
        let mouse_pos_x = cursor_position.0;
        let mouse_pos_y = cursor_position.1;
        let mouse_pos_str = format!("Mouse coordinate: {mouse_pos_x}, {mouse_pos_y}");
        self.grid.write_at(0,3, &mouse_pos_str);
        let grid_pos_x = (mouse_pos_x / app.width as f64 * self.grid_width as f64).floor() as i32;
        let grid_pos_y = (mouse_pos_y / app.height as f64 * self.grid_height as f64).floor() as i32;
        let mouse_pos_str = format!("Grid coordinate: {grid_pos_x}, {grid_pos_y}");
        self.grid.write_at(0,4, &mouse_pos_str);
        if grid_pos_x >= 0 && grid_pos_x < self.grid_width && grid_pos_y >= 0 && grid_pos_y < self.grid_height {
            self.grid.inverse_color_at(grid_pos_x as i32, grid_pos_y as i32);
        }
    }
}