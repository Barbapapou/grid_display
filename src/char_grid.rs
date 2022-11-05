use crate::glyph_info::GlyphInfo;

#[derive(Copy, Clone)]
pub struct CharGrid {
    pub(crate) char: char,
    pub(crate) fg_color: [f32; 4],
    pub(crate) bg_color: [f32; 4],
}

impl CharGrid {
    pub fn switch_char(&mut self, char: char) {
        self.char = char;
    }

    pub fn switch_fg_color(&mut self, color: [f32; 4]){
        self.fg_color = color;
    }

    pub fn switch_bg_color(&mut self, color: [f32; 4]){
        self.bg_color = color;
    }
}