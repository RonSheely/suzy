use std::cell::{Ref, RefMut};

use drying_paint::{Watcher, WatcherId};
pub use drying_paint::Watched;

use crate::dims::{Rect, Dim};
use crate::graphics::DrawContext;
use crate::platform::{DefaultRenderPlatform, RenderPlatform};
use crate::pointer::PointerEvent;

mod anon;
mod content;
mod graphic;
mod init;
mod internal;
mod newwidget;
mod receivers;
mod rect;

use anon::AnonWidget;
use internal::WidgetInternal;
use rect::WidgetRect;
use receivers::{
    DrawChildReceiver,
    PointerEventChildReceiver,
    DrawGraphicBeforeReceiver,
    DrawGraphicAfterReceiver,
    FindWidgetReceiver,
};

pub use anon::{
    OwnedWidgetProxy,
    WidgetProxy,
    WidgetProxyMut,
};
pub use content::WidgetContent;
pub use graphic::WidgetGraphic;
pub use init::WidgetInit;
pub use internal::WidgetExtra;
pub use newwidget::NewWidget;
pub use receivers::{
    WidgetChildReceiver,
    WidgetMutChildReceiver,
    WidgetGraphicReceiver,
};


/// A basic structure to wrap some data and turn it into a widget.
#[derive(Default)]
pub struct Widget<T, P = DefaultRenderPlatform>
where
    P: RenderPlatform,
    T: WidgetContent<P> + ?Sized,
{
    watcher: Watcher<WidgetInternal<P, T>>,
}

impl<P, T> Widget<T, P>
where
    P: RenderPlatform,
    T: WidgetContent<P>,
{
    pub fn id(&self) -> WidgetId {
        WidgetId {
            id: self.watcher.id(),
        }
    }

    /// Get an anonymous reference to this widget. This is required by
    /// WidgetContent::children(), for example.
    pub fn proxy(&self) -> WidgetProxy<P> {
        WidgetProxy { anon: self }
    }

    /// Get an mutable anonymous reference to this widget. This is required
    /// by WidgetContent::children_mut(), for example.
    pub fn proxy_mut(&mut self) -> WidgetProxyMut<P> {
        WidgetProxyMut { anon: self }
    }

    fn internal(&self) -> Ref<WidgetInternal<P, T>> { self.watcher.data() }
    fn internal_mut(&mut self) -> RefMut<WidgetInternal<P, T>> {
        self.watcher.data_mut()
    }

    pub fn content(&self) -> Ref<T> {
        Ref::map(self.internal(), |w| &w.content)
    }

    pub fn content_mut(&mut self) -> RefMut<T> {
        RefMut::map(self.internal_mut(), |w| &mut w.content)
    }

    pub(crate) fn draw(&mut self, ctx: &mut DrawContext<P>) {
        let mut wid_int = self.internal_mut();
        let content = &mut wid_int.content;
        content.graphics(DrawGraphicBeforeReceiver { ctx });
        content.children_mut(DrawChildReceiver { ctx });
        content.graphics(DrawGraphicAfterReceiver { ctx });
    }

    pub(crate) fn find_widget<F>(&mut self, id: WidgetId, func: F)
    where
        F: FnOnce(&mut dyn AnonWidget<P>)
    {
        self.find_widget_internal(&id, &mut Some(func));
    }

    fn find_widget_internal(
        &mut self,
        id: &WidgetId,
        func: &mut Option<impl FnOnce(&mut dyn AnonWidget<P>)>,
    ) {
        if let Some(f) = func.take() {
            if self.id() == *id {
                f(self);
            } else {
                *func = Some(f);
                let content = &mut self.internal_mut().content;
                content.children_mut(FindWidgetReceiver { id, func });
            }
        }
    }

    pub(crate) fn pointer_event(&mut self, event: &mut PointerEvent) -> bool {
        let mut handled_by_child = false;
        {
            let content = &mut self.internal_mut().content;
            content.children_mut(PointerEventChildReceiver {
                event,
                handled: &mut handled_by_child,
            });
        }
        handled_by_child || self.pointer_event_self(event)
    }

    pub(crate) fn pointer_event_self(&mut self, event: &mut PointerEvent)
        -> bool
    {
        let id = self.id();
        let mut borrow = self.internal_mut();
        let wid_int: &mut WidgetInternal<P, T> = &mut borrow;
        let mut extra = WidgetExtra {
            id,
            rect: &mut wid_int.rect,
        };
        T::pointer_event(&mut wid_int.content, &mut extra, event)
    }

}

impl<P, T> Widget<T, P>
where
    P: RenderPlatform,
    T: WidgetContent<P> + Default,
{
    pub fn default_with_rect<R: Rect>(rect: &R) -> Self {
        Widget {
            watcher: Watcher::create(WidgetInternal {
                rect: WidgetRect::external_from(rect),
                content: Default::default(),
                _platform: Default::default(),
            }),
        }
    }
}

impl<P, T> Rect for Widget<T, P>
where
    P: RenderPlatform,
    T: WidgetContent<P>,
{
    fn x(&self) -> Dim { self.internal().rect.x() }
    fn y(&self) -> Dim { self.internal().rect.y() }

    fn x_mut<F, R>(&mut self, f: F) -> R
        where F: FnOnce(&mut Dim) -> R
    {
        self.internal_mut().rect.external_x_mut(f)
    }

    fn y_mut<F, R>(&mut self, f: F) -> R
        where F: FnOnce(&mut Dim) -> R
    {
        self.internal_mut().rect.external_y_mut(f)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WidgetId {
    id: WatcherId,
}

impl<P, T> From<&Widget<T, P>> for WidgetId
where
    P: RenderPlatform,
    T: WidgetContent<P>,
{
    fn from(widget: &Widget<T, P>) -> Self {
        widget.id()
    }
}

impl From<&mut WidgetExtra<'_>> for WidgetId {
    fn from(extra: &mut WidgetExtra) -> Self {
        extra.id()
    }
}
