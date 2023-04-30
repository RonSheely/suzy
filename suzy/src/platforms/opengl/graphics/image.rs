/* SPDX-License-Identifier: (Apache-2.0 OR MIT OR Zlib) */
/* Copyright © 2021 Violet Leonard */

use crate::{
    dims::{Dim, Padding2d, Rect, SimplePadding2d, SimpleRect},
    graphics::{DrawContext, Graphic},
    platforms::opengl,
};

use opengl::context::bindings::{FALSE, FLOAT, TRIANGLES, UNSIGNED_BYTE};
use opengl::{DualVertexBufferIndexed, OpenGlRenderPlatform, Texture};

#[rustfmt::skip]
static SLICED_INDICES: [u8; 18 * 3] = [
    12, 13, 15,
    13, 14, 15,
    0, 4, 11,
    4, 12, 11,
    4, 5, 12,
    5, 13, 12,
    5, 1, 13,
    1, 6, 13,
    11, 12, 10,
    12, 15, 10,
    13, 6, 14,
    6, 7, 14,
    10, 15, 3,
    15, 9, 3,
    15, 14, 9,
    14, 8, 9,
    14, 7, 8,
    7, 2, 8,
];

/// A common graphic used for user interfaces, a sliced image is defined by
/// fixed-sized corners and an inner area which stretches to fill the
/// graphic area.
///
/// See the [Wikipedia article](https://en.wikipedia.org/wiki/9-slice_scaling)
/// on 9-slice scaling for more information.
pub struct SlicedImage {
    rect: SimpleRect,
    padding: SimplePadding2d,
    texture: Texture,
    buffers: DualVertexBufferIndexed<f32>,
}

impl Default for SlicedImage {
    fn default() -> Self {
        Self {
            rect: SimpleRect::default(),
            padding: SimplePadding2d::default(),
            texture: Texture::default(),
            buffers: DualVertexBufferIndexed::new(true, false, false),
        }
    }
}

impl SlicedImage {
    /// Create a new SlicedImage.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the texture used by this graphic.  The given padding describes
    /// the sliced area.
    pub fn set_image(&mut self, texture: Texture, padding: SimplePadding2d) {
        self.texture = texture;
        self.padding = padding;
        self.update_image();
    }

    fn indicies_to_draw(&self) -> opengl::context::bindings::types::GLsizei {
        if self.padding == SimplePadding2d::zero() {
            6
        } else {
            SLICED_INDICES.len() as _
        }
    }

    fn update_image(&mut self) {
        let mut uvs = [0f32; 32];
        let Self {
            buffers,
            texture,
            padding,
            ..
        } = self;
        buffers.set_data_1(|_gl| {
            texture.size().and_then(|(tex_width, tex_height)| {
                let uvs = &mut uvs;
                texture.transform_uvs(move || {
                    let left = padding.left() / tex_width;
                    let right = 1.0 - (padding.right() / tex_width);
                    let bottom = padding.bottom() / tex_height;
                    let top = 1.0 - (padding.top() / tex_height);
                    #[rustfmt::skip]
                    let data = [
                        0.0, 0.0,
                        1.0, 0.0,
                        1.0, 1.0,
                        0.0, 1.0,
                        left, 0.0,
                        right, 0.0,
                        1.0, bottom,
                        1.0, top,
                        right, 1.0,
                        left, 1.0,
                        0.0, top,
                        0.0, bottom,
                        left, bottom,
                        right, bottom,
                        right, top,
                        left, top,
                    ];
                    *uvs = data;
                    &mut uvs[..]
                })
            })
        });
    }

    fn update(&mut self) {
        let mut inner = SimpleRect::default();
        inner.set_fill(&self.rect, &self.padding);
        let rect = &self.rect;
        let mut vertices = [0f32; 32];
        self.buffers.set_data_0(|_gl| {
            #[rustfmt::skip]
            let data = [
                // outer corners
                rect.left(), rect.bottom(),
                rect.right(), rect.bottom(),
                rect.right(), rect.top(),
                rect.left(), rect.top(),
                // bottom edge
                inner.left(), rect.bottom(),
                inner.right(), rect.bottom(),
                // right edge
                rect.right(), inner.bottom(),
                rect.right(), inner.top(),
                // top edge
                inner.right(), rect.top(),
                inner.left(), rect.top(),
                // left edge
                rect.left(), inner.top(),
                rect.left(), inner.bottom(),
                // inner corners
                inner.left(), inner.bottom(),
                inner.right(), inner.bottom(),
                inner.right(), inner.top(),
                inner.left(), inner.top(),
            ];
            vertices = data;
            &vertices[..]
        });
    }
}

impl Rect for SlicedImage {
    fn x(&self) -> Dim {
        self.rect.x()
    }
    fn y(&self) -> Dim {
        self.rect.y()
    }

    fn x_mut<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut Dim) -> R,
    {
        let res = self.rect.x_mut(f);
        self.update();
        res
    }

    fn y_mut<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut Dim) -> R,
    {
        let res = self.rect.y_mut(f);
        self.update();
        res
    }
}

impl Graphic<OpenGlRenderPlatform> for SlicedImage {
    fn draw(&mut self, ctx: &mut DrawContext<'_, OpenGlRenderPlatform>) {
        ctx.push(|ctx| {
            ctx.params().standard_mode();
            ctx.params().use_texture(self.texture.clone());
            let indicies_to_draw = self.indicies_to_draw();
            if let Some(ready) = self.buffers.check_ready(ctx) {
                let gl = ready.gl;
                ready.bind_0();
                unsafe {
                    gl.VertexAttribPointer(
                        0,
                        2,
                        FLOAT,
                        FALSE,
                        0,
                        std::ptr::null(),
                    );
                }
                ready.bind_1();
                unsafe {
                    gl.VertexAttribPointer(
                        1,
                        2,
                        FLOAT,
                        FALSE,
                        0,
                        std::ptr::null(),
                    );
                }
                ready.bind_indices();
                unsafe {
                    gl.DrawElements(
                        TRIANGLES,
                        indicies_to_draw,
                        UNSIGNED_BYTE,
                        std::ptr::null(),
                    );
                }
            } else {
                self.update();
                self.texture.bind(ctx.render_ctx_mut());
                self.update_image();
                self.buffers.set_indices(|_gl| &SLICED_INDICES[..]);
            }
        });
    }
}

impl crate::platform::graphics::SlicedImage for SlicedImage {
    fn set_color(&mut self, _color: crate::graphics::Color) {
        todo!()
    }

    fn set_slice_padding(&mut self, padding: impl Padding2d) {
        self.padding = (&padding).into();
        self.update();
    }

    fn set_corners(&mut self, _style: crate::graphics::CornerStyle) {
        todo!()
    }
}
