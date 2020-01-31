use crate::{context::Context, Disappoints};
use futures_core::{
    future::Future,
    task::{self, Poll},
};
use pin_project::pin_project;
use std::pin::Pin;

#[pin_project]
#[derive(Debug)]
pub struct Expected<Fut> {
    #[pin]
    fut: Fut,
    ctx: Context,
}

impl<Fut> Future for Expected<Fut>
where
    Fut: Future,
{
    type Output = (Fut::Output, Disappoints);

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        let me = self.project();
        let fut = me.fut;
        let ctx = me.ctx;
        let res = match ctx.set(|| fut.poll(cx)) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(res) => res,
        };
        let disappoints = ctx.take_disappoints();
        Poll::Ready((res, disappoints))
    }
}

pub trait FutureExpectedExt: Future + Sized {
    fn expected(self) -> Expected<Self> {
        Expected {
            fut: self,
            ctx: Context::default(),
        }
    }
}

impl<F: Future> FutureExpectedExt for F {}
