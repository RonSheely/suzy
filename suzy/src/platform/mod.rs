/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! This module offers a flexible interface for the low-level aspects of
//! the GUI system.
//!
//! One render platform is currently included, based on OpenGL ES 2.0
//!
//! Two windowing platforms are currently included; the primary is based on
//! SDL2.  A windowless OSMesa interface is also included, intended for
//! automated testing.
//!
//! If no platforms are enabled (using features) the default platform is
//! a "stub" platform which has no functionality (all methods panic).

mod shared;
pub use shared::{
    Platform,
    RenderPlatform,
    Event,
    EventLoopState,
    SimpleEventLoopState,
};

// Platforms

// stub types are used in case no other defaults are available - all their
// methods panic
mod stub;

#[cfg(feature = "platform_opengl")]
pub mod opengl;

#[cfg(feature = "sdl")]
pub mod sdl2;

#[cfg(feature = "platform_osmesa")]
pub mod osmesa;

// Default Platform

#[cfg(not(feature = "platform_opengl"))]
pub use self::stub::StubPlatform as DefaultPlatform;
#[cfg(all(feature = "platform_opengl", not(feature = "sdl")))]
pub use self::stub::StubOpenglPlatform as DefaultPlatform;
#[cfg(feature = "sdl")]
pub use self::sdl2::SDLPlatform as DefaultPlatform;

// Platform used for tests

#[cfg(not(feature = "platform_opengl"))]
pub use self::stub::StubPlatform as TestPlatform;
#[cfg(all(
    feature = "platform_opengl",
    not(feature = "sdl"),
    not(feature = "platform_osmesa"),
))]
pub use self::stub::StubOpenglPlatform as TestPlatform;
#[cfg(all(feature = "sdl", not(feature = "platform_osmesa")))]
pub use self::sdl2::SDLPlatform as TestPlatform;
#[cfg(feature = "platform_osmesa")]
pub use self::osmesa::OSMesaPlatform as TestPlatform;

pub type DefaultRenderPlatform = <DefaultPlatform as Platform>::Renderer;
pub type TestRenderPlatform = <TestPlatform as Platform>::Renderer;

