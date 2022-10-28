use std::mem::size_of;
use std::os::raw::c_void;
use std::ptr;
use gl::types::*;
use crate::glyph_info::GlyphInfo;

pub struct Quad {
    vao: u32,
    program: GLuint,
    u_fg_color_location: GLint,
    u_bg_color_location: GLint,
    pub fg_color: [f32; 4],
    pub bg_color: [f32; 4],
    texture: GLuint,
}

impl Quad {
    pub fn new(position: [f32; 4], fg_color: [f32; 4], bg_color: [f32; 4], program: GLuint, char: char) -> Quad {
        let vertices: [f32; 20] = [
            position[1],  position[3], 0.0, 1.0, 1.0,
            position[1],  position[2], 0.0, 1.0, 0.0,
            position[0],  position[2], 0.0, 0.0, 0.0,
            position[0],  position[3], 0.0, 0.0, 1.0
        ];

        let indices: [u32; 6] = [
            0, 1, 3,
            1, 2, 3
        ];

        let mut vbo: u32 = 0;
        let mut ebo: u32 = 0;
        let mut vao: u32 = 0;
        let u_fg_color_location: GLint;
        let u_bg_color_location: GLint;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * size_of::<f32>()) as isize, vertices.as_ptr() as *const c_void, gl::STATIC_DRAW);

            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indices.len() * size_of::<f32>()) as isize, indices.as_ptr() as *const c_void, gl::STATIC_DRAW);

            let stride = (5 * size_of::<f32>()) as i32;
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null::<c_void>());
            gl::EnableVertexAttribArray(0);

            let offset = (3 * size_of::<f32>()) as i32;
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, offset as *const c_void);
            gl::EnableVertexAttribArray(1);

            u_fg_color_location = gl::GetUniformLocation(program, b"uFgColor\0".as_ptr() as *const GLchar);
            u_bg_color_location = gl::GetUniformLocation(program, b"uBgColor\0".as_ptr() as *const GLchar);
        }

        let texture = GlyphInfo::get_glyph_texture(char);

        Quad {
            vao,
            program,
            u_fg_color_location,
            u_bg_color_location,
            fg_color,
            bg_color,
            texture
        }
    }

    pub unsafe fn draw(&self) {
        gl::BindVertexArray(self.vao);
        gl::UseProgram(self.program);
        gl::Uniform4f(self.u_fg_color_location, self.fg_color[0], self.fg_color[1], self.fg_color[2], self.fg_color[3]);
        gl::Uniform4f(self.u_bg_color_location, self.bg_color[0], self.bg_color[1], self.bg_color[2], self.bg_color[3]);
        gl::BindTexture(gl::TEXTURE_2D, self.texture);
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        gl::BindVertexArray(0);
    }

    pub fn switch_char(&mut self, char: char) {
        self.texture = GlyphInfo::get_glyph_texture(char);
    }

    pub fn switch_fg_color(&mut self, color: [f32; 4]){
        self.fg_color = color;
    }

    pub fn switch_bg_color(&mut self, color: [f32; 4]){
        self.bg_color = color;
    }
}