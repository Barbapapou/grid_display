#[derive(Copy, Clone)]
pub struct Quad {
    pub(crate) char: char,
    pub(crate) fg_color: [f32; 4],
    pub(crate) bg_color: [f32; 4],
}

impl Quad {
    #[inline]
    pub fn switch_char(&mut self, char: char) {
        self.char = char;
    }
    #[inline]
    pub fn switch_fg_color(&mut self, color: [f32; 4]){
        self.fg_color = color;
    }
    #[inline]
    pub fn switch_bg_color(&mut self, color: [f32; 4]){
        self.bg_color = color;
    }
}