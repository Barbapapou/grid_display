use std::collections::HashMap;
use std::ffi::c_void;
use rusttype::{Point, Rect, Scale};
use crate::UNIFONT;
use crate::util::rgba8::RGBA8;
use crate::util::vector::{Vector2f, UvLayout};

const SCALE_GLYPH: Scale = Scale{x:16.0, y:16.0};

pub struct CacheGlyph {
    char_to_rect: HashMap<char, UvLayout>,
    img: Vec<RGBA8>,
    img_width: u32,
    img_height: u32,
    pub texture: u32,
    nbr_glyph: u32,
    is_dirty: bool,
}

impl CacheGlyph {
    pub fn new() -> CacheGlyph {
        let img_width: u32 = 1024;
        let img_height: u32 = 1024;

        let char_to_rect = HashMap::new();
        let img = vec![RGBA8{r:0, b:0, g:0, a:255}; (img_width * img_height) as usize];

        let mut texture: u32 = 0;
        unsafe {
            gl::GenTextures(1, &mut texture);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, img_width as i32, img_height as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, img.as_ptr() as *const c_void);
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        CacheGlyph {
            char_to_rect,
            img,
            img_width,
            img_height,
            texture,
            nbr_glyph: 0,
            is_dirty: true,
        }
    }

    pub fn update_texture(&mut self) {
        if self.is_dirty {
            unsafe {
                gl::BindTexture(gl::TEXTURE_2D, self.texture);
                gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, self.img_width as i32, self.img_height as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, self.img.as_ptr() as *const c_void);
                gl::GenerateMipmap(gl::TEXTURE_2D);
            }
            self.is_dirty = false;
        }
    }

    pub fn get_uv_layout(&mut self, c: char) -> UvLayout {
        let mut uv_layout = self.char_to_rect.get(&c);
        let temp;
        if uv_layout.is_none() {
            temp = self.gen_new_layout(c);
            uv_layout = Some(&temp);
            self.is_dirty = true;
        }
        *uv_layout.unwrap()
    }

    fn gen_new_layout(&mut self, c: char) -> UvLayout {
        let font = unsafe {UNIFONT.as_ref().unwrap()};
        let v_metrics = font.v_metrics(SCALE_GLYPH);
        let position = Point {x: 0.0, y: v_metrics.ascent};
        let glyph = font.glyph(c).scaled(SCALE_GLYPH).positioned(position);
        let bounding_box = glyph.pixel_bounding_box().unwrap_or(Rect{min: Point{x:0, y:0}, max: Point{x:0, y:0}});
        let glyph_width = bounding_box.width();
        let glyph_height = bounding_box.height();
        let glyph_offset_x = bounding_box.min.x.abs();
        let glyph_offset_y = bounding_box.min.y;
        println!("{c}, min_x: {glyph_offset_x}, min_y: {glyph_offset_y}, width: {glyph_width}, height: {glyph_height}");
        let x_o = self.nbr_glyph * 8 % self.img_width;
        let y_o = self.nbr_glyph * 8 / self.img_width * 16;
        glyph.draw(|x, y, v| {
            let x_c = x + glyph_offset_x as u32;
            let y_c = 15 - (y + glyph_offset_y as u32); // 16 - 1 == glyph max height
            if x_o + x_c > self.img_width - 1 || y_o + y_c > self.img_height - 1 { return }
            let color = if v > 0.0 {RGBA8{r:255, b:255, g:255, a:255}}
            else {RGBA8{r:0, b:0, g:0, a:255}};
            self.img[((x_o + x_c) + (y_o + y_c) * self.img_width) as usize] = color;
        });
        self.nbr_glyph += 1;

        let min = Vector2f {x: x_o as f32 / self.img_width as f32, y: y_o as f32 / self.img_height as f32 };
        let max = Vector2f {x: (x_o + 8) as f32 / self.img_width as f32, y: (y_o + 16) as f32 / self.img_height as f32 };

        let uv_layout = UvLayout {
            min,
            max
        };

        self.char_to_rect.insert(c, uv_layout);
        uv_layout
    }
}