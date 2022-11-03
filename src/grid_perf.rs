use std::ffi::c_void;
use std::mem::size_of;
use std::ptr;
use rusttype::gpu_cache::Cache;
use gl::types::*;
use image::{DynamicImage, GenericImage, Rgba};
use rusttype::{Point, Scale};
use crate::UNIFONT;

pub struct GridPerf<'grid> {
    vao: u32,
    program: u32,
    nb_vertex: i32,
    vertex_position_buffer: u32,
    texture_coordinate_buffer: u32,
    indices_buffer: u32,
    vertex_position_attrib_location: GLint,
    texture_coordinate_attrib_location: GLint,
    texture: u32,
    cache: Cache<'grid>,

}

impl<'grid> GridPerf<'grid> {
    pub fn new(width: u32, height: u32, program: u32) -> GridPerf<'grid> {
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

        let mut vao: u32 = 0;
        let mut vertex_position_buffer: u32 = 0;
        let mut texture_coordinate_buffer: u32 = 0;
        let mut indices_buffer: u32 = 0;
        let mut vertex_position_attrib_location: GLint = 0;
        let mut texture_coordinate_attrib_location: GLint = 0;

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

            gl::GenBuffers(1, &mut indices_buffer);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, indices_buffer);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indices.len() * size_of::<f32>()) as isize, indices.as_ptr() as *const c_void, gl::STATIC_DRAW);
        }

        let tex_width = 1024;
        let tex_height = 1024;

        let mut cache = Cache::builder().dimensions(tex_width, tex_height).build();
        let mut img = DynamicImage::new_rgba8(tex_width, tex_height);
        const SCALE_GLYPH: Scale = Scale { x: 16.0, y: 16.0 };
        let font = unsafe {UNIFONT.as_ref().unwrap()};
        let v_metrics = font.v_metrics(SCALE_GLYPH);
        let position = Point {x: 0.0, y: v_metrics.ascent};
        let glyph = font.glyph('a').scaled(SCALE_GLYPH).positioned(position);
        cache.queue_glyph(0, glyph.clone());
        cache.cache_queued(|rect, data| {
            for (i, v) in data.iter().enumerate() {
                let x = rect.min.x + (i as u32 % rect.width()) as u32;
                let y = rect.min.y + (i as u32 / rect.width()) as u32;
                img.put_pixel(x, y, Rgba([*v, *v, *v, *v]))
            }
        }).unwrap();

        let mut texture = 0;

        unsafe {
            gl::GenTextures(1, &mut texture);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, tex_width as i32, tex_height as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, img.as_bytes().as_ptr() as *const c_void);
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        // TEMP

        let rect = cache.rect_for(0, &glyph).unwrap().unwrap().0;
        let texture_coordinate_t: [f32; 8] = [
            rect.max.x, rect.min.y,
            rect.max.x, rect.max.y,
            rect.min.x, rect.max.y,
            rect.min.x, rect.min.y
        ];
        texture_coordinate[0..8].copy_from_slice(&texture_coordinate_t);

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, texture_coordinate_buffer);
            gl::BufferSubData(gl::ARRAY_BUFFER, 0, (8 * size_of::<f32>()) as isize, texture_coordinate.as_ptr() as *const c_void);
        }

        img.save("img.png").expect("TODO: panic message");

        GridPerf {
            vao,
            program,
            nb_vertex: i_b_count as i32,
            vertex_position_buffer,
            texture_coordinate_buffer,
            indices_buffer,
            vertex_position_attrib_location,
            texture_coordinate_attrib_location,
            texture,
            cache
        }
    }

    pub unsafe fn draw(&mut self) {
        gl::UseProgram(self.program);
        gl::BindVertexArray(self.vao);
        gl::BindTexture(gl::TEXTURE_2D, self.texture);
        gl::DrawElements(gl::TRIANGLES, self.nb_vertex, gl::UNSIGNED_INT, ptr::null());
    }
}