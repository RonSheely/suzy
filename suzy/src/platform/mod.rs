/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! A flexible interface for the low-level aspects of the GUI system.
//!
//! One render platform is currently included, based on OpenGL ES 2.0
//!
//! Two windowing platforms are currently included; the primary is based on
//! SDL2.  A windowless OSMesa interface is also included, intended for
//! automated testing.
//!
//! If no platforms are enabled (using features) the default platform is
//! a "stub" platform which has no functionality (all methods panic).

mod event;
pub use event::{Event, EventLoopState, SimpleEventLoopState};

pub mod graphics;

pub use crate::platforms::{
    DefaultPlatform, DefaultRenderPlatform, TestPlatform, TestRenderPlatform,
};

/// A platform handles window creation and manages an event loop.
pub trait Platform: 'static {
    /// The event loop state tracked by this platform.
    type State: EventLoopState;

    /// The RenderPlatform this platform supports.
    type Renderer: RenderPlatform;

    /// The type of window this platform creates.
    type Window: crate::window::Window<Self::Renderer>;

    /// Initialize the platform.
    fn new() -> Self;

    /// Create a window.
    fn create_window(
        &mut self,
        settings: crate::window::WindowBuilder,
    ) -> Result<Self::Window, String>;

    /// Run, the event loop, calling the provided closure with each new event.
    fn run<F>(self, event_handler: F) -> !
    where
        F: 'static + FnMut(&mut Self::State, Event);
}

/// A RenderPlatform provides tools to create Graphics.
pub trait RenderPlatform: 'static {
    /// The shared context passed along to draw calls.
    type Context: 'static;

    /// The parameters passed to draw calls.
    type DrawParams: crate::graphics::DrawParams<Self::Context>;

    /// A default type of WidgetContent this RenderPlatform provides for
    /// buttons.
    type DefaultButtonContent: Default
        + crate::selectable::Selectable
        + crate::widget::WidgetContent<Self>
        + crate::widgets::TextContent;

    /// The platform's texture primitive.
    type Texture: graphics::Texture + Default;

    /// The platform's graphic primitive for 9-sliced images.
    type SlicedImage: graphics::SlicedImage<Self::Texture> + Default + crate::graphics::Graphic<Self>;

    /// The platform's graphic primitive for selectable 9-sliced images.
    type SelectableSlicedImage: graphics::SelectableSlicedImage<Self::Texture> + Default + crate::graphics::Graphic<Self>;
}
