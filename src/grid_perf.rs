use std::ffi::c_void;
use std::mem::size_of;
use std::ptr;

pub struct GridPerf {
    vao: u32,
    program: u32,
    nb_triangle: i32,
}

impl GridPerf {
    pub fn new(width: u32, height: u32, program: u32) -> GridPerf {
        let width_f = width as f32;
        let height_f = height as f32;

        let mut vertices: Vec<f32> = vec![0.0; (20 * width * height) as usize];
        let mut indices: Vec<u32> = vec![0; (6 * width * height) as usize];
        let mut v_b_count: usize = 0;
        let mut i_b_count: usize = 0;
        let mut v_count: u32 = 0;

        for y in 0..height {
            for x in 0..width {
                let start_x = ((x as f32)       / width_f ) * 2.0 - 1.0;
                let end_x =   ((x as f32 + 1.0) / width_f ) * 2.0 - 1.0;
                let start_y = ((y as f32)       / height_f) * 2.0 - 1.0;
                let end_y =   ((y as f32 + 1.0) / height_f) * 2.0 - 1.0;

                let vertices_t: [f32; 20] = [
                    end_x  , end_y  , 0.0, 1.0, 1.0,
                    end_x  , start_y, 0.0, 1.0, 0.0,
                    start_x, start_y, 0.0, 0.0, 0.0,
                    start_x, end_y  , 0.0, 0.0, 1.0
                ];

                let indices_t: [u32; 6] = [
                    v_count    , v_count + 1, v_count + 3,
                    v_count + 1, v_count + 2, v_count + 3
                ];

                vertices[v_b_count..(v_b_count + 20)].copy_from_slice(&vertices_t);
                indices[i_b_count..(i_b_count + 6)].copy_from_slice(&indices_t);

                i_b_count += 6;
                v_b_count += 20;
                v_count += 4;
            }
        }

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

            let stride = (5 * size_of::<f32>()) as i32;
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null::<c_void>());
            gl::EnableVertexAttribArray(0);

            let offset = (3 * size_of::<f32>()) as i32;
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, offset as *const c_void);
            gl::EnableVertexAttribArray(1);
        }

        GridPerf {
            vao,
            program,
            nb_triangle: (width * height * 6) as i32
        }
    }

    pub unsafe fn draw(&self) {
        gl::BindVertexArray(self.vao);
        gl::UseProgram(self.program);
        gl::DrawElements(gl::TRIANGLES, self.nb_triangle, gl::UNSIGNED_INT, ptr::null());
        gl::BindVertexArray(0);
    }
}