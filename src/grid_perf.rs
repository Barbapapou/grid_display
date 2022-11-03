use std::ffi::c_void;
use std::mem::size_of;
use std::ptr;
use rand::{Rng};
use rusttype::gpu_cache::Cache;
use gl::types::*;

pub struct GridPerf {
    program: u32,
    nb_vertex: i32,
    vertex_position_buffer: u32,
    texture_coordinate_buffer: u32,
    indices_buffer: u32,
    vertex_position_attrib_location: GLint,
    texture_coordinate_attrib_location: GLint,
}

impl GridPerf {
    pub fn new(width: u32, height: u32, program: u32) -> GridPerf {
        let width_f = width as f32;
        let height_f = height as f32;

        let mut vertex_position: Vec<f32> = vec![0.0; (12 * width * height) as usize];
        let mut texture_coordinate: Vec<f32> = vec![0.0; (8 * width * height) as usize];
        let mut indices: Vec<u32> = vec![0; (6 * width * height) as usize];
        let mut vp_b_count: usize = 0;
        let mut tc_b_count: usize = 0;
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

                let indices_t: [u32; 6] = [
                    v_count    , v_count + 1, v_count + 3,
                    v_count + 1, v_count + 2, v_count + 3
                ];
                indices[i_b_count..(i_b_count + 6)].copy_from_slice(&indices_t);

                vp_b_count += 12;
                tc_b_count += 8;
                i_b_count += 6;
                v_count += 4;
            }
        }

        let mut vertex_position_buffer: u32 = 0;
        let mut texture_coordinate_buffer: u32 = 0;
        let mut indices_buffer: u32 = 0;
        let mut vertex_position_attrib_location: GLint = 0;
        let mut texture_coordinate_attrib_location: GLint = 0;

        unsafe {
            vertex_position_attrib_location = gl::GetAttribLocation(program, b"aVertexPosition\0".as_ptr() as *const i8);
            gl::GenBuffers(1, &mut vertex_position_buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_position_buffer);
            gl::BufferData(gl::ARRAY_BUFFER, (vertex_position.len() * size_of::<f32>()) as isize, vertex_position.as_ptr() as *const c_void, gl::STATIC_DRAW);

            texture_coordinate_attrib_location = gl::GetAttribLocation(program, b"aTextureCoord\0".as_ptr() as *const i8);
            gl::GenBuffers(1, &mut texture_coordinate_buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, texture_coordinate_buffer);
            gl::BufferData(gl::ARRAY_BUFFER, (texture_coordinate.len() * size_of::<f32>()) as isize, texture_coordinate.as_ptr() as *const c_void, gl::DYNAMIC_DRAW);

            gl::GenBuffers(1, &mut indices_buffer);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, indices_buffer);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indices.len() * size_of::<f32>()) as isize, indices.as_ptr() as *const c_void, gl::STATIC_DRAW);
        }

        // let mut cache = Cache::builder().dimensions(1024, 1024).build();

        GridPerf {
            program,
            nb_vertex: i_b_count as i32,
            vertex_position_buffer,
            texture_coordinate_buffer,
            indices_buffer,
            vertex_position_attrib_location,
            texture_coordinate_attrib_location,
        }
    }

    pub unsafe fn draw(&mut self) {
        gl::UseProgram(self.program);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_position_buffer);
        gl::VertexAttribPointer(self.vertex_position_attrib_location as GLuint, 3, gl::FLOAT, gl::FALSE, 0, ptr::null::<c_void>());
        gl::EnableVertexAttribArray(self.vertex_position_attrib_location as GLuint);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.texture_coordinate_buffer);
        gl::VertexAttribPointer(self.texture_coordinate_attrib_location as GLuint, 2, gl::FLOAT, gl::FALSE, 0, ptr::null::<c_void>());
        gl::EnableVertexAttribArray(self.texture_coordinate_attrib_location as GLuint);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.indices_buffer);
        gl::DrawElements(gl::TRIANGLES, self.nb_vertex, gl::UNSIGNED_INT, ptr::null());
    }
}