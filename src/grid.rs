use rand::Rng;
use crate::box_drawing::BoxDrawing;
use crate::Quad;

pub struct Grid {
    width: i32,
    height: i32,
    quads: Vec<Quad>
}

impl Grid {
    pub fn new(width: i32, height: i32, shader_program: u32) -> Grid
    {
        let mut quads = Vec::with_capacity((width * height) as usize);
        let width_f = width as f32;
        let height_f = height as f32;

        for y in 0..height {
            for x in 0..width {
                let start_x = ((x as f32)       / width_f ) * 2.0 - 1.0;
                let end_x =   ((x as f32 + 1.0) / width_f ) * 2.0 - 1.0;
                let start_y = ((y as f32)       / height_f) * 2.0 - 1.0;
                let end_y =   ((y as f32 + 1.0) / height_f) * 2.0 - 1.0;
                let fg_color = [1.0, 1.0, 1.0, 1.0];
                let bg_color = [0.0, 0.0, 0.0, 1.0];
                let quad = Quad::new([start_x, end_x, start_y, end_y], fg_color, bg_color, shader_program, ' ');
                quads.push(quad);
            }
        }

        Grid {
            width,
            height,
            quads,
        }
    }

    pub unsafe fn draw(&self) {
        for quad in self.quads.as_slice() {
            quad.draw();
        }
    }

    /*
    Modification
     */

    pub fn clear_grid(&mut self) {
        for quad in self.quads.as_mut_slice() {
            quad.switch_char(' ');
            quad.switch_fg_color([1.0, 1.0, 1.0, 1.0]);
            quad.switch_bg_color([0.0, 0.0, 0.0, 1.0]);
        }
    }

    pub fn write_at(&mut self, x: i32, y: i32, text: &str) {
        let text_vec: Vec<char> = text.chars().collect();
        let mut text_len = text_vec.len() as i32;
        let start_position = y * self.width + x;
        if start_position + text_len > self.quads.len() as i32 - 1 {
            let to_trim = start_position + (text_len - 1) - (self.quads.len() as i32 - 1);
            text_len -= to_trim;
        }
        for text_index in 0..text_len {
            let quad = &mut self.quads[(start_position + text_index) as usize];
            quad.switch_char(text_vec[text_index as usize]);
        }
    }

    pub fn write_box(&mut self, x_start: i32, y_start: i32, x_end: i32, y_end: i32) {
        let quads = self.quads.as_mut_slice();
        let (h_line, v_line, l_l_corner, u_l_corner, l_r_corner, u_r_corner) = BoxDrawing::get_char(BoxDrawing::Arc);
        for x in x_start..=x_end {
            for y in y_start..=y_end {
                let index = (y * self.width + x) as usize;
                if (x != x_start && x != x_end && y != y_start && y != y_end) || x > self.width || y > self.height {
                    continue;
                }
                if x == x_start && y == y_start {
                    quads[index].switch_char(l_l_corner);
                }
                else if x == x_end && y == y_end {
                    quads[index].switch_char(u_r_corner);
                }
                else if x == x_start && y == y_end {
                    quads[index].switch_char(u_l_corner);
                }
                else if x == x_end && y == y_start {
                    quads[index].switch_char(l_r_corner);
                }
                else if x == x_start || x == x_end {
                    quads[index].switch_char(v_line);
                }
                else if y == y_start || y == y_end {
                    quads[index].switch_char(h_line);
                }
            }
        }
    }

    pub fn shuffle_glyph(&mut self) {
        let mut rng = rand::thread_rng();
        for quad in self.quads.as_mut_slice() {
            let char = char::from_u32((rng.gen::<f32>() * 65_536.0) as u32).unwrap_or('�');
            let fg_color = [rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>(), 1.0];
            let bg_color = [rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>(), 1.0];
            quad.switch_char(char);
            quad.switch_fg_color(fg_color);
            quad.switch_bg_color(bg_color);
        }
    }
}