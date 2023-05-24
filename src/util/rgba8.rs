use hex::FromHexError;
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

    pub fn from_hex_string(input: &String) -> Result<Self, FromHexError> {
        let r = input.substring(0, 2);
        let r = hex::decode(r)?;
        let r = r.iter().enumerate().map(|(i, v)| v << (i * 8)).sum();
        let g = input.substring(2, 4);
        let g = hex::decode(g)?;
        let g = g.iter().enumerate().map(|(i, v)| v << (i * 8)).sum();
        let b = input.substring(4, 6);
        let b = hex::decode(b)?;
        let b = b.iter().enumerate().map(|(i, v)| v << (i * 8)).sum();
        Ok(RGBA8 {r, g, b, a: 255})
    }
}

impl From<RGBA8> for [f32; 4] {
    fn from(value: RGBA8) -> Self {
        [
            value.r as f32 / 255.0,
            value.g as f32 / 255.0,
            value.b as f32 / 255.0,
            value.a as f32 / 255.0,
        ]
    }
}