use rand::Rng;
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
        let mut rng = rand::thread_rng();
        let width_f = width as f32;
        let height_f = height as f32;

        let text: Vec<char> = "Tema la taille de la glyphe.".chars().collect();
        let len_text = text.len() as i32;

        for y in 0..height {
            for x in 0..width {
                let start_x = ((x as f32)       / width_f ) * 2.0 - 1.0;
                let end_x =   ((x as f32 + 1.0) / width_f ) * 2.0 - 1.0;
                let start_y = ((y as f32)       / height_f) * 2.0 - 1.0;
                let end_y =   ((y as f32 + 1.0) / height_f) * 2.0 - 1.0;
                let new_char = text[((x + width * y) % len_text) as usize];
                // let new_char = char::from_u32(x + width * y).unwrap_or('�');
                let fg_color = [rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>(), 1.0];
                let bg_color = [0.0, 0.0, 0.0, 1.0];
                let quad = Quad::new([start_x, end_x, start_y, end_y], fg_color, bg_color, shader_program, new_char);
                quads.push(quad);
            }
        }

        Grid {
            width,
            height,
            quads,
        }
    }

    pub fn get_quad_at(&self, x: i32, y: i32) -> &Quad {
        &self.quads[(y * self.width + x) as usize]
    }

    pub unsafe fn draw(&self) {
        for quad in self.quads.as_slice() {
            quad.draw();
        }
    }

    /*
    Modification
     */

    pub fn write_at(&mut self, x: i32, y: i32, text: &str) {
        let text_vec: Vec<char> = text.chars().collect();
        let mut text_len = text_vec.len() as i32;
        let start_position = y * self.width + x;
        if start_position + text_len > self.quads.len() as i32 - 1 {
            let to_trim = start_position + text_len - self.quads.len() as i32 - 1;
            text_len -= to_trim;
        }
        for text_index in 0..text_len {
            let quad = &mut self.quads[(start_position + text_index) as usize];
            quad.switch_char(text_vec[text_index as usize]);
        }
    }

    pub fn shuffle_glyph(&mut self) {
        let mut rng = rand::thread_rng();
        for quad in self.quads.as_mut_slice() {
            let char = char::from_u32((rng.gen::<f32>() * 65_536.0) as u32).unwrap_or('�');
            let fg_color = [rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>(), 1.0];
            quad.switch_char(char);
            quad.switch_fg_color(fg_color);
        }
    }
}