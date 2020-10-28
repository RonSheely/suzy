/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#![cfg(feature = "platform_opengl")]

extern crate suzy;

use suzy::dims::{
    Rect,
    SimplePadding2d,
    Padding,
};
use suzy::window::WindowSettings;
use suzy::graphics::Color;
use suzy::app::{
    App,
    AppBuilder,
};
use suzy::widget::{
    Widget,
    WidgetContent,
};
use suzy::platform::opengl::{
    OpenGlRenderPlatform,
    SlicedImage,
    Tint,
};
use suzy::platform::TestPlatform;

mod utils;
use utils::*;

#[derive(Default)]
struct Root {
    tint: Tint,
    image: SlicedImage,
}

impl WidgetContent<OpenGlRenderPlatform> for Root {
    fn init<I: suzy::widget::WidgetInit<Self, OpenGlRenderPlatform>>(mut init: I) {
        init.watch(|root, _rect| {
            root.tint.set_tint_color(Color::RED);
        });
        init.watch(|root, rect| {
            root.image.set_fill(&rect, &SimplePadding2d::zero());
        });
    }

    fn children<R: suzy::widget::WidgetChildReceiver<OpenGlRenderPlatform>>(&mut self, _receiver: R) {
    }

    fn graphics<R: suzy::widget::WidgetGraphicReceiver<OpenGlRenderPlatform>>(&mut self, mut receiver: R) {
        receiver.graphic(&mut self.tint);
        receiver.graphic(&mut self.image);
    }
}

#[test]
fn tint_red() {
    let mut builder = AppBuilder::default();
    builder.set_size((480.0, 360.0));
    builder.set_background_color(Color::BLACK);
    let app: App<TestPlatform> = builder.build();
    let app = app.with(|app| {
        app.add_root(Widget::<Root>::default);
    }).0;
    app.test(|mut app| {
        let capture = app.take_screenshot();
        assert!(is_color(&capture, Color::RED));
    });
}

