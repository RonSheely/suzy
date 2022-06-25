/* SPDX-License-Identifier: (Apache-2.0 OR MIT OR Zlib) */
/* Copyright © 2021 Violet Leonard */

//! Suzy's watch system provides the main way to define functionality within
//! the framework.  It enables you to describe the relationships between
//! widgets in a declaritive way.
//!
//! The watch system is based off an "automatic" observer pattern, inspired
//! by Kivy's "[Kv Language](https://kivy.org/doc/stable/guide/lang.html)".
//!
//! Inside a "watch" closure Suzy tracks which values are accessed at
//! runtime and automatically binds to them.  The closure is re-run when
//! the bound values change.
//!
//! The watch system is defined in terms of relationships between two API
//! surfaces: [`Watched`](struct.Watched.html) represents some data which will
//! be interesting to observe, and
//! [`WidgetInit::watch`](../widget/trait.WidgetInit.html#tymethod.watch)
//! is used to submit a closure which observes values.
//!
//! A [`WatchedEvent`](stuct.WatchedEvent.html) is similar to a Watched value.
//! Instead of representing a "current state" however, it provides a system
//! where each watch closure will be run exactly once with each value provided
//! to [`dispatch`](struct.WatchedEvent.html#method.dispatch).
//!
//! Other utilities for less common situations are provided in this module.
//!
//! # Examples
//! Place a fixed sized button at the bottom-left of a custom widget, with
//! a small amount of padding.  Because the layout instructions are included
//! in a closure submitted to `watch`, whenever the position of the Widget
//! `MyWidgetData` changes, the closure will be re-run and the position of the
//! button will be updated to match.
//!
//! ```rust
//! # use suzy::widget::{self, *};
//! # use suzy::dims::Rect;
//! # use suzy::selectable::SelectableIgnored;
//! # type ButtonContent = SelectableIgnored<()>;
//! use suzy::widgets::Button;
//!
//! struct MyWidgetData {
//!     button: Button<ButtonContent>,
//! }
//!
//! impl widget::Content for MyWidgetData {
//!     fn desc(mut desc: impl widget::Desc<Self>) {
//!         desc.watch(|this, rect| {
//!             this.button.set_width(200.0);
//!             this.button.set_height(100.0);
//!             this.button.set_left(rect.left() + 50.0);
//!             this.button.set_bottom(rect.bottom() + 50.0);
//!         });
//!         desc.child(|this| &mut this.button);
//!     }
//! }

pub use drying_paint::{
    watched_channel, AtomicWatchedMeta, AtomicWatchedMetaTrigger, Watched,
    WatchedCell, WatchedEvent, WatchedMeta, WatchedReceiver, WatchedSender,
    Watcher, WatcherId, WatcherMeta,
};

#[cfg(feature = "inline_tweak")]
#[cfg(debug_assertions)]
#[doc(hidden)]
pub use inline_tweak::inline_tweak;
