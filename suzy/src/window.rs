/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! Types associated with the creation and control of windows.

use crate::graphics::Color;
use crate::graphics::DrawContext;
use crate::platform::RenderPlatform;
use crate::pointer::PointerEventData;

/// An event that happened on a window.
pub enum WindowEvent {
    /// The window size changed.
    Resize,

    /// The scale of the window changed.
    DpScaleChange,

    /// A key was pressed.
    KeyDown(i32),

    /// The window close button was clicked.
    Quit,

    /// A pointer event happened on the window.
    Pointer(PointerEventData),
}

/// A trait which represents the settings a window might have.
pub trait WindowSettings {
    /// Get the size of the window in dp
    fn size(&self) -> (f32, f32);
    
    /// Set the size of the window in dp
    fn set_size(&mut self, size: (f32, f32));

    /// Get the window title
    fn title(&self) -> &str;

    /// Set the window title
    fn set_title(&mut self, title: String);

    /// Get the window fullscreen state
    fn fullscreen(&self) -> bool;

    /// Set the fullscreen state
    fn set_fullscreen(&mut self, fullscreen: bool);

    /// Get the window background color
    fn background_color(&self) -> Color;

    /// Set the window background color
    fn set_background_color(&mut self, color: Color);
}

/// A structure which defines the initial creation parameters for a window
pub struct WindowBuilder {
    size: (f32, f32),
    title: String,
    fullscreen: bool,
    background_color: Color,
}

impl WindowBuilder {
    /// Consumes the window builder, returning just the title string.
    pub fn into_title(self) -> String { self.title }
}

impl Default for WindowBuilder {
    fn default() -> Self {
        Self {
            size: (1000.0, 500.0),
            title: "Suzy Window".to_string(),
            fullscreen: false,
            background_color: Color::create_rgba(0.176, 0.176, 0.176, 1.0),
        }
    }
}

impl WindowSettings for WindowBuilder {
    fn size(&self) -> (f32, f32) { self.size }
    
    fn set_size(&mut self, size: (f32, f32)) {
        self.size = size;
    }

    fn title(&self) -> &str { &self.title }

    fn set_title(&mut self, title: String) {
        self.title = title;
    }

    fn fullscreen(&self) -> bool { self.fullscreen }

    fn set_fullscreen(&mut self, fullscreen: bool) {
        self.fullscreen = fullscreen;
    }

    fn background_color(&self) -> Color { self.background_color }

    fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }
}

/// A trait which represents a window.
pub trait Window<P: RenderPlatform> : WindowSettings {
    /// Get the pixel density of the window as displayed
    fn pixels_per_dp(&self) -> f32;

    /// Take a pointer event generated by the event system, and ensure the
    /// values are properly in dp according to this window's scale.
    fn normalize_pointer_event(&self, event: &mut PointerEventData);

    /// Called in response to an event indicating a change in size.
    fn recalculate_viewport(&mut self);

    /// Do some sort of synchonization - this function is expected to block
    /// for some period of time. In a double buffered context, this will
    /// usually cause the back buffer to be displayed.
    fn flip(&mut self);

    /// Prepare to draw to this window, create a DrawContext.
    fn prepare_draw(&mut self, first_pass: bool) -> DrawContext<P>;

    /// Take a screenshot of the contents of window.
    fn take_screenshot(&self) -> Box<[u8]>;
}
