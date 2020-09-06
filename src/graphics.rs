/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::platform::{
    DefaultRenderPlatform,
    RenderPlatform,
};

pub trait Graphic<P: RenderPlatform = DefaultRenderPlatform> {
    fn draw(&mut self, ctx: &mut DrawContext<P>);
}

impl<P: RenderPlatform> Graphic<P> for () {
    fn draw(&mut self, _ctx: &mut DrawContext<P>) {}
}

impl<P: RenderPlatform> Graphic<P> for [(); 0] {
    fn draw(&mut self, _ctx: &mut DrawContext<P>) {}
}

impl<P: RenderPlatform, T: Graphic<P>> Graphic<P> for [T] {
    fn draw(&mut self, ctx: &mut DrawContext<P>) {
        for graphic in self {
            graphic.draw(ctx);
        }
    }
}

pub trait DrawParams<Ctx> {
    fn apply_all(&mut self, ctx: &mut Ctx);

    fn apply_change(current: &Self, new: &mut Self, ctx: &mut Ctx);
}

#[derive(Clone, Copy, Debug)]
enum LastApplied<T> {
    History(usize),
    Current,
    None,
    Removed(T),
}

impl<T> Default for LastApplied<T> {
    fn default() -> Self { Self::None }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawPass {
    DrawAll,
    UpdateContext,
    DrawRemaining,
}

#[derive(Debug)]
pub struct DrawContext<'a, P: RenderPlatform = DefaultRenderPlatform> {
    context: &'a mut P::Context,
    current: P::DrawParams,
    history: Vec<P::DrawParams>,
    last_applied: LastApplied<P::DrawParams>,
    pass: DrawPass,
}

impl<'a, P: RenderPlatform> DrawContext<'a, P> {
    pub fn new(
        ctx: &'a mut P::Context,
        starting: P::DrawParams,
        first_pass: bool,
    ) -> Self {
        let pass = if first_pass {
            DrawPass::DrawAll
        } else {
            DrawPass::DrawRemaining
        };
        Self {
            context: ctx,
            current: starting,
            history: Vec::new(),
            last_applied: LastApplied::None,
            pass,
        }
    }

    pub fn pass(ctx: &Self) -> DrawPass { ctx.pass }

    pub fn render_ctx(ctx: &Self) -> &P::Context { &ctx.context }

    pub fn render_ctx_mut(ctx: &mut Self) -> &mut P::Context { &mut ctx.context }

    pub fn graphic_not_ready(ctx: &mut Self) {
        ctx.pass = DrawPass::UpdateContext;
    }

    pub fn prepare_draw(ctx: &mut Self) {
        assert_ne!(
            ctx.pass, 
            DrawPass::UpdateContext,
            "prepare_draw called during an UpdateContext pass",
        );
        match std::mem::replace(&mut ctx.last_applied, LastApplied::Current) {
            LastApplied::Current => (),
            LastApplied::None => ctx.current.apply_all(ctx.context),
            LastApplied::Removed(old) => {
                DrawParams::apply_change(&old, &mut ctx.current, ctx.context);
            },
            LastApplied::History(index) => {
                DrawParams::apply_change(
                    &ctx.history[index],
                    &mut ctx.current,
                    ctx.context,
                );
            }
        }
    }

    pub fn force_redraw<F, R>(ctx: &mut Self, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R
    {
        let restore = if ctx.pass == DrawPass::DrawRemaining {
            ctx.pass = DrawPass::DrawAll;
            true
        } else {
            false
        };
        let ret = f(ctx);
        if restore && ctx.pass != DrawPass::UpdateContext {
            ctx.pass = DrawPass::DrawRemaining;
        }
        ret
    }

    pub(crate) fn draw<'b, I>(ctx: &mut Self, roots: I) -> bool
    where
        I: 'b + Iterator<Item = &'b mut crate::widget::OwnedWidgetProxy<P>>,
    {
        if ctx.pass == DrawPass::UpdateContext {
            ctx.pass = DrawPass::DrawRemaining;
        }
        for widget in roots {
            widget.draw(ctx);
        }
        ctx.pass == DrawPass::UpdateContext
    }
}

impl<'a, P> DrawContext<'a, P>
where
    P: RenderPlatform,
    P::DrawParams: Clone
{
    pub fn push<R, F: FnOnce(&mut Self) -> R>(ctx: &mut Self, func: F) -> R {
        Self::manually_push(ctx);
        let ret = func(ctx);
        Self::manually_pop(ctx);
        ret
    }

    pub fn manually_push(ctx: &mut Self) {
        let new = ctx.current.clone();
        let old = std::mem::replace(&mut ctx.current, new);
        ctx.history.push(old);
        if let LastApplied::Current = ctx.last_applied {
            ctx.last_applied = LastApplied::History(ctx.history.len() - 1);
        }
    }

    pub fn manually_pop(ctx: &mut Self) {
        let new = ctx.history.pop().expect(
            "DrawContext::pop called more times than push!"
        );
        let old = std::mem::replace(&mut ctx.current, new);
        match &ctx.last_applied {
            LastApplied::History(index) => {
                use std::cmp::Ordering;
                ctx.last_applied = match index.cmp(&ctx.history.len()) {
                    Ordering::Less => LastApplied::History(*index),
                    Ordering::Equal => LastApplied::Current,
                    Ordering::Greater => {
                        debug_assert!(false, "DrawContext corrupted");
                        LastApplied::None
                    },
                };
            },
            LastApplied::Current => {
                ctx.last_applied = LastApplied::Removed(old);
            }
            _ => (),
        };
    }
}

impl<P: RenderPlatform> std::ops::Deref for DrawContext<'_, P> {
    type Target = P::DrawParams;
    fn deref(&self) -> &P::DrawParams { &self.current }
}

impl<P: RenderPlatform> std::ops::DerefMut for DrawContext<'_, P> {
    fn deref_mut(&mut self) -> &mut P::DrawParams { &mut self.current }
}

impl<P: RenderPlatform> Drop for DrawContext<'_, P> {
    fn drop(&mut self) {
        if self.pass != DrawPass::UpdateContext {
            Self::prepare_draw(self);
        }
    }
}
