/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::math::Rect;
use crate::platform::{DefaultPlatform, DefaultRenderPlatform, RenderPlatform};
use crate::pointer::PointerEvent;

use super::{
    WidgetChildReceiver,
    WidgetGraphicReceiver,
    WidgetInit,
    WidgetExtra,
};

/// This trait provides the "glue" between the data you define in custom
/// widgets and the behavior suzy defines for widgets.  There are three
/// required methods: `init`, `children`, and `graphics`.
///
/// The `init` method is the primary point for registering the `watch`
/// functions that define the behavior of a widget. See the
/// [watch](../watch/index.html) module for more information.
///
/// The methods `children` and `graphics` should be fairly straightforward
/// to implement: they provide a simple "internal iterator" format which
/// allows suzy to iterate over the children and graphics a custom widget
/// contains.
///
/// Custom widgets may contain any number of graphics and child widgets, or
/// none of either.
///
/// For example, if a custom widget contains two buttons as children:
///
/// ```rust
/// # use suzy::widget::*;
/// # use suzy::selectable::SelectableIgnored;
/// # type ButtonContent = SelectableIgnored<()>;
/// use suzy::widgets::Button;
///
/// struct MyWidgetData {
///     button_one: Button<ButtonContent>,
///     button_two: Button<ButtonContent>,
/// }
///
/// impl WidgetContent for MyWidgetData {
///     // ...
/// #   fn init<I: WidgetInit<Self>>(_init: I) {}
/// #   fn graphics<R: WidgetGraphicReceiver>(&mut self, _receiver: R) {}
///
///     fn children<R: WidgetChildReceiver>(&mut self, mut receiver: R) {
///         receiver.child(&mut self.button_one);
///         receiver.child(&mut self.button_two);
///     }
/// }
/// ```
///
/// Or, if the custom widget only has a single graphic:
///
/// ```rust
/// # use suzy::widget::*;
/// # type MyGraphic = ();
///
/// struct MyWidgetData {
///     graphic: MyGraphic,
/// }
///
/// impl WidgetContent for MyWidgetData {
///     // ...
/// #   fn init<I: WidgetInit<Self>>(_init: I) {}
/// #   fn children<R: WidgetChildReceiver>(&mut self, _receiver: R) {}
///
///     fn graphics<R: WidgetGraphicReceiver>(&mut self, mut receiver: R) {
///         receiver.graphic(&mut self.graphic);
///     }
/// }
/// ```
///
pub trait WidgetContent<P = DefaultRenderPlatform>
where
    P: RenderPlatform,
    Self: 'static,
{
    /// This method provides a convient place to register functions which
    /// watch values and update parts of the widget when they change.
    fn init<I: WidgetInit<Self, P>>(init: I);

    /// Use this method to specify the children a custom widget contains.
    ///
    /// Call `receiver.child` for each child.
    fn children<R: WidgetChildReceiver<P>>(&mut self, receiver: R);

    /// Use this method to specify the graphics a custom widget contains.
    ///
    /// Call `receiver.graphic` for each graphic.
    fn graphics<R: WidgetGraphicReceiver<P>>(&mut self, receiver: R);

    /// Override this method to define a custom shape for the widget.
    ///
    /// This is used by e.g. Button to test if it should handle a pointer
    /// event.  The default is a standard rectangular test.
    fn hittest(&self, extra: &mut WidgetExtra<'_>, point: (f32, f32)) -> bool {
        extra.contains(point)
    }

    /// Override this method to handle pointer events directly by a custom
    /// widget.
    ///
    /// Return true if this successfully handled the event.
    fn pointer_event(
        &mut self,
        extra: &mut WidgetExtra<'_>,
        event: &mut PointerEvent,
    ) -> bool {
        let _unused = (extra, event);
        false
    }

    /// This is a convience function to create and run an App with this
    /// content as the only initial root widget.
    fn run_as_app() -> !
    where
        Self: Default + WidgetContent<DefaultRenderPlatform>,
    {
        run_widget_as_app::<Self>()
    }
}

impl<P: RenderPlatform> WidgetContent<P> for () {
    fn init<I: WidgetInit<Self, P>>(_init: I) {}
    fn children<R: WidgetChildReceiver<P>>(&mut self, _receiver: R) {}
    fn graphics<R: WidgetGraphicReceiver<P>>(&mut self, _receiver: R) {}
}

fn run_widget_as_app<T>() -> !
where
    T: Default + WidgetContent<DefaultRenderPlatform>,
{
    use crate::app::{App, AppBuilder};
    use crate::window::WindowSettings;

    let name = std::any::type_name::<T>().rsplit("::").next().unwrap();
    let (_, title) = name.chars().fold(
        (false, String::new()),
        |(prev, mut title), ch| {
            if prev && ch.is_uppercase() {
                title.push(' ');
            }
            title.push(ch);
            (ch.is_lowercase(), title)
        }
    );
    let mut builder = AppBuilder::default();
    builder.set_title(title);
    let app: App<DefaultPlatform> = builder.build();
    let (app, _) = app.with(|current| {
        current.add_root(super::Widget::<T>::default);
    });
    app.run();
}
