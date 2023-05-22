use substring::Substring;

#[derive(Copy, Clone)]
pub struct RGBA8
{
    pub r: u8,
    pub b: u8,
    pub g: u8,
    pub a: u8,
}

impl RGBA8 {
    #[inline]
    pub fn new (r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {r, g, b, a}
    }

    pub fn from_hex_string(input: &String) -> Self {
        let r = input.substring(0, 2);
        let r = hex::decode(r).expect("Failed to decode the red component of the color");
        let r = r.iter().enumerate().map(|(i, v)| v << (i * 8)).sum();
        let g = input.substring(2, 4);
        let g = hex::decode(g).expect("Failed to decode the green component of the color");
        let g = g.iter().enumerate().map(|(i, v)| v << (i * 8)).sum();
        let b = input.substring(4, 6);
        let b = hex::decode(b).expect("Failed to decode the blue component of the color");
        let b = b.iter().enumerate().map(|(i, v)| v << (i * 8)).sum();
        RGBA8 {r, g, b, a: 255}
    }
}

impl Into<[f32; 4]> for RGBA8 {
    fn into(self) -> [f32; 4] {
        [self.r as f32 / 255.0, self.g as f32 / 255.0, self.b as f32 / 255.0, self.a as f32 / 255.0]
    }
}
