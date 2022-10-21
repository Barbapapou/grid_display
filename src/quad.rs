use std::mem::size_of;
use std::os::raw::c_void;
use std::ptr;
use gl::types::*;

pub struct Quad {
    vao: u32,
    program: GLuint,
    u_color_location: GLint,
    color: [f32; 4],
}

impl Quad {
    pub fn new(start_x:f32, end_x:f32, start_y:f32, end_y:f32, color: [f32; 4], program: GLuint) -> Quad {
        let vertices: [f32; 12] = [
            end_x  ,  end_y  , 0.0,
            end_x  ,  start_y, 0.0,
            start_x,  start_y, 0.0,
            start_x,  end_y  , 0.0
        ];

        let indices: [u32; 6] = [
            0, 1, 3,
            1, 2, 3
        ];

        let mut vbo: u32 = 0;
        let mut ebo: u32 = 0;
        let mut vao: u32 = 0;
        let u_color_location: GLint;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * size_of::<f32>()) as isize, vertices.as_ptr() as *const c_void, gl::STATIC_DRAW);

            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indices.len() * size_of::<f32>()) as isize, indices.as_ptr() as *const c_void, gl::STATIC_DRAW);

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (3 * size_of::<f32>()) as i32, ptr::null::<c_void>());
            gl::EnableVertexAttribArray(0);

            u_color_location = gl::GetUniformLocation(program, b"uColor\0".as_ptr() as *const GLchar);
        }

        Quad {
            vao,
            program,
            u_color_location,
            color
        }
    }

    pub unsafe fn draw(&self) {
        gl::BindVertexArray(self.vao);
        gl::UseProgram(self.program);
        gl::Uniform4f(self.u_color_location, self.color[0], self.color[1], self.color[2], self.color[3]);
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        gl::BindVertexArray(0);
    }
}