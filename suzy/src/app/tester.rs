/* SPDX-License-Identifier: (Apache-2.0 OR MIT OR Zlib) */
/* Copyright © 2021 Violet Leonard */

use std::time;

use crate::{platform::Platform, pointer::PointerEventData};

use super::App;

/// An interface to enable some automated testing of an app.
///
/// Retrieve with [`App::test`](struct.App.html#method.test)
pub struct AppTesterInterface<'a, P: Platform> {
    start_time: time::Instant,
    app: &'a mut App<P>,
    needs_draw: bool,
}

impl<'a, P: Platform> AppTesterInterface<'a, P> {
    /// Create a tester interface from a CurrentApp.
    pub fn new(app: &'a mut App<P>) -> Self {
        let start_time = std::time::Instant::now();
        app.start_frame(start_time);
        Self {
            app,
            start_time,
            needs_draw: true,
        }
    }
}

impl<P: Platform> AppTesterInterface<'_, P> {
    /// Issue an update to ensure all events have fully resolved
    pub fn draw_if_needed(&mut self) {
        if self.needs_draw {
            self.app.update_watches();
            self.app.draw();
            self.needs_draw = false;
        }
    }

    /// Start the next frame with a default frame time.
    pub fn next_frame_60fps(&mut self) {
        self.next_frame(time::Duration::from_nanos(16666667));
    }

    /// Update and draw the current frame, then start a new one, acting as
    /// though `frame_time` has passed (e.g. for the purposes of App::time).
    pub fn next_frame(&mut self, frame_time: time::Duration) {
        self.draw_if_needed();
        self.app.finish_draw();
        self.start_time += frame_time;
        self.app.start_frame(self.start_time);
        self.needs_draw = true;
    }

    /// Simulate a pointer event.
    ///
    /// If the passed in event is already in the suzy coordinate system,
    /// remember to set `normalized` to true.
    pub fn pointer(&mut self, pointer: PointerEventData) {
        self.app.pointer_event(pointer);
        self.needs_draw = true;
    }

    /// Take a screenshot.
    ///
    /// Data returned by this function may be dependent on the suzy platform
    /// in use.
    pub fn take_screenshot(&mut self) -> Box<[u8]> {
        self.draw_if_needed();
        self.app.take_screenshot()
    }

    /// Short-hand to simulate a mouse click
    ///
    /// This is equivalent to:
    /// 1) sending a mouse-down pointer event
    /// 2) advancing the frame with the default frame time
    /// 3) sending a mouse-up pointer event
    pub fn mouse_click(&mut self, pos: [f32; 2]) {
        let [px, py] = pos;
        self.pointer(PointerEventData {
            id: crate::pointer::PointerId::Mouse,
            action: crate::pointer::PointerAction::Down,
            x: px,
            y: py,
        });
        self.next_frame_60fps();
        self.pointer(PointerEventData {
            id: crate::pointer::PointerId::Mouse,
            action: crate::pointer::PointerAction::Up,
            x: px,
            y: py,
        });
    }
}
