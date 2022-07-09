/* SPDX-License-Identifier: (Apache-2.0 OR MIT OR Zlib) */
/* Copyright © 2021 Violet Leonard */

#![allow(clippy::unimplemented)]

use crate::platform::{Event, SimpleEventLoopState};

macro_rules! stub {
    () => {
        unimplemented!("StubPlatform used at runtime")
    };
}

/// The stub platform is used as a placeholder when no other platforms are
/// enabled.  All its methods will panic.
#[derive(Default)]
pub struct StubPlatform;

/// The stub platform is used as a placeholder when no other platforms are
/// enabled.  All its methods will panic.
#[derive(Default)]
pub struct StubWindow;

/// The stub platform is used as a placeholder when no other platforms are
/// enabled.  All its methods will panic.
#[derive(Default)]
pub struct StubRenderPlatform;

/// The stub platform is used as a placeholder when no other platforms are
/// enabled.  All its methods will panic.
#[derive(Default)]
pub struct StubDrawContext;

/// The stub platform is used as a placeholder when no other platforms are
/// enabled.  All its methods will panic.
#[cfg(feature = "platform_opengl")]
#[derive(Default)]
pub struct StubOpenglPlatform;

impl crate::platform::RenderPlatform for StubRenderPlatform {
    type DrawPassInfo = ();
    type DrawContextBuilder = fn(&mut ()) -> StubDrawContext;

    type Texture = StubTexture;
    type SlicedImage = StubSlicedImage;
    type SelectableSlicedImage = StubSelectableSlicedImage;
    type Text = StubText;
    type TextEdit = StubTextEdit;
}

impl crate::graphics::PlatformDrawContext<()> for StubDrawContext {
    fn finish(self) -> Option<()> {
        stub!()
    }
}

impl crate::platform::Platform for StubPlatform {
    type State = SimpleEventLoopState;
    type Window = StubWindow;
    type Renderer = StubRenderPlatform;

    fn new() -> Self {
        stub!()
    }

    fn create_window(
        &mut self,
        _settings: crate::window::WindowBuilder,
    ) -> Result<Self::Window, String> {
        stub!()
    }

    fn run<F>(self, _event_handler: F) -> !
    where
        F: 'static + FnMut(&mut Self::State, Event),
    {
        stub!()
    }
}

#[cfg(feature = "platform_opengl")]
impl crate::platform::Platform for StubOpenglPlatform {
    type State = SimpleEventLoopState;
    type Window = StubWindow;
    type Renderer = super::opengl::OpenGlRenderPlatform;

    fn new() -> Self {
        stub!()
    }

    fn create_window(
        &mut self,
        _settings: crate::window::WindowBuilder,
    ) -> Result<Self::Window, String> {
        stub!()
    }

    fn run<F>(self, _event_handler: F) -> !
    where
        F: 'static + FnMut(&mut Self::State, Event),
    {
        stub!()
    }
}

impl crate::window::WindowSettings for StubWindow {
    fn size(&self) -> (f32, f32) {
        stub!()
    }
    fn set_size(&mut self, _size: (f32, f32)) {
        stub!()
    }
    fn title(&self) -> &str {
        stub!()
    }
    fn set_title(&mut self, _title: String) {
        stub!()
    }
    fn fullscreen(&self) -> bool {
        stub!()
    }
    fn set_fullscreen(&mut self, _fullscreen: bool) {
        stub!()
    }
    fn background_color(&self) -> crate::graphics::Color {
        stub!()
    }
    fn set_background_color(&mut self, _color: crate::graphics::Color) {
        stub!()
    }
}

impl crate::window::Window<StubRenderPlatform> for StubWindow {
    fn pixels_per_dp(&self) -> f32 {
        stub!()
    }

    fn normalize_pointer_event(
        &self,
        _event: &mut crate::pointer::PointerEventData,
    ) {
        stub!()
    }

    fn recalculate_viewport(&mut self) {
        stub!()
    }

    fn flip(&mut self) {
        stub!()
    }

    fn prepare_draw(&mut self, _first_pass: Option<()>) -> StubDrawContext {
        stub!()
    }

    fn take_screenshot(&self) -> Box<[u8]> {
        stub!()
    }
}

#[cfg(feature = "platform_opengl")]
impl crate::window::Window<super::opengl::OpenGlRenderPlatform>
    for StubWindow
{
    fn pixels_per_dp(&self) -> f32 {
        stub!()
    }

    fn normalize_pointer_event(
        &self,
        _event: &mut crate::pointer::PointerEventData,
    ) {
        stub!()
    }

    fn recalculate_viewport(&mut self) {
        stub!()
    }

    fn flip(&mut self) {
        stub!()
    }

    fn prepare_draw(
        &mut self,
        _first_pass: Option<()>,
    ) -> crate::graphics::DrawContext<super::opengl::OpenGlRenderPlatform>
    {
        stub!()
    }

    fn take_screenshot(&self) -> Box<[u8]> {
        stub!()
    }
}

#[derive(Default)]
pub struct StubTexture;

impl crate::platform::graphics::Texture for StubTexture {
    fn load_static(
        _width: u16,
        _height: u16,
        _alignment: u16,
        _pixels: &'static [u8],
    ) -> Self {
        stub!()
    }
}

#[derive(Default)]
pub struct StubSlicedImage;

impl crate::platform::graphics::SlicedImage<StubTexture> for StubSlicedImage {
    fn set_image<P>(&mut self, _texture: StubTexture, _padding: P)
    where
        P: crate::dims::Padding2d,
    {
        stub!()
    }
}

impl crate::graphics::Graphic<StubRenderPlatform> for StubSlicedImage {
    fn draw(
        &mut self,
        _ctx: &mut crate::graphics::DrawContext<StubRenderPlatform>,
    ) {
        stub!()
    }
}

impl crate::dims::Rect for StubSlicedImage {
    fn x(&self) -> crate::dims::Dim {
        stub!()
    }
    fn y(&self) -> crate::dims::Dim {
        stub!()
    }
    fn x_mut<F, R>(&mut self, _f: F) -> R
    where
        F: FnOnce(&mut crate::dims::Dim) -> R,
    {
        stub!()
    }
    fn y_mut<F, R>(&mut self, _f: F) -> R
    where
        F: FnOnce(&mut crate::dims::Dim) -> R,
    {
        stub!()
    }
}

#[derive(Default)]
pub struct StubSelectableSlicedImage;

impl crate::selectable::Selectable for StubSelectableSlicedImage {
    fn selection_changed(
        &mut self,
        _state: crate::selectable::SelectionState,
    ) {
        stub!()
    }
}

impl crate::platform::graphics::SelectableSlicedImage<StubTexture>
    for StubSelectableSlicedImage
{
    fn set_image<P>(
        &mut self,
        _texture: StubTexture,
        _padding: P,
        _states: &'static [crate::selectable::SelectionState],
    ) where
        P: crate::dims::Padding2d,
    {
        stub!()
    }
}

impl crate::graphics::Graphic<StubRenderPlatform>
    for StubSelectableSlicedImage
{
    fn draw(
        &mut self,
        _ctx: &mut crate::graphics::DrawContext<StubRenderPlatform>,
    ) {
        stub!()
    }
}

impl crate::dims::Rect for StubSelectableSlicedImage {
    fn x(&self) -> crate::dims::Dim {
        stub!()
    }
    fn y(&self) -> crate::dims::Dim {
        stub!()
    }
    fn x_mut<F, R>(&mut self, _f: F) -> R
    where
        F: FnOnce(&mut crate::dims::Dim) -> R,
    {
        stub!()
    }
    fn y_mut<F, R>(&mut self, _f: F) -> R
    where
        F: FnOnce(&mut crate::dims::Dim) -> R,
    {
        stub!()
    }
}

#[derive(Default)]
pub struct StubText;

impl crate::platform::graphics::Text for StubText {
    fn set_text<'a, T>(
        &mut self,
        _text: T,
        _pos: &crate::text::TextPosition,
        _settings: &crate::text::TextSettings,
    ) where
        T: 'a + Iterator<Item = crate::text::RichTextCommand<'a>>,
    {
        stub!()
    }
}

impl crate::graphics::Graphic<StubRenderPlatform> for StubText {
    fn draw(
        &mut self,
        _ctx: &mut crate::graphics::DrawContext<StubRenderPlatform>,
    ) {
        stub!()
    }
}

#[derive(Default)]
pub struct StubTextEdit;

impl crate::platform::graphics::TextEdit for StubTextEdit {
    fn set_text_plain(
        &mut self,
        _text: &str,
        _pos: &crate::text::TextPosition,
        _settings: &crate::text::TextSettings,
    ) {
        stub!()
    }
    fn char_at(&self, _x: f32, _y: f32) -> Option<usize> {
        stub!()
    }
    fn char_rect(&self, _index: usize) -> Option<crate::dims::SimpleRect> {
        stub!()
    }
}

impl crate::graphics::Graphic<StubRenderPlatform> for StubTextEdit {
    fn draw(
        &mut self,
        _ctx: &mut crate::graphics::DrawContext<StubRenderPlatform>,
    ) {
        stub!()
    }
}
