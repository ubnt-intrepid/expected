use crate::{Disappoint, Disappoints};
use std::{cell::Cell, ptr::NonNull};

#[derive(Debug, Default)]
pub(crate) struct Context {
    disappoints: Vec<Disappoint>,
}

thread_local! {
    static TLS_CTX: Cell<Option<NonNull<Context>>> = Cell::new(None);
}

struct SetOnDrop(Option<NonNull<Context>>);

impl Drop for SetOnDrop {
    fn drop(&mut self) {
        TLS_CTX.with(|tls| {
            tls.set(self.0.take());
        });
    }
}

impl Context {
    pub(crate) fn set<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let old_tls = TLS_CTX.with(|tls| tls.replace(Some(NonNull::from(self))));
        let _reset = SetOnDrop(old_tls);
        f()
    }

    pub(crate) fn with<F, R>(f: F) -> Option<R>
    where
        F: FnOnce(&mut Self) -> R,
    {
        let ctx_ptr = TLS_CTX.with(|ctx| ctx.replace(None));
        let _reset = SetOnDrop(ctx_ptr);
        match ctx_ptr {
            Some(mut ctx) => unsafe { Some(f(ctx.as_mut())) },
            None => {
                eprintln!("warning: expect!() should be invoked inside of `expected`.");
                None
            }
        }
    }

    pub(crate) fn add_disappoint(&mut self, disappoint: Disappoint) {
        self.disappoints.push(disappoint);
    }

    pub(crate) fn take_disappoints(&mut self) -> Disappoints {
        Disappoints(std::mem::replace(&mut self.disappoints, vec![]))
    }
}
