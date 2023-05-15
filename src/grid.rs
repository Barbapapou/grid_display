use std::ffi::c_void;
use std::mem::size_of;
use std::ptr;
use gl::types::*;
use rand::{Rng, thread_rng};
use crate::Application;
use crate::box_drawing::BoxDrawing;
use crate::cache_glyph::CacheGlyph;
use crate::quad::Quad;

pub struct Grid {
    pub width: u32,
    pub height: u32,
    vao: u32,
    program: u32,
    nb_vertex: i32,
    texture_coordinate_buffer: u32,
    texture_coordinate: Vec<f32>,
    fg_color_buffer: u32,
    fg_color: Vec<f32>,
    bg_color_buffer: u32,
    bg_color: Vec<f32>,
    cache_glyph: CacheGlyph,
    quads: Vec<Quad>
}

impl Grid {
    pub fn new(width: u32, height: u32, program: u32) -> Grid {
        let width_f = width as f32;
        let height_f = height as f32;

        let mut vertex_position: Vec<f32> = vec![0.0; (12 * width * height) as usize];
        let mut texture_coordinate: Vec<f32> = vec![0.0; (8 * width * height) as usize];
        let mut fg_color: Vec<f32> = vec![0.0; (16 * width * height) as usize];
        let mut bg_color: Vec<f32> = vec![0.0; (16 * width * height) as usize];
        let mut indices: Vec<u32> = vec![0; (6 * width * height) as usize];
        let mut vp_b_count: usize = 0;
        let mut tc_b_count: usize = 0;
        let mut c_b_count: usize = 0;
        let mut i_b_count: usize = 0;
        let mut v_count: u32 = 0;

        for y in 0..height {
            for x in 0..width {
                let start_x = ((x as f32)       / width_f ) * 2.0 - 1.0;
                let end_x =   ((x as f32 + 1.0) / width_f ) * 2.0 - 1.0;
                let start_y = ((y as f32)       / height_f) * 2.0 - 1.0;
                let end_y =   ((y as f32 + 1.0) / height_f) * 2.0 - 1.0;

                let vertex_position_t: [f32; 12] = [
                    end_x  , end_y  , 0.0,
                    end_x  , start_y, 0.0,
                    start_x, start_y, 0.0,
                    start_x, end_y  , 0.0,
                ];
                vertex_position[vp_b_count..(vp_b_count + 12)].copy_from_slice(&vertex_position_t);

                let texture_coordinate_t: [f32; 8] = [
                    1.0, 1.0,
                    1.0, 0.0,
                    0.0, 0.0,
                    0.0, 1.0
                ];
                texture_coordinate[tc_b_count..(tc_b_count + 8)].copy_from_slice(&texture_coordinate_t);

                // Can cut the memory cost by 4 using instancing and glVertexAttribPointer, not sure it would give better performance tho
                let fg_color_t: [f32; 16] = [
                    1.0, 1.0, 1.0, 1.0,
                    1.0, 1.0, 1.0, 1.0,
                    1.0, 1.0, 1.0, 1.0,
                    1.0, 1.0, 1.0, 1.0];
                fg_color[c_b_count..(c_b_count+16)].copy_from_slice(&fg_color_t);

                let bg_color_t: [f32; 16] = [
                    0.0, 0.0, 0.0, 1.0,
                    0.0, 0.0, 0.0, 1.0,
                    0.0, 0.0, 0.0, 1.0,
                    0.0, 0.0, 0.0, 1.0];
                bg_color[c_b_count..(c_b_count+16)].copy_from_slice(&bg_color_t);

                let indices_t: [u32; 6] = [
                    v_count    , v_count + 1, v_count + 3,
                    v_count + 1, v_count + 2, v_count + 3
                ];
                indices[i_b_count..(i_b_count + 6)].copy_from_slice(&indices_t);

                vp_b_count += 12;
                tc_b_count += 8;
                c_b_count += 16;
                i_b_count += 6;
                v_count += 4;
            }
        }

        let mut vao: u32 = 0;
        let mut vertex_position_buffer: u32 = 0;
        let mut texture_coordinate_buffer: u32 = 0;
        let mut fg_color_buffer: u32 = 0;
        let mut bg_color_buffer: u32 = 0;
        let mut indices_buffer: u32 = 0;
        let vertex_position_attrib_location: GLint;
        let texture_coordinate_attrib_location: GLint;
        let fg_color_attrib_location: GLint;
        let bg_color_attrib_location: GLint;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            vertex_position_attrib_location = gl::GetAttribLocation(program, b"aVertexPosition\0".as_ptr() as *const i8);
            gl::GenBuffers(1, &mut vertex_position_buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_position_buffer);
            gl::BufferData(gl::ARRAY_BUFFER, (vertex_position.len() * size_of::<f32>()) as isize, vertex_position.as_ptr() as *const c_void, gl::STATIC_DRAW);
            gl::VertexAttribPointer(vertex_position_attrib_location as GLuint, 3, gl::FLOAT, gl::FALSE, 0, ptr::null::<c_void>());
            gl::EnableVertexAttribArray(vertex_position_attrib_location as GLuint);

            texture_coordinate_attrib_location = gl::GetAttribLocation(program, b"aTextureCoord\0".as_ptr() as *const i8);
            gl::GenBuffers(1, &mut texture_coordinate_buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, texture_coordinate_buffer);
            gl::BufferData(gl::ARRAY_BUFFER, (texture_coordinate.len() * size_of::<f32>()) as isize, texture_coordinate.as_ptr() as *const c_void, gl::DYNAMIC_DRAW);
            gl::VertexAttribPointer(texture_coordinate_attrib_location as GLuint, 2, gl::FLOAT, gl::FALSE, 0, ptr::null::<c_void>());
            gl::EnableVertexAttribArray(texture_coordinate_attrib_location as GLuint);

            fg_color_attrib_location = gl::GetAttribLocation(program, b"aFgColor\0".as_ptr() as *const i8);
            gl::GenBuffers(1, &mut fg_color_buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, fg_color_buffer);
            gl::BufferData(gl::ARRAY_BUFFER, (fg_color.len() * size_of::<f32>()) as isize, fg_color.as_ptr() as *const c_void, gl::DYNAMIC_DRAW);
            gl::VertexAttribPointer(fg_color_attrib_location as GLuint, 4, gl::FLOAT, gl::FALSE, 0, ptr::null::<c_void>());
            gl::EnableVertexAttribArray(fg_color_attrib_location as GLuint);

            bg_color_attrib_location = gl::GetAttribLocation(program, b"aBgColor\0".as_ptr() as *const i8);
            gl::GenBuffers(1, &mut bg_color_buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, bg_color_buffer);
            gl::BufferData(gl::ARRAY_BUFFER, (bg_color.len() * size_of::<f32>()) as isize, bg_color.as_ptr() as *const c_void, gl::DYNAMIC_DRAW);
            gl::VertexAttribPointer(bg_color_attrib_location as GLuint, 4, gl::FLOAT, gl::FALSE, 0, ptr::null::<c_void>());
            gl::EnableVertexAttribArray(bg_color_attrib_location as GLuint);

            gl::GenBuffers(1, &mut indices_buffer);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, indices_buffer);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indices.len() * size_of::<f32>()) as isize, indices.as_ptr() as *const c_void, gl::STATIC_DRAW);
        }

        let cache_glyph = CacheGlyph::new();
        let quads = vec![Quad {
            char: ' ',
            fg_color: [1.0, 1.0, 1.0, 1.0],
            bg_color: [0.0, 0.0, 0.0, 1.0]
        }; (width * height) as usize];

        Grid {
            width,
            height,
            vao,
            program,
            nb_vertex: i_b_count as i32,
            texture_coordinate_buffer,
            texture_coordinate,
            fg_color_buffer,
            fg_color,
            bg_color_buffer,
            bg_color,
            cache_glyph,
            quads
        }
    }

    pub unsafe fn draw(&mut self) {

        for i in 0..(self.width * self.height) {
            let rect = self.cache_glyph.get_uv_layout(self.quads[i as usize].char);
            let offset_tc: usize = (i * 8) as usize;
            let offset_c: usize = (i * 16) as usize;
            let texture_coordinate = &mut self.texture_coordinate;
            texture_coordinate[offset_tc]     = rect.max.x; texture_coordinate[offset_tc + 1] = rect.max.y;
            texture_coordinate[offset_tc + 2] = rect.max.x; texture_coordinate[offset_tc + 3] = rect.min.y;
            texture_coordinate[offset_tc + 4] = rect.min.x; texture_coordinate[offset_tc + 5] = rect.min.y;
            texture_coordinate[offset_tc + 6] = rect.min.x; texture_coordinate[offset_tc + 7] = rect.max.y;
            let fg_color = &mut self.fg_color;
            let fg_color_t = self.quads[i as usize].fg_color;
            let bg_color = &mut self.bg_color;
            let bg_color_t = self.quads[i as usize].bg_color;
            for i in 0..4 {
                bg_color[offset_c + i * 4]     = bg_color_t[0];
                bg_color[offset_c + i * 4 + 1] = bg_color_t[1];
                bg_color[offset_c + i * 4 + 2] = bg_color_t[2];
                bg_color[offset_c + i * 4 + 3] = bg_color_t[3];
                fg_color[offset_c + i * 4]     = fg_color_t[0];
                fg_color[offset_c + i * 4 + 1] = fg_color_t[1];
                fg_color[offset_c + i * 4 + 2] = fg_color_t[2];
                fg_color[offset_c + i * 4 + 3] = fg_color_t[3];
            }
        }

        gl::BindBuffer(gl::ARRAY_BUFFER, self.texture_coordinate_buffer);
        gl::BufferSubData(gl::ARRAY_BUFFER, 0, (self.texture_coordinate.len() * size_of::<f32>()) as isize, self.texture_coordinate.as_ptr() as *const c_void);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.fg_color_buffer);
        gl::BufferSubData(gl::ARRAY_BUFFER, 0, (self.fg_color.len() * size_of::<f32>()) as isize, self.fg_color.as_ptr() as *const c_void);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.bg_color_buffer);
        gl::BufferSubData(gl::ARRAY_BUFFER, 0, (self.bg_color.len() * size_of::<f32>()) as isize, self.bg_color.as_ptr() as *const c_void);

        self.cache_glyph.update_texture();
        gl::UseProgram(self.program);
        gl::BindVertexArray(self.vao);
        gl::BindTexture(gl::TEXTURE_2D, self.cache_glyph.texture);
        gl::DrawElements(gl::TRIANGLES, self.nb_vertex, gl::UNSIGNED_INT, ptr::null());
    }

    /*
    Modification
     */

    pub fn clear(&mut self) {
        for char in self.quads.as_mut_slice() {
            char.switch_char(' ');
            char.switch_fg_color([1.0, 1.0, 1.0, 1.0]);
            char.switch_bg_color([0.0, 0.0, 0.0, 1.0]);
        }
    }

    pub fn clear_char(&mut self) {
        for char in self.quads.as_mut_slice() {
            char.switch_char(' ');
        }
    }

    pub fn clear_fg_color(&mut self) {
        for char in self.quads.as_mut_slice() {
            char.switch_fg_color([1.0, 1.0, 1.0, 1.0]);
        }
    }

    pub fn clear_bg_color(&mut self) {
        for quad in self.quads.as_mut_slice() {
            quad.switch_bg_color([0.0, 0.0, 0.0, 1.0]);
        }
    }

    pub fn write_at(&mut self, x: i32, y: i32, text: &str) {
        let text_vec: Vec<char> = text.chars().collect();
        let mut text_len = text_vec.len() as i32;
        let start_position = y * self.width as i32 + x;
        if start_position + text_len > self.quads.len() as i32 - 1 {
            let to_trim = start_position + (text_len - 1) - (self.quads.len() as i32 - 1);
            text_len -= to_trim;
        }
        for text_index in 0..text_len {
            let char = &mut self.quads[(start_position + text_index) as usize];
            char.switch_char(text_vec[text_index as usize]);
        }
    }

    pub fn write_box(&mut self, x_start: i32, y_start: i32, x_end: i32, y_end: i32, box_style: BoxDrawing) {
        let char = self.quads.as_mut_slice();
        let (h_line, v_line, l_l_corner, u_l_corner, l_r_corner, u_r_corner) = BoxDrawing::get_char(box_style);
        for x in x_start..=x_end {
            for y in y_start..=y_end {
                let index = (y * self.width as i32 + x) as usize;
                if (x != x_start && x != x_end && y != y_start && y != y_end) || index > (self.width * self.height) as usize {
                    continue;
                }
                if x == x_start && y == y_start {
                    char[index].switch_char(l_l_corner);
                }
                else if x == x_end && y == y_end {
                    char[index].switch_char(u_r_corner);
                }
                else if x == x_start && y == y_end {
                    char[index].switch_char(u_l_corner);
                }
                else if x == x_end && y == y_start {
                    char[index].switch_char(l_r_corner);
                }
                else if x == x_start || x == x_end {
                    char[index].switch_char(v_line);
                }
                else if y == y_start || y == y_end {
                    char[index].switch_char(h_line);
                }
            }
        }
    }

    pub fn inverse_color_at(&mut self, x: i32, y: i32) {
        let quad = &mut self.quads.as_mut_slice()[(x + y * self.width as i32) as usize];
        std::mem::swap(&mut quad.fg_color, &mut quad.bg_color);
    }

    pub fn switch_bg_at(&mut self, x: i32, y: i32, color: [f32;4]) {
        self.quads.as_mut_slice()[(x + y * self.width as i32) as usize].switch_bg_color(color);
    }

    pub fn shuffle_glyph(&mut self) {
        let mut rng = thread_rng();
        for quad in self.quads.as_mut_slice() {
            let char = char::from_u32((rng.gen::<f32>() * 128.0) as u32).unwrap_or('�');
            let fg_color = [rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>(), 1.0];
            let bg_color = [rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>(), 1.0];
            quad.switch_char(char);
            quad.switch_fg_color(fg_color);
            quad.switch_bg_color(bg_color);
        }
    }
}