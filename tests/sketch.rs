#![allow(clippy::assertions_on_constants, clippy::eq_op, clippy::identity_op)]

use expected::{expect, expect_eq, expected};
use maybe_unwind::maybe_unwind;
use std::sync::Once;

#[test]
fn no_disappoints() {
    let ((), disappoints) = expected(|| {
        expect!(1 + 1 == 2);
    });
    assert_eq!(disappoints.len(), 0);
}

#[test]
fn has_one_disappoint() {
    let ((), disappoints) = expected(|| {
        expect!(1 + 2 == 2);
    });
    assert_eq!(disappoints.len(), 1);
}

#[test]
fn has_more_disappoints() {
    let ((), disappoints) = expected(|| {
        expect!(1 + 0 == 2);
        expect!(1 + 1 == 2);
        expect!(1 - 1 == -1);
    });
    assert_eq!(disappoints.len(), 2);
}

#[test]
fn with_assertions() {
    static SET_HOOK: Once = Once::new();
    SET_HOOK.call_once(maybe_unwind::set_hook);

    let (res, disappoints) = expected(|| {
        maybe_unwind(|| {
            expect!(1 + 0 == 2);
            assert!(1 + 0 == 2);
            expect!(1 + 0 == 2);
        })
    });
    let _unwind = res.unwrap_err();
    assert_eq!(disappoints.len(), 1);
}

#[cfg(feature = "futures")]
#[test]
fn with_futures() {
    use expected::FutureExpectedExt as _;
    use futures_executor::block_on;
    use futures_test::future::FutureTestExt as _;

    let ((), disappoints) = block_on(
        async {
            expect!(1 + 0 == 2);
            expect!(1 + 1 == 2);
            async {}.pending_once().await;
            expect!(1 - 1 == -1);
        }
        .expected(),
    );
    assert_eq!(disappoints.len(), 2);
}

#[test]
#[ignore]
fn smoke_macros() {
    let _ = expected(|| {
        expect!(1 + 1 == 3);
        expect!(1 + 1 == 3, "1 + 1 must be equal to {}", 2);
        expect_eq!(1 + 1, 3);
        expect_eq!(1 + 1, 3, "1 + 1 must be equal to {}", 2);
    });
}
