/*!
An assertion utility focused on unit testing.
!*/

#![doc(html_root_url = "https://docs.rs/expected/0.0.1")]
//#![deny(missing_docs)]
#![forbid(clippy::todo, clippy::unimplemented)]
#![cfg_attr(test, deny(warnings))]

mod context;
mod disappoint;
mod macros;

cfg_if::cfg_if! {
    if #[cfg(feature = "futures")] {
        mod futures;
        pub use futures::{FutureExpectedExt, Expected};
    }
}

pub use crate::disappoint::{Disappoint, Disappoints};

use crate::context::Context;

/// Run the provided closure and checks to see if all expectation have been satisfied.
pub fn expected<F, R>(f: F) -> (R, Disappoints)
where
    F: FnOnce() -> R,
{
    let mut ctx = Context::default();
    let value = ctx.set(f);
    (value, ctx.take_disappoints())
}

#[doc(hidden)] // private API
#[inline(never)]
pub fn disappoint(payload: String, file: &'static str, line: u32, column: u32) {
    Context::with(|ctx| {
        ctx.add_disappoint(Disappoint::new(payload, file, line, column));
    })
    .unwrap_or_else(|| {
        eprintln!("warning: expect!() should be invoked inside of `expected`.");
    });
}
