use std::mem::size_of;
use std::os::raw::c_void;
use std::ptr;
use gl::types::*;
use image::{DynamicImage, GenericImage, Rgba};
use rusttype::{Font, Point, Rect, Scale};

pub struct Quad {
    vao: u32,
    program: GLuint,
    u_color_location: GLint,
    color: [f32; 4],
    texture: GLuint,
}

impl Quad {
    pub fn new(start_x:f32, end_x:f32, start_y:f32, end_y:f32, color: [f32; 4], program: GLuint, font: &Font, char: char) -> Quad {
        let vertices: [f32; 20] = [
            end_x  ,  end_y  , 0.0, 1.0, 1.0,
            end_x  ,  start_y, 0.0, 1.0, 0.0,
            start_x,  start_y, 0.0, 0.0, 0.0,
            start_x,  end_y  , 0.0, 0.0, 1.0
        ];

        let indices: [u32; 6] = [
            0, 1, 3,
            1, 2, 3
        ];

        let mut vbo: u32 = 0;
        let mut ebo: u32 = 0;
        let mut vao: u32 = 0;
        let u_color_location: GLint;
        let mut texture: u32 = 0;

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

            u_color_location = gl::GetUniformLocation(program, b"uColor\0".as_ptr() as *const GLchar);

            let img_width = 128;
            let img_height = 128;
            let scale = Scale::uniform(128.0);

            let glyph = font.glyph(char).scaled(scale).positioned(Point{x:0.0, y:0.0});
            let mut img = DynamicImage::new_rgba8(img_width as u32, img_height as u32);
            let bounding_box = glyph.pixel_bounding_box().unwrap_or(Rect{ min: Point { x: 0, y: 0 },  max: Point { x: 0, y: 0 }});
            let glyph_width = bounding_box.width();
            let glyph_height = bounding_box.height();
            println!("char:{char}, width:{glyph_width}, height:{glyph_height}");
            let glyph_offset_x = (img_width - glyph_width) / 2;
            let glyph_offset_y = (img_height - glyph_height) / 2;

            glyph.draw(|x, y, v| {
                let x_c = x + glyph_offset_x as u32;
                let y_c = (img_height - 1) as u32 - (y + glyph_offset_y as u32);
                let color = if v > 0.001 {
                    Rgba([255, 255, 255, 255])
                } else {
                    Rgba([0, 0, 0, 255])
                };
                img.put_pixel(
                    x_c,
                    y_c,
                    color,
                )
            });

            gl::GenTextures(1, &mut texture);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, img_width, img_height, 0, gl::RGBA, gl::UNSIGNED_BYTE, img.as_bytes().as_ptr() as *const c_void);
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Quad {
            vao,
            program,
            u_color_location,
            color,
            texture
        }
    }

    pub unsafe fn draw(&self) {
        gl::BindVertexArray(self.vao);
        gl::UseProgram(self.program);
        gl::Uniform4f(self.u_color_location, self.color[0], self.color[1], self.color[2], self.color[3]);
        gl::BindTexture(gl::TEXTURE_2D, self.texture);
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        gl::BindVertexArray(0);
    }
}