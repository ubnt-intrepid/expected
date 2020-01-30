use scoped_tls::scoped_thread_local;
use std::cell::RefCell;
use std::fmt;

/// A set of disappoints occurred during an execution of `asseverate`.
#[derive(Debug)]
pub struct Disappoints(Vec<Disappoint>);

impl std::ops::Deref for Disappoints {
    type Target = [Disappoint];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl fmt::Display for Disappoints {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "one or more expectations have been disappointed:")?;
        for disappoint in &self.0 {
            writeln!(f, "{}", disappoint)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Disappoint {
    payload: String,
    file: &'static str,
    line: u32,
    column: u32,
}

impl fmt::Display for Disappoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "[{}:{}:{}] {}",
            self.file, self.line, self.column, self.payload,
        )
    }
}

/// Show an expectation that the specified conditional expression is true.
///
/// Unlike `assert!` in the standard library, this macro does not panic
/// when the condition is not satisfied. Instead, it will store the information
/// that the expectation is disappointed to the thread-local storage and
/// continue the subsequent line.
#[macro_export]
macro_rules! expect {
    ($cond:expr $(,)?) => {
        if !$cond {
            $crate::add_disappointment(
                concat!("expectation disappointed: ", stringify!($cond)).into(),
                file!(),
                line!(),
                column!(),
            );
        }
    };
}

#[doc(hidden)] // private API
pub fn add_disappointment(payload: String, file: &'static str, line: u32, column: u32) {
    if !DISAPPOINTS.is_set() {
        eprintln!("warning: expect!() should be invoked inside of `expected`.");
        return;
    }

    DISAPPOINTS.with(|disappoints| {
        if let Ok(mut disappoints) = disappoints.try_borrow_mut() {
            disappoints.push(Disappoint {
                payload,
                file,
                line,
                column,
            });
        } else {
        }
    });
}

scoped_thread_local! {
    static DISAPPOINTS: RefCell<Vec<Disappoint>>
}

/// Run the provided closure and checks to see if all expectation have been satisfied.
pub fn expected<F, R>(f: F) -> (R, Disappoints)
where
    F: FnOnce() -> R,
{
    let disappoints = RefCell::new(vec![]);
    let value = DISAPPOINTS.set(&disappoints, f);

    let disappoints = disappoints.into_inner();
    (value, Disappoints(disappoints))
}
