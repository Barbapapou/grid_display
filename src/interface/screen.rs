use std::collections::VecDeque;
use crate::{Application, Grid};
use crate::interface::box_drawing::BoxDrawing;
use crate::interface::ui_action::UiAction;
use crate::interface::ui_element::UiElement;
use crate::interface::ui_error::UiError;
use crate::interface::ui_text::UiText;
use crate::util::vector::Vector2;

pub struct Screen {
    pub grid: Grid,
    pub grid_width: u32,
    pub grid_height: u32,
    pub ui_elements: Vec<Box<dyn UiElement>>,
    pub next_id: u64,
    pub action_queue: VecDeque<UiAction>
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

        let mut screen = Screen {
            grid,
            grid_width,
            grid_height,
            ui_elements,
            next_id: 0,
            action_queue: VecDeque::new(),
        };

        match UiText::new(String::from("Lorem `cff00ff `l0 ipsum `kl dolor `kc sit amet, consectetur adipiscing elit.\nSuspendisse mi nisl, porta at mollis sit amet, tempor id nunc.\nPellentesque mi nisi, congue a sem ut, vulputate fermentum lacus. Integer eu eleifend massa.\nUt eget porttitor sapien. Donec lacus elit, aliquet ut massa et, tristique imperdiet ex.\nVestibulum lectus massa, consequat a enim vel, volutpat maximus ligula.\nInteger viverra mollis consectetur."), Vector2::new(5, 40), Vector2::new(30, 10)) {
            Ok(mut lorem_ipsum) => {
                lorem_ipsum.set_box_drawing(true, BoxDrawing::Arc);
                lorem_ipsum.actions.push(UiAction::AddUiText(String::from("Generated by action!"), Vector2::new(40, 30), Vector2::new(30, 10)));
                screen.add_element(Box::new(lorem_ipsum));
            }
            Err(_) => {}
        }

        match UiText::new(String::from(""), Vector2::new(1, 1), Vector2::new(10, 1)) {
            Ok(mut delta_time) => {
                delta_time.update_function = |ui_text: &mut UiText, app: &Application, _grid: &Grid| {
                    let delta_time_str = format!("{} ms", app.delta_time);
                    match ui_text.set_text(delta_time_str) {
                        Ok(_) => {}
                        Err(_) => {}
                    }
                };
                delta_time.set_box_drawing(true, BoxDrawing::Double);
                screen.add_element(Box::new(delta_time));
            }
            Err(_) => {}
        }

        match UiText::new(String::from(""), Vector2::new(0, 3), Vector2::new(30, 1)) {
            Ok(mut mouse_pos) => {
                mouse_pos.update_function = |ui_text: &mut UiText, app: &Application, _grid: &Grid| {
                    let mouse_pos_str = format!("Mouse coordinate: {}, {}", app.cursor_position.x, app.cursor_position.y);
                    match ui_text.set_text(mouse_pos_str) {
                        Ok(_) => {}
                        Err(_) => {}
                    }
                };
                screen.add_element(Box::new(mouse_pos));
            }
            Err(_) => {}
        }

        match UiText::new(String::from(""), Vector2::new(0, 4), Vector2::new(30, 1)) {
            Ok(mut grid_pos) => {
                grid_pos.update_function = |ui_text: &mut UiText, app: &Application, grid: &Grid| {
                    let grid_pos_x = (app.cursor_position.x / app.width as f64 * grid.width as f64).floor() as i32;
                    let grid_pos_y = (app.cursor_position.y / app.height as f64 * grid.height as f64).floor() as i32;
                    let mouse_pos_str = format!("Grid coordinate: {grid_pos_x}, {grid_pos_y}");
                    match ui_text.set_text(mouse_pos_str) {
                        Ok(_) => {}
                        Err(_) => {}
                    }
                };
                screen.add_element(Box::new(grid_pos));
            }
            Err(_) => {}
        }

        screen
    }

    pub fn update(&mut self, app: &Application) {
        self.grid.clear();

        for ui_element in self.ui_elements.as_mut_slice() {
            ui_element.update(app, &self.grid, &mut self.action_queue);
            ui_element.draw(&mut self.grid);
        }

        // read action queue
        while let Some(action) = self.action_queue.pop_front() {
            self.perform_action(action);
        }

        // if app.grid_position.x >= 0 && app.grid_position.x < self.grid_width as i32 && app.grid_position.y >= 0 && app.grid_position.y < self.grid_height as i32 {
        //     self.grid.inverse_color_at(Vector2::new(app.grid_position.x, app.grid_position.y));
        // }
    }

    pub fn add_element(&mut self, mut ui_element: Box<dyn UiElement>) {
        ui_element.set_id(self.next_id);
        self.next_id += 1;
        self.ui_elements.push(ui_element);
    }

    pub fn perform_action(&mut self, action: UiAction) {
        match action {
            UiAction::AddUiText(text, pos, size) => {
                match UiText::new(text, pos, size) {
                    Ok(ui_text) => {
                        self.add_element(Box::new(ui_text));
                    }
                    Err(_) => {
                        // todo write error message
                    }
                }
            }
            UiAction::WriteError(_) => {
                // todo write error message
            }
        }
    }
}