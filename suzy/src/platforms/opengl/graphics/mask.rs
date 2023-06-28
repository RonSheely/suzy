/* SPDX-License-Identifier: (Apache-2.0 OR MIT OR Zlib) */
/* Copyright © 2023 Violet Leonard */

use crate::{
    graphics::{DrawContext, Graphic},
    platforms::opengl,
    widget::WidgetGraphic,
};

use opengl::OpenGlRenderPlatform;

#[derive(Clone, Copy, Debug, Default)]
pub struct Mask<T> {
    pub graphic: T,
}

pub struct MaskPush<'a, T> {
    mask: &'a mut Mask<T>,
}

pub struct MaskPop;

impl<T> WidgetGraphic<OpenGlRenderPlatform> for Mask<T>
where
    T: Graphic<OpenGlRenderPlatform>,
{
    type BeforeGetter = fn(&mut ()) -> MaskPush<'_, T>;

    type AfterGetter = fn(&mut ()) -> MaskPop;

    fn before_children(&mut self) -> MaskPush<'_, T> {
        MaskPush { mask: self }
    }

    fn after_children(&mut self) -> MaskPop {
        MaskPop
    }
}

impl<T> Graphic<OpenGlRenderPlatform> for MaskPush<'_, T>
where
    T: Graphic<OpenGlRenderPlatform>,
{
    fn draw(&mut self, ctx: &mut DrawContext<'_, OpenGlRenderPlatform>) {
        ctx.push_mask();
        self.mask.graphic.draw(ctx);
        ctx.start_masking();
    }
}

impl Graphic<OpenGlRenderPlatform> for MaskPop {
    fn draw(&mut self, ctx: &mut DrawContext<'_, OpenGlRenderPlatform>) {
        ctx.pop_mask();
    }
}
