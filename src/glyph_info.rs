
use std::collections::{HashMap};
use std::os::raw::c_void;
use image::{DynamicImage, GenericImage, Rgba};
use rusttype::{Font, Point, Rect, Scale};

const IMG_WIDTH: i32 = 32;
const IMG_HEIGHT: i32 = 32;
const SCALE_GLYPH: Scale = Scale { x: 32.0, y: 32.0 };

pub static mut GLYPH_CACHE: Option<HashMap<char, GlyphInfo>> = None;
pub static mut UNIFONT: Option<Font> = None;

#[derive(Clone, Copy)]
pub struct GlyphInfo {
    pub texture: u32,
}

impl GlyphInfo {

    pub fn get_glyph_texture(char: char) -> u32 {
        let glyph_cache = unsafe { GLYPH_CACHE.as_mut().unwrap() };
        let mut glyph_info = glyph_cache.get(&char);
        let temp; // avoid lifetime issues
        if glyph_info.is_none() {
            temp = GlyphInfo::generate_new_entry(char);
            glyph_info = Some(&temp);
        }
        glyph_info.unwrap().texture
    }

    pub fn generate_new_entry(char: char) -> GlyphInfo {
        let glyph = unsafe { UNIFONT.as_ref().unwrap().glyph(char).scaled(SCALE_GLYPH).positioned(Point{x:0.0, y:0.0}) };
        let mut img = DynamicImage::new_rgba8(IMG_WIDTH as u32, IMG_HEIGHT as u32);
        let bounding_box = glyph.pixel_bounding_box().unwrap_or(Rect{ min: Point { x: 0, y: 0 },  max: Point { x: 0, y: 0 }});
        let glyph_width = bounding_box.width();
        let glyph_height = bounding_box.height();
        let mut glyph_offset_x = (IMG_WIDTH - glyph_width) / 2;
        let mut glyph_offset_y = (IMG_HEIGHT - glyph_height) / 2;
        let codepoint = char as u32;
        if (0x2500..=0x257F).contains(&codepoint) {
            glyph_offset_x = 0;
            glyph_offset_y = 0;
        }
        glyph.draw(|x, y, v| {
            let x_c = x + glyph_offset_x as u32;
            let y_c = (IMG_HEIGHT - 1) as u32 - (y + glyph_offset_y as u32);
            let color = if v > 0.0 {
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

        let mut texture: u32 = 0;
        unsafe {
            gl::GenTextures(1, &mut texture);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST_MIPMAP_LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, IMG_WIDTH, IMG_HEIGHT, 0, gl::RGBA, gl::UNSIGNED_BYTE, img.as_bytes().as_ptr() as *const c_void);
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        let result = GlyphInfo {
            texture,
        };
        let glyph_cache = unsafe { GLYPH_CACHE.as_mut().unwrap() };
        glyph_cache.insert(char, result);
        result
    }
}