use crate::{Application, Grid};
use crate::interface::box_drawing::BoxDrawing;
use crate::interface::ui_element::UiElement;
use crate::interface::ui_text::UiText;
use crate::util::Vector2;

pub struct Screen {
    pub grid: Grid,
    pub grid_width: u32,
    pub grid_height: u32,
    pub ui_elements: Vec<Box<dyn UiElement>>,
}

impl Screen {
    pub fn new(shader_program: u32) -> Screen {
        let mul = 5;
        let grid_width = 16 * 2 * mul;
        let grid_height = 9 * mul;

        let grid = Grid::new(grid_width, grid_height, shader_program);
        let mut ui_elements: Vec<Box<dyn UiElement>> = vec![
            // Box::new(UiText::new(String::from("<Hello world>"), Vector2 {x: 15, y: 15})),
            // Box::new(UiText::new(String::from("<Hello from the whole world>"), Vector2 {x: 15, y: 18})),
        ];

        let mut lorem_ispum = Box::new(UiText::new(String::from("Lorem `cff00ff ipsum `c5c9cc4 dolor sit amet, consectetur adipiscing elit.\nSuspendisse mi nisl, porta at mollis sit amet, tempor id nunc.\nPellentesque mi nisi, congue a sem ut, vulputate fermentum lacus. Integer eu eleifend massa.\nUt eget porttitor sapien. Donec lacus elit, aliquet ut massa et, tristique imperdiet ex.\nVestibulum lectus massa, consequat a enim vel, volutpat maximus ligula.\nInteger viverra mollis consectetur."), Vector2::new(5, 40), Vector2::new(30, 10)));
        lorem_ispum.set_box_drawing(true, BoxDrawing::Arc);
        // lorem_ispum.fg_color = [0.0, 0.0, 1.0, 0.4];
        // lorem_ispum.bg_color = [0.0, 1.0, 0.0, 0.0];
        ui_elements.push(lorem_ispum);

        let mut delta_time = UiText::new(String::from(""), Vector2::new(1, 1), Vector2::new(10, 1));
        delta_time.update_function = |ui_text: &mut UiText, app: &Application, _grid: &Grid| {
            let delta_time_str = format!("{} ms", app.delta_time);
            ui_text.set_text(delta_time_str);
        };
        delta_time.set_box_drawing(true, BoxDrawing::Double);
        ui_elements.push(Box::new(delta_time));

        let mut mouse_pos = UiText::new(String::from(""), Vector2::new(0, 3), Vector2::new(30, 1));
        mouse_pos.update_function = |ui_text: &mut UiText, app: &Application, _grid: &Grid| {
            let mouse_pos_str = format!("Mouse coordinate: {}, {}", app.cursor_position.x, app.cursor_position.y);
            ui_text.set_text(mouse_pos_str);
        };
        ui_elements.push(Box::new(mouse_pos));

        let mut grid_pos = UiText::new(String::from(""), Vector2::new(0, 4), Vector2::new(30, 1));
        grid_pos.update_function = |ui_text: &mut UiText, app: &Application, grid: &Grid| {
            let grid_pos_x = (app.cursor_position.x / app.width as f64 * grid.width as f64).floor() as i32;
            let grid_pos_y = (app.cursor_position.y / app.height as f64 * grid.height as f64).floor() as i32;
            let mouse_pos_str = format!("Grid coordinate: {grid_pos_x}, {grid_pos_y}");
            ui_text.set_text(mouse_pos_str);
        };
        ui_elements.push(Box::new(grid_pos));

        Screen {
            grid,
            grid_width,
            grid_height,
            ui_elements,
        }
    }

    pub fn update(&mut self, app: &Application) {
        self.grid.clear();

        for ui_element in self.ui_elements.as_mut_slice() {
            ui_element.update(app, &self.grid);
            ui_element.draw(&mut self.grid);
        }

        // if app.grid_position.x >= 0 && app.grid_position.x < self.grid_width as i32 && app.grid_position.y >= 0 && app.grid_position.y < self.grid_height as i32 {
        //     self.grid.inverse_color_at(Vector2::new(app.grid_position.x, app.grid_position.y));
        // }
    }
}