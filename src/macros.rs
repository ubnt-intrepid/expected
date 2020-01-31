/// Declare an expectation that the specified conditional expression is true.
///
/// Unlike `assert!` in the standard library, this macro does not panic
/// when the condition is not satisfied. Instead, it will store the information
/// that the expectation is disappointed to the thread-local storage and
/// continue the subsequent line.
#[macro_export]
macro_rules! expect {
    ($cond:expr $(,)?) => {
        if !$cond {
            $crate::disappoint(
                Box::new(concat!("expectation disappointed: ", stringify!($cond))),
                file!(),
                line!(),
                column!(),
            );
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        if !$cond {
            $crate::disappoint(
                Box::new(format!($($arg)+)),
                file!(),
                line!(),
                column!(),
            );
        }
    };
}

/// Declare an expectation that the specified values are equal.
#[macro_export]
macro_rules! expect_eq {
    ($lhs:expr, $rhs:expr $(,)?) => {
        match (&$lhs, &$rhs) {
            (lhs, rhs) => {
                if !(*lhs == *rhs) {
                    $crate::disappoint(
                        Box::new(format!(
                            r#"expectation disappointed: `(left == right)`
  left: `{:?}`,
 right: `{:?}`"#,
                            &*lhs, &*rhs
                        )),
                        file!(),
                        line!(),
                        column!(),
                    );
                }
            }
        }
    };
    ($lhs:expr, $rhs:expr, $($arg:tt)+) => {
        match (&$lhs, &$rhs) {
            (lhs, rhs) => {
                if !(*lhs == *rhs) {
                    $crate::disappoint(
                        Box::new(format!(
                            r#"expectation disappointed: `(left == right)`
  left: `{:?}`,
 right: `{:?}`: {}"#,
                            &*lhs, &*rhs, format_args!($($arg)+)
                        )),
                        file!(),
                        line!(),
                        column!(),
                    );
                }
            }
        }
    };
}
