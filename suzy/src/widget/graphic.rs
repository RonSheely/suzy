/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::platform::{DefaultRenderPlatform, RenderPlatform};
use crate::graphics::Graphic;

pub trait WidgetGraphic<P = DefaultRenderPlatform>
where
    P: RenderPlatform,
{
    type Before: Graphic<P>;
    type After: Graphic<P>;

    fn before_children(&mut self) -> &mut Self::Before;
    fn after_children(&mut self) -> &mut Self::After;
}

impl<T: Graphic<P>, P: RenderPlatform> WidgetGraphic<P> for T
where Self: Sized
{
    type Before = Self;
    type After = [(); 0];

    fn before_children(&mut self) -> &mut Self { self }
    fn after_children(&mut self) -> &mut [(); 0] { &mut [] }
}