use std::convert::TryFrom;

use crate::window;
use crate::window::{WindowEvent, WindowSettings, WindowBuilder};
use crate::graphics::DrawContext;

pub struct Window {}

macro_rules! stub {
    () => { unimplemented!("dummy platform is entirely unimplemented") };
}

impl TryFrom<WindowBuilder> for Window {
    type Error = String;

    fn try_from(_builder: WindowBuilder) -> Result<Self, Self::Error> {
        stub!();
    }
}

impl WindowSettings for Window {
    fn size(&self) -> (f32, f32) { stub!() }
    fn set_size(&mut self, _size: (f32, f32)) { stub!() }
    fn title(&self) -> &str { stub!() }
    fn set_title(&mut self, _title: String) { stub!() }
    fn fullscreen(&self) -> bool { stub!() }
    fn set_fullscreen(&mut self, _fullscreen: bool) { stub!() }
}

impl<'a> window::Window<'a> for Window {
    fn pixels_per_dp(&self) -> f32 { stub!() }
    fn clear(&mut self) { stub!() }
    fn flip(&mut self) { stub!() }

    type Events = Events<'a>;
    fn events(&'a mut self) -> Self::Events { stub!() }

    fn prepare_draw(&mut self) -> DrawContext { stub!() }
}

pub struct Events<'a> {
    _window: &'a mut Window,
}

impl Iterator for Events<'_> {
    type Item = WindowEvent;
    fn next(&mut self) -> Option<WindowEvent> { stub!() }
}
