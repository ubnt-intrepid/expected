use crate::{context::Context, Disappoints};
use futures_core::{
    future::Future,
    task::{self, Poll},
};
use pin_project::pin_project;
use std::pin::Pin;

/// A future for the [`expected`] method.
///
/// [`expected`]: ./trait.FutureExpectedExt.html#method.expected
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

/// An extension trait for `Future`s that provides an adaptor for tracking
/// that all expectations are satisfied.
pub trait FutureExpectedExt: Future + Sized {
    /// See that all expectations are satisfied until the future is completed.
    ///
    /// # Example
    ///
    /// ```
    /// use expected::{expect, expect_eq};
    /// use expected::FutureExpectedExt as _;
    /// # use futures_test::future::FutureTestExt as _;
    ///
    /// # futures_executor::block_on(async {
    /// let name = "Alice";
    /// let country = "Wonderland";
    /// let age = 14;
    ///
    /// let fut = async {
    ///     expect_eq!(name, "Alice");
    ///     expect!(country.contains("land"));
    /// #   async {}.pending_once().await;
    ///     // ...
    ///     expect!(age >= 20);
    /// };
    ///
    /// let (_, disappoints) = fut.expected().await;
    /// # assert_eq!(disappoints.len(), 1);
    ///
    /// if !disappoints.is_empty() {
    ///     eprintln!("{}", disappoints);
    /// }
    /// # });
    fn expected(self) -> Expected<Self> {
        Expected {
            fut: self,
            ctx: Context::default(),
        }
    }
}

impl<F: Future> FutureExpectedExt for F {}
