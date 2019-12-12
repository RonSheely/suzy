
use super::Lerp;

#[derive(Copy, Clone, PartialEq)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

const MAX8: f32 = std::u8::MAX as f32;

impl Color {
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color { r, g, b, a }
    }

    pub fn rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color {
            r: (r as f32) / MAX8,
            g: (g as f32) / MAX8,
            b: (b as f32) / MAX8,
            a: (a as f32) / MAX8,
        }
    }

    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::rgba(r, g, b, 1.0)
    }

    pub fn rgb8(r: u8, g: u8, b: u8) -> Self {
        Self::rgba8(r, g, b, u8::max_value())
    }
}

impl Lerp for Color {
    type Output = Color;
    fn lerp(from: Self, to: Self, t: f32) -> Self::Output {
        Color {
            r: Lerp::lerp(from.r, to.r, t),
            g: Lerp::lerp(from.g, to.g, t),
            b: Lerp::lerp(from.b, to.b, t),
            a: Lerp::lerp(from.a, to.a, t),
        }
    }
}

impl Lerp for &Color {
    type Output = Color;
    fn lerp(from: Self, to: Self, t: f32) -> Self::Output {
        Color {
            r: Lerp::lerp(from.r, to.r, t),
            g: Lerp::lerp(from.g, to.g, t),
            b: Lerp::lerp(from.b, to.b, t),
            a: Lerp::lerp(from.a, to.a, t),
        }
    }
}

impl From<u32> for Color {
    fn from(code: u32) -> Self {
        let array = code.to_be_bytes();
        Color::rgba8(array[1], array[2], array[3], array[0])
    }
}

pub struct ParseColorError {
}

impl From<std::num::ParseIntError> for ParseColorError {
    fn from(orig: std::num::ParseIntError) -> Self { Self { } }
}

#[cfg(feature = "lookup_consts")]
pub use super::consts::COLOR_NAMES;

impl std::str::FromStr for Color {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("#") {
            let hex_part = s.split_at(1).1;
            if (hex_part.len() == 6 || hex_part.len() == 8)
                && hex_part.chars().all(|c| c.is_ascii_hexdigit())
            {
                let mut int = u32::from_str_radix(hex_part, 16)?;
                if hex_part.len() == 6 {
                    int <<= 8;
                }
                Ok(int.into())
            } else {
                Err(ParseColorError {})
            }
        } else if cfg!(feature = "lookup_consts") {
            let lower = s.to_ascii_lowercase();
            if let Some(color) = COLOR_NAMES.get(lower.as_str()) {
                Ok(*color)
            } else {
                Err(ParseColorError {})
            }
        } else {
            Err(ParseColorError {})
        }
    }
}
