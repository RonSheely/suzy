/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::window::WindowEvent;

/// Possible events a platform could generate.
pub enum Event<'a> {
    /// Window event, such as resize
    WindowEvent(WindowEvent),

    /// Emitted when a new frame is being started.
    StartFrame(std::time::Instant),

    /// Update all pending watch closures.
    Update,

    /// Render all widgets.
    Draw,

    /// After rendering, finalize displaying the results to the user.
    FinishDraw,

    /// Take a screenshot.
    ///
    /// Sent by testing platform in some circumstances.
    TakeScreenshot(&'a mut Box<[u8]>),
}

/// A trait which the event handler can use to shutdown the event loop.
pub trait EventLoopState {
    /// Signal that the event loop should stop.
    fn request_shutdown(&mut self);
}

/// A type which implements EventLoopState with a single boolean flag.
#[derive(Clone, Copy, Debug)]
pub struct SimpleEventLoopState {
    /// A flag indicating if the event loop should keep running.
    pub running: bool,
}

impl Default for SimpleEventLoopState {
    fn default() -> Self {
        Self { running: true }
    }
}

impl EventLoopState for SimpleEventLoopState {
    fn request_shutdown(&mut self) {
        self.running = false;
    }
}