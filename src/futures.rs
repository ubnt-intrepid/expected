use crate::{Disappoint, Disappoints, DISAPPOINTS};
use futures_core::{
    future::Future,
    task::{self, Poll},
};
use pin_project::pin_project;
use std::{cell::RefCell, pin::Pin};

#[pin_project]
#[derive(Debug)]
pub struct Expected<Fut> {
    #[pin]
    fut: Fut,
    disappoints: Option<RefCell<Vec<Disappoint>>>,
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
        let res = match DISAPPOINTS.set(me.disappoints.as_ref().unwrap(), || fut.poll(cx)) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(res) => res,
        };
        let disappoints = me.disappoints.take().unwrap().into_inner();
        Poll::Ready((res, Disappoints(disappoints)))
    }
}

pub trait FutureExpectedExt: Future + Sized {
    fn expected(self) -> Expected<Self> {
        Expected {
            fut: self,
            disappoints: Some(RefCell::new(vec![])),
        }
    }
}

impl<F: Future> FutureExpectedExt for F {}
