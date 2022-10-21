use std::mem::size_of;
use std::os::raw::c_void;
use std::ptr;

pub struct Quad {
    vao: u32,
    vbo: u32,
    ebo: u32,
}

impl Quad {
    pub fn new() -> Quad {
        let vertices: [f32; 12] = [
            0.5,  0.5, 0.0,
            0.5, -0.5, 0.0,
            -0.5, -0.5, 0.0,
            -0.5,  0.5, 0.0
        ];

        let indices: [u32; 6] = [
            0, 1, 3,
            1, 2, 3
        ];

        let mut vbo: u32 = 0;
        let mut ebo: u32 = 0;
        let mut vao: u32 = 0;

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
        }

        Quad {
            vao,
            vbo,
            ebo,
        }
    }

    pub unsafe fn draw(&self) {
        gl::BindVertexArray(self.vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
    }
}