use std::path::Path;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::convert::TryInto;

use sdl2::surface::Surface;
use sdl2::image::LoadSurface;
use sdl2::pixels::PixelFormatEnum;

use crate::platform::opengl::image::{
    Texture, TextureLoader, TextureLoadResult, TextureBuilder
};

#[derive(Debug)]
struct SdlImageLoadError(String);

impl Display for SdlImageLoadError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        std::fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for SdlImageLoadError {}

impl From<String> for SdlImageLoadError {
    fn from(msg: String) -> Self { Self(msg) }
}

pub fn load_texture(path: &Path) -> TextureLoadResult {
    let surf = Surface::from_file(path)
        .map_err(|msg| Box::new(SdlImageLoadError(msg)))?;
    let (fmt, type_, surf, opaque) = match surf.pixel_format_enum() {
        PixelFormatEnum::RGB24 => (gl::RGB, gl::UNSIGNED_BYTE, surf, true),
        PixelFormatEnum::BGR24 => (gl::BGR, gl::UNSIGNED_BYTE, surf, true),
        PixelFormatEnum::RGBA8888 => {
            (gl::RGBA, gl::UNSIGNED_BYTE, surf, false)
        },
        PixelFormatEnum::BGRA8888 => {
            (gl::BGRA, gl::UNSIGNED_BYTE, surf, false)
        },
        _ => {
            let conv = surf.convert_format(PixelFormatEnum::BGRA8888)
                .map_err(|msg| Box::new(SdlImageLoadError(msg)))?;
            (gl::BGRA, gl::UNSIGNED_BYTE, conv, false)
        },
    };
    let (width, height) = surf.size();
    let width = width.try_into().map_err(Box::new)?;
    let height = height.try_into().map_err(Box::new)?;
    let mut builder = if opaque {
        TextureBuilder::create_opaque(width, height)
    } else {
        TextureBuilder::create(width, height)
    };
    surf.with_lock(|pixels| {
        builder.sub_image(
            0, 0,
            width, height,
            fmt, type_, 
            pixels.as_ptr() as *const _,
        );
    });
    Ok(builder.build())
}
